//! This example shows how to set a custom bot HTTP client for sending HTTP requests.
//!
//! Usually you don't need to use a custom client, because [`telers`] provides default client,
//! but if you want to use a custom client, you can do it by using [`Bot::with_client`] method and use it in handlers.
//!
//! You can use any client, which implements [`Session`] trait and use it in handlers:
//! ```ignore
//! async fn handler(bot: Bot<impl Session>) -> HandlerResult {
//!    // ...
//! }
//! ```
//! You the same can use another client and use it directly:
//! ```ignore
//! async fn handler(bot: Bot<SomeClientDirectly>) -> HandlerResult {
//!     // ...
//! }
//! ```
//!
//! You can run this example by setting `BOT_TOKEN` and optional `RUST_LOG` environment variable and running:
//! ```bash
//! RUST_LOG={log_level} BOT_TOKEN={your_bot_token} cargo run --package bot_http_client
//! ```
//!
//! [`Bot::with_client`]: telers::Bot#method.with_client

use async_trait::async_trait;
use log::{error, info};
use std::borrow::Cow;
use telers::{
    client::{session::ClientResponse, telegram, Session},
    enums::UpdateType,
    event::{telegram::HandlerResult, EventReturn, ToServiceProvider as _},
    methods::{CopyMessage, TelegramMethod},
    types::Message,
    Bot, Dispatcher, Router,
};

#[derive(Clone)]
struct CustomClient {
    api: Cow<'static, telegram::APIServer>,
}

impl Default for CustomClient {
    fn default() -> Self {
        Self {
            api: Cow::Borrowed(&telegram::PRODUCTION),
        }
    }
}

#[async_trait]
impl Session for CustomClient {
    fn api(&self) -> &telegram::APIServer {
        &self.api
    }

    async fn send_request<Client, T>(
        &self,
        _bot: &Bot<Client>,
        _method: &T,
        _timeout: Option<f32>,
    ) -> Result<ClientResponse, anyhow::Error>
    where
        Client: Session,
        T: TelegramMethod + Send + Sync,
        T::Method: Send + Sync,
    {
        unimplemented!(
            "Send request is not implemented for custom client. \
            You can use default client or implement it for your custom client."
        )
    }
}

async fn echo_handler(bot: Bot<impl Session>, message: Message) -> HandlerResult {
    bot.send(
        &CopyMessage::new(message.chat.id, message.chat.id, message.message_id),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    pretty_env_logger::init();

    let Ok(bot_token) = std::env::var("BOT_TOKEN") else {
        panic!("BOT_TOKEN env variable is not set!");
    };

    let bot = Bot::with_client(bot_token, CustomClient::default());

    let mut router = Router::new("main");
    router.message.register(echo_handler);

    let dispatcher = Dispatcher::builder()
        .main_router(router)
        .bot(bot)
        .allowed_update(UpdateType::Message)
        .build();

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => info!("Bot stopped"),
        Err(err) => error!("Bot stopped with error: {err}"),
    }
}
