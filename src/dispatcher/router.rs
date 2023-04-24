use super::{
    event::{
        bases::{EventReturn, PropagateEventResult},
        service::{ServiceProvider, ToServiceProvider},
        simple::{
            handler::Result as SimpleHandlerResult,
            observer::{Observer as SimpleObserver, ObserverInner as SimpleObserverInner},
        },
        telegram::observer::{
            Observer as TelegramObserver, ObserverInner as TelegramObserverInner,
            Request as TelegramObserverRequest,
        },
    },
    middlewares::outer::{
        Middleware as OuterMiddleware, Middlewares as OuterMiddlewares,
        UserContext as UserContextMiddleware,
    },
};

use crate::{
    client::Bot,
    context::Context,
    enums::{
        observer_name::{Simple as SimpleObserverName, Telegram as TelegramObserverName},
        update_type::UpdateType,
    },
    error::AppErrorKind,
    types::Update,
};

use async_recursion::async_recursion;
use log;
use std::{
    collections::HashSet,
    fmt::{self, Debug, Formatter},
    iter::once,
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct Request<Client> {
    pub bot: Arc<Bot<Client>>,
    pub update: Arc<Update>,
    pub context: Arc<Context>,
}

impl<Client> PartialEq for Request<Client> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.bot, &other.bot)
            && Arc::ptr_eq(&self.update, &other.update)
            && Arc::ptr_eq(&self.context, &other.context)
    }
}

impl<Client> Request<Client> {
    #[must_use]
    pub fn new<B, U, C>(bot: B, update: U, context: C) -> Self
    where
        B: Into<Arc<Bot<Client>>>,
        U: Into<Arc<Update>>,
        C: Into<Arc<Context>>,
    {
        Self {
            bot: bot.into(),
            update: update.into(),
            context: context.into(),
        }
    }
}

impl<Client> From<Request<Client>> for TelegramObserverRequest<Client> {
    fn from(req: Request<Client>) -> Self {
        Self::new(req.bot, req.update, req.context)
    }
}

#[derive(Debug)]
pub struct Response<Client> {
    pub request: Request<Client>,
    pub propagate_result: PropagateEventResult<Client>,
}

impl<Client> Response<Client> {
    #[must_use]
    pub fn new(request: Request<Client>, propagate_result: PropagateEventResult<Client>) -> Self {
        Self {
            request,
            propagate_result,
        }
    }
}

/// Router can route update, and it nested update types like messages, callback query, polls and all other event types.
/// Event handlers can be registered in observer by following methods:
/// - By observer method - [`router.{event_type}.register(handler).filter(...).filters(...)`]
/// - By observer method - [`router.{event_type}.on(handler).filter(...).filters(...)`]
pub struct Router<Client> {
    /// Can be used for logging and debugging
    pub router_name: &'static str,
    /// Sub routers of this router. \
    /// If update is processed by this router, it will be propagated to sub routers.
    pub sub_routers: Vec<Router<Client>>,

    pub message: TelegramObserver<Client>,
    pub edited_message: TelegramObserver<Client>,
    pub channel_post: TelegramObserver<Client>,
    pub edited_channel_post: TelegramObserver<Client>,
    pub inline_query: TelegramObserver<Client>,
    pub chosen_inline_result: TelegramObserver<Client>,
    pub callback_query: TelegramObserver<Client>,
    pub shipping_query: TelegramObserver<Client>,
    pub pre_checkout_query: TelegramObserver<Client>,
    pub poll: TelegramObserver<Client>,
    pub poll_answer: TelegramObserver<Client>,
    pub my_chat_member: TelegramObserver<Client>,
    pub chat_member: TelegramObserver<Client>,
    pub chat_join_request: TelegramObserver<Client>,

    /// This special event observer is used to handle all telegram events.
    /// It's called for router and its sub routers and before other telegram observers.
    /// This observer is useful for register important middlewares (often libraries) like `FSMContext` and `UserContext`,
    /// that set up context for other.
    ///
    /// The order of calls looks simplistically like this:
    /// Dispatcher -> Router -> Update observer -> Sub routers -> Update observer
    ///            -> Router -> Other telegram observers -> Sub routers -> Other telegram observers
    pub update: TelegramObserver<Client>,

    pub startup: SimpleObserver,
    pub shutdown: SimpleObserver,
}

impl<Client> Router<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    #[rustfmt::skip]
    pub fn new(router_name: &'static str) -> Self {
        Self {
            router_name,
            sub_routers: vec![],
            message: TelegramObserver::new(TelegramObserverName::Message.as_str()),
            edited_message: TelegramObserver::new(TelegramObserverName::EditedMessage.as_str()),
            channel_post: TelegramObserver::new(TelegramObserverName::ChannelPost.as_str()),
            edited_channel_post: TelegramObserver::new(TelegramObserverName::EditedChannelPost.as_str()),
            inline_query: TelegramObserver::new(TelegramObserverName::InlineQuery.as_str()),
            chosen_inline_result: TelegramObserver::new(TelegramObserverName::ChosenInlineResult.as_str()),
            callback_query: TelegramObserver::new(TelegramObserverName::CallbackQuery.as_str()),
            shipping_query: TelegramObserver::new(TelegramObserverName::ShippingQuery.as_str()),
            pre_checkout_query: TelegramObserver::new(TelegramObserverName::PreCheckoutQuery.as_str()),
            poll: TelegramObserver::new(TelegramObserverName::Poll.as_str()),
            poll_answer: TelegramObserver::new(TelegramObserverName::PollAnswer.as_str()),
            my_chat_member: TelegramObserver::new(TelegramObserverName::MyChatMember.as_str()),
            chat_member: TelegramObserver::new(TelegramObserverName::ChatMember.as_str()),
            chat_join_request: TelegramObserver::new(TelegramObserverName::ChatJoinRequest.as_str()),
            update: TelegramObserver::new(TelegramObserverName::Update.as_str()),
            startup: SimpleObserver::new(SimpleObserverName::Startup.as_str()),
            shutdown: SimpleObserver::new(SimpleObserverName::Shutdown.as_str()),
        }
    }

    /// Register inner middlewares in router and sub routers of the router
    fn register_inner_middlewares(&self, router: &mut Router<Client>) {
        // Register middlewares of current router observers to sub router observers at first positions
        macro_rules! register_middlewares {
            ($observer:ident) => {
                let mut index = 0;
                for middleware in &self.$observer.inner_middlewares.middlewares {
                    router.$observer.inner_middlewares.register_at_position(index, Arc::clone(middleware));
                    index += 1;
                }
            };
            ($observer:ident, $($observers:ident),+) => {
                register_middlewares!($observer);
                register_middlewares!($($observers),+);
            };
        }

        // Call register middlewares macro for all telegram event observers
        register_middlewares!(
            message,
            edited_message,
            channel_post,
            edited_channel_post,
            inline_query,
            chosen_inline_result,
            callback_query,
            shipping_query,
            pre_checkout_query,
            poll,
            poll_answer,
            my_chat_member,
            chat_member,
            chat_join_request,
            update
        );

        router.sub_routers.iter_mut().for_each(|sub_router| {
            self.register_inner_middlewares(sub_router);
        });
    }

    /// Include a sub router
    ///
    /// This method will register all middlewares of router,
    /// which registered before call this method, in sub router
    pub fn include_router(&mut self, mut router: Router<Client>) -> &mut Self {
        self.register_inner_middlewares(&mut router);

        self.sub_routers.push(router);
        self
    }

    /// Alias to [`Router::include_router`] method
    pub fn include(&mut self, router: Router<Client>) -> &mut Self {
        self.include_router(router)
    }
}

impl<Client> Router<Client> {
    #[must_use]
    pub fn telegram_observers(&self) -> Vec<&TelegramObserver<Client>> {
        vec![
            &self.message,
            &self.edited_message,
            &self.channel_post,
            &self.edited_channel_post,
            &self.inline_query,
            &self.chosen_inline_result,
            &self.callback_query,
            &self.shipping_query,
            &self.pre_checkout_query,
            &self.poll,
            &self.poll_answer,
            &self.my_chat_member,
            &self.chat_member,
            &self.chat_join_request,
            &self.update,
        ]
    }

    #[must_use]
    pub fn event_observers(&self) -> Vec<&SimpleObserver> {
        vec![&self.startup, &self.shutdown]
    }

    /// Resolve registered update types
    ///
    /// Is useful for getting updates only for registered update types
    /// # Warning
    /// This method doesn't preserve registration order of update types
    /// # Returns
    /// Registered update types
    #[must_use]
    pub fn resolve_used_update_types(&self) -> Vec<UpdateType> {
        let mut used_update_types = HashSet::new();

        self.sub_routers.iter().for_each(|router| {
            used_update_types.extend(router.resolve_used_update_types());
        });

        used_update_types.extend(
            self.telegram_observers()
                .iter()
                .filter(|observer| !observer.handlers.is_empty())
                .map(|observer| {
                    <&str as TryInto<UpdateType>>::try_into(observer.event_name).expect(
                        "Can't convert event name to UpdateType. This is a bug. Please, report it.",
                    )
                }),
        );

        used_update_types.into_iter().collect()
    }

    /// Resolve registered update types with skip update types
    ///
    /// Is useful for getting updates only for registered update types with skip some updates types
    /// # Arguments
    /// * `skip_updates` - Skip update types
    /// # Warning
    /// This method doesn't preserve registration order of update types
    /// # Returns
    /// Registered update types
    #[must_use]
    pub fn resolve_used_update_types_with_skip(
        &self,
        skip_updates: impl IntoIterator<Item = UpdateType>,
    ) -> Vec<UpdateType> {
        let skip_updates = skip_updates.into_iter().collect::<HashSet<_>>();

        self.resolve_used_update_types()
            .into_iter()
            .filter(|update_type| !skip_updates.contains(update_type))
            .collect()
    }
}

impl<Client> Debug for Router<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.router_name)
            .finish()
    }
}

impl<Client> Default for Router<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    fn default() -> Self {
        Self::new("default")
    }
}

impl<Client> AsRef<Router<Client>> for Router<Client> {
    #[must_use]
    fn as_ref(&self) -> &Self {
        self
    }
}

pub struct OuterMiddlewaresConfig<Client> {
    message: OuterMiddlewares<Client>,
    edited_message: OuterMiddlewares<Client>,
    channel_post: OuterMiddlewares<Client>,
    edited_channel_post: OuterMiddlewares<Client>,
    inline_query: OuterMiddlewares<Client>,
    chosen_inline_result: OuterMiddlewares<Client>,
    callback_query: OuterMiddlewares<Client>,
    shipping_query: OuterMiddlewares<Client>,
    pre_checkout_query: OuterMiddlewares<Client>,
    poll: OuterMiddlewares<Client>,
    poll_answer: OuterMiddlewares<Client>,
    my_chat_member: OuterMiddlewares<Client>,
    chat_member: OuterMiddlewares<Client>,
    chat_join_request: OuterMiddlewares<Client>,
    update: OuterMiddlewares<Client>,
}

impl<Client> OuterMiddlewaresConfig<Client> {
    pub fn clear(&mut self) {
        self.message.clear();
        self.edited_message.clear();
        self.channel_post.clear();
        self.edited_channel_post.clear();
        self.inline_query.clear();
        self.chosen_inline_result.clear();
        self.callback_query.clear();
        self.shipping_query.clear();
        self.pre_checkout_query.clear();
        self.poll.clear();
        self.poll_answer.clear();
        self.my_chat_member.clear();
        self.chat_member.clear();
        self.chat_join_request.clear();
        self.update.clear();
    }

    #[must_use]
    pub fn builder() -> OuterMiddlewaresConfigBuilder<Client> {
        OuterMiddlewaresConfigBuilder::default()
    }
}

impl<Client> Clone for OuterMiddlewaresConfig<Client> {
    fn clone(&self) -> Self {
        Self {
            message: self.message.clone(),
            edited_message: self.edited_message.clone(),
            channel_post: self.channel_post.clone(),
            edited_channel_post: self.edited_channel_post.clone(),
            inline_query: self.inline_query.clone(),
            chosen_inline_result: self.chosen_inline_result.clone(),
            callback_query: self.callback_query.clone(),
            shipping_query: self.shipping_query.clone(),
            pre_checkout_query: self.pre_checkout_query.clone(),
            poll: self.poll.clone(),
            poll_answer: self.poll_answer.clone(),
            my_chat_member: self.my_chat_member.clone(),
            chat_member: self.chat_member.clone(),
            chat_join_request: self.chat_join_request.clone(),
            update: self.update.clone(),
        }
    }
}

impl<Client> Default for OuterMiddlewaresConfig<Client>
where
    Client: Send + Sync + 'static,
{
    #[must_use]
    fn default() -> Self {
        Self::builder()
            .update(UserContextMiddleware::default())
            .build()
    }
}

pub struct OuterMiddlewaresConfigBuilder<Client> {
    message: OuterMiddlewares<Client>,
    edited_message: OuterMiddlewares<Client>,
    channel_post: OuterMiddlewares<Client>,
    edited_channel_post: OuterMiddlewares<Client>,
    inline_query: OuterMiddlewares<Client>,
    chosen_inline_result: OuterMiddlewares<Client>,
    callback_query: OuterMiddlewares<Client>,
    shipping_query: OuterMiddlewares<Client>,
    pre_checkout_query: OuterMiddlewares<Client>,
    poll: OuterMiddlewares<Client>,
    poll_answer: OuterMiddlewares<Client>,
    my_chat_member: OuterMiddlewares<Client>,
    chat_member: OuterMiddlewares<Client>,
    chat_join_request: OuterMiddlewares<Client>,
    update: OuterMiddlewares<Client>,
}

impl<Client> OuterMiddlewaresConfigBuilder<Client> {
    #[must_use]
    pub fn message<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            message: self
                .message
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn edited_message<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            edited_message: self
                .edited_message
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn channel_post<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            channel_post: self
                .channel_post
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn edited_channel_post<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            edited_channel_post: self
                .edited_channel_post
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn inline_query<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            inline_query: self
                .inline_query
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn chosen_inline_result<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            chosen_inline_result: self
                .chosen_inline_result
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn callback_query<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            callback_query: self
                .callback_query
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn shipping_query<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            shipping_query: self
                .shipping_query
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn pre_checkout_query<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            pre_checkout_query: self
                .pre_checkout_query
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn poll<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            poll: self
                .poll
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn poll_answer<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            poll_answer: self
                .poll_answer
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn my_chat_member<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            my_chat_member: self
                .my_chat_member
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn chat_member<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            chat_member: self
                .chat_member
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn chat_join_request<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            chat_join_request: self
                .chat_join_request
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn update<T>(self, val: T) -> Self
    where
        T: OuterMiddleware<Client> + 'static,
    {
        Self {
            update: self
                .update
                .into_iter()
                .chain(Some(Arc::new(val) as _))
                .collect(),
            ..self
        }
    }

    #[must_use]
    pub fn build(self) -> OuterMiddlewaresConfig<Client> {
        OuterMiddlewaresConfig {
            message: self.message,
            edited_message: self.edited_message,
            channel_post: self.channel_post,
            edited_channel_post: self.edited_channel_post,
            inline_query: self.inline_query,
            chosen_inline_result: self.chosen_inline_result,
            callback_query: self.callback_query,
            shipping_query: self.shipping_query,
            pre_checkout_query: self.pre_checkout_query,
            poll: self.poll,
            poll_answer: self.poll_answer,
            my_chat_member: self.my_chat_member,
            chat_member: self.chat_member,
            chat_join_request: self.chat_join_request,
            update: self.update,
        }
    }
}

impl<Client> Default for OuterMiddlewaresConfigBuilder<Client> {
    #[must_use]
    fn default() -> Self {
        Self {
            message: OuterMiddlewares::default(),
            edited_message: OuterMiddlewares::default(),
            channel_post: OuterMiddlewares::default(),
            edited_channel_post: OuterMiddlewares::default(),
            inline_query: OuterMiddlewares::default(),
            chosen_inline_result: OuterMiddlewares::default(),
            callback_query: OuterMiddlewares::default(),
            shipping_query: OuterMiddlewares::default(),
            pre_checkout_query: OuterMiddlewares::default(),
            poll: OuterMiddlewares::default(),
            poll_answer: OuterMiddlewares::default(),
            my_chat_member: OuterMiddlewares::default(),
            chat_member: OuterMiddlewares::default(),
            chat_join_request: OuterMiddlewares::default(),
            update: OuterMiddlewares::default(),
        }
    }
}

pub struct Config<Client> {
    outer_middlewares: OuterMiddlewaresConfig<Client>,
}

impl<Client> Clone for Config<Client> {
    fn clone(&self) -> Self {
        Self {
            outer_middlewares: self.outer_middlewares.clone(),
        }
    }
}

impl<Client> Config<Client> {
    #[must_use]
    pub fn new(outer_middlewares: OuterMiddlewaresConfig<Client>) -> Self {
        Self { outer_middlewares }
    }
}

impl<Client> Default for Config<Client>
where
    Client: Send + Sync + 'static,
{
    fn default() -> Self {
        Self {
            outer_middlewares: OuterMiddlewaresConfig::default(),
        }
    }
}

impl<Client> ToServiceProvider for Router<Client>
where
    Client: Send + Sync + 'static,
{
    type Config = Config<Client>;
    type ServiceProvider = RouterInner<Client>;
    type InitError = ();

    fn to_service_provider(
        mut self,
        mut config: Self::Config,
    ) -> Result<Self::ServiceProvider, Self::InitError> {
        let router_name = self.router_name;

        // Register outer middlewares to router observers
        macro_rules! register_middlewares {
            ($observer:ident) => {
                let mut index = 0;
                for middleware in &config.outer_middlewares.$observer {
                    self.$observer.outer_middlewares.register_at_position(index, Arc::clone(middleware));
                    index += 1;
                }
            };
            ($observer:ident, $($observers:ident),+) => {
                register_middlewares!($observer);
                register_middlewares!($($observers),+);
            };
        }

        // Call register middlewares macro for all telegram event observers
        register_middlewares!(
            message,
            edited_message,
            channel_post,
            edited_channel_post,
            inline_query,
            chosen_inline_result,
            callback_query,
            shipping_query,
            pre_checkout_query,
            poll,
            poll_answer,
            my_chat_member,
            chat_member,
            chat_join_request,
            update
        );

        // Clear outer middlewares from config, because they're useless for sub routers
        config.outer_middlewares.clear();

        let sub_routers = self
            .sub_routers
            .into_iter()
            .map(|router| router.to_service_provider(config.clone()))
            .collect::<Result<_, _>>()?;
        let message = self.message.to_service_provider_default()?;
        let edited_message = self.edited_message.to_service_provider_default()?;
        let channel_post = self.channel_post.to_service_provider_default()?;
        let edited_channel_post = self.edited_channel_post.to_service_provider_default()?;
        let inline_query = self.inline_query.to_service_provider_default()?;
        let chosen_inline_result = self.chosen_inline_result.to_service_provider_default()?;
        let callback_query = self.callback_query.to_service_provider_default()?;
        let shipping_query = self.shipping_query.to_service_provider_default()?;
        let pre_checkout_query = self.pre_checkout_query.to_service_provider_default()?;
        let poll = self.poll.to_service_provider_default()?;
        let poll_answer = self.poll_answer.to_service_provider_default()?;
        let my_chat_member = self.my_chat_member.to_service_provider_default()?;
        let chat_member = self.chat_member.to_service_provider_default()?;
        let chat_join_request = self.chat_join_request.to_service_provider_default()?;
        let update = self.update.to_service_provider_default()?;
        let startup = self.startup.to_service_provider_default()?;
        let shutdown = self.shutdown.to_service_provider_default()?;

        Ok(RouterInner {
            router_name,
            sub_routers,
            message,
            edited_message,
            channel_post,
            edited_channel_post,
            inline_query,
            chosen_inline_result,
            callback_query,
            shipping_query,
            pre_checkout_query,
            poll,
            poll_answer,
            my_chat_member,
            chat_member,
            chat_join_request,
            update,
            startup,
            shutdown,
        })
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct RouterInner<Client> {
    router_name: &'static str,
    sub_routers: Vec<RouterInner<Client>>,

    message: TelegramObserverInner<Client>,
    edited_message: TelegramObserverInner<Client>,
    channel_post: TelegramObserverInner<Client>,
    edited_channel_post: TelegramObserverInner<Client>,
    inline_query: TelegramObserverInner<Client>,
    chosen_inline_result: TelegramObserverInner<Client>,
    callback_query: TelegramObserverInner<Client>,
    shipping_query: TelegramObserverInner<Client>,
    pre_checkout_query: TelegramObserverInner<Client>,
    poll: TelegramObserverInner<Client>,
    poll_answer: TelegramObserverInner<Client>,
    my_chat_member: TelegramObserverInner<Client>,
    chat_member: TelegramObserverInner<Client>,
    chat_join_request: TelegramObserverInner<Client>,
    update: TelegramObserverInner<Client>,

    startup: SimpleObserverInner,
    shutdown: SimpleObserverInner,
}

impl<Client> ServiceProvider for RouterInner<Client> {}

impl<Client> RouterInner<Client>
where
    Client: Send + Sync + Clone + 'static,
{
    /// Propagate event to routers
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    /// # Warning
    /// This function doesn't compare the update type with the request update type.
    /// Assumed that [`UpdateType`] is correct because it is derived from [`Update`].
    /// This behaviour allows you not to check recursively [`UpdateType`] and can be used for testing purposes,
    /// but it's not recommended to use it in production.
    #[async_recursion]
    #[must_use]
    pub async fn propagate_event(
        &self,
        update_type: UpdateType,
        request: Request<Client>,
    ) -> Result<Response<Client>, AppErrorKind> {
        self.propagate_update_event(request.clone()).await?;

        let observer = self.telegram_observer_by_update_type(update_type);

        let mut request = request;
        for middleware in &observer.outer_middlewares {
            let (updated_request, event_return) = middleware.call(request.clone()).await?;

            match event_return {
                // Update request because the middleware could have changed it
                EventReturn::Finish => request = updated_request,
                // If middleware returns skip, then we should skip this middleware and its changes
                EventReturn::Skip => continue,
                // If middleware returns cancel, then we should cancel propagation
                EventReturn::Cancel => {
                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    })
                }
            }
        }

        self.propagate_event_by_observer(observer, update_type, request)
            .await
    }

    /// Propagate update event to routers
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    #[async_recursion]
    #[must_use]
    async fn propagate_update_event(
        &self,
        request: Request<Client>,
    ) -> Result<Response<Client>, AppErrorKind> {
        let mut request = request;
        for middleware in &self.update.outer_middlewares {
            let (updated_request, event_return) = middleware.call(request.clone()).await?;

            match event_return {
                // Update request because the middleware could have changed it
                EventReturn::Finish => request = updated_request,
                // If middleware returns skip, then we should skip this middleware and its changes
                EventReturn::Skip => continue,
                // If middleware returns cancel, then we should cancel propagation
                EventReturn::Cancel => {
                    return Ok(Response {
                        request,
                        propagate_result: PropagateEventResult::Rejected,
                    })
                }
            }
        }

        self.propagate_update_event_by_observer(request).await
    }

    /// Propagate event to routers by observer
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    async fn propagate_update_event_by_observer(
        &self,
        request: Request<Client>,
    ) -> Result<Response<Client>, AppErrorKind> {
        let observer_request = request.clone().into();
        let observer_response = self.update.trigger(observer_request).await?;

        match observer_response.propagate_result {
            // Propagate event to sub routers
            PropagateEventResult::Unhandled => {}
            // Return a response if the event handled
            PropagateEventResult::Handled(response) => {
                return Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Handled(response),
                });
            }
            // Return a response if the event rejected
            // Router don't know about rejected event by observer
            PropagateEventResult::Rejected => {
                return Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                });
            }
        };

        // Propagate event to sub routers' observer
        for router in &self.sub_routers {
            let router_response = router.propagate_update_event(request.clone()).await?;
            match router_response.propagate_result {
                // Propagate event to next sub router's observer if the event unhandled by the sub router's observer
                PropagateEventResult::Unhandled => continue,
                PropagateEventResult::Handled(_) | PropagateEventResult::Rejected => {
                    return Ok(router_response)
                }
            };
        }

        // Return a response if the event unhandled by observer
        Ok(Response {
            request,
            propagate_result: PropagateEventResult::Unhandled,
        })
    }

    /// Propagate event to routers by observer
    /// # Errors
    /// - If any outer middleware returns error
    /// - If any inner middleware returns error
    /// - If any handler returns error. Probably it's error to extract args to the handler
    async fn propagate_event_by_observer(
        &self,
        observer: &TelegramObserverInner<Client>,
        update_type: UpdateType,
        request: Request<Client>,
    ) -> Result<Response<Client>, AppErrorKind> {
        let observer_request = request.clone().into();
        let observer_response = observer.trigger(observer_request).await?;

        match observer_response.propagate_result {
            // Propagate event to sub routers
            PropagateEventResult::Unhandled => {}
            // Return a response if the event handled
            PropagateEventResult::Handled(response) => {
                return Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Handled(response),
                });
            }
            // Return a response if the event rejected
            // Router don't know about rejected event by observer
            PropagateEventResult::Rejected => {
                return Ok(Response {
                    request,
                    propagate_result: PropagateEventResult::Unhandled,
                });
            }
        };

        // Propagate event to sub routers' observer
        for router in &self.sub_routers {
            let router_response = router.propagate_event(update_type, request.clone()).await?;
            match router_response.propagate_result {
                // Propagate event to next sub router's observer if the event unhandled by the sub router's observer
                PropagateEventResult::Unhandled => continue,
                PropagateEventResult::Handled(_) | PropagateEventResult::Rejected => {
                    return Ok(router_response)
                }
            };
        }

        // Return a response if the event unhandled by observer
        Ok(Response {
            request,
            propagate_result: PropagateEventResult::Unhandled,
        })
    }
}

impl<Client> RouterInner<Client> {
    #[must_use]
    pub const fn telegram_observer_by_update_type(
        &self,
        update_type: UpdateType,
    ) -> &TelegramObserverInner<Client> {
        match update_type {
            UpdateType::Message => &self.message,
            UpdateType::EditedMessage => &self.edited_message,
            UpdateType::ChannelPost => &self.channel_post,
            UpdateType::EditedChannelPost => &self.edited_channel_post,
            UpdateType::InlineQuery => &self.inline_query,
            UpdateType::ChosenInlineResult => &self.chosen_inline_result,
            UpdateType::CallbackQuery => &self.callback_query,
            UpdateType::ShippingQuery => &self.shipping_query,
            UpdateType::PreCheckoutQuery => &self.pre_checkout_query,
            UpdateType::Poll => &self.poll,
            UpdateType::PollAnswer => &self.poll_answer,
            UpdateType::MyChatMember => &self.my_chat_member,
            UpdateType::ChatMember => &self.chat_member,
            UpdateType::ChatJoinRequest => &self.chat_join_request,
        }
    }

    /// Emit startup events
    /// # Errors
    /// If any startup observer returns error
    pub async fn emit_startup(&self) -> SimpleHandlerResult {
        log::debug!("{self:?}: Emit startup");

        for startup in
            once(&self.startup).chain(self.sub_routers.iter().map(|router| &router.startup))
        {
            startup.trigger(()).await?;
        }
        Ok(())
    }

    /// Emit shutdown events
    /// # Errors
    /// If any shutdown observer returns error
    pub async fn emit_shutdown(&self) -> SimpleHandlerResult {
        log::debug!("{self:?}: Emit shutdown");

        for shutdown in
            once(&self.shutdown).chain(self.sub_routers.iter().map(|router| &router.shutdown))
        {
            shutdown.trigger(()).await?;
        }
        Ok(())
    }
}

impl<Client> Debug for RouterInner<Client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Router")
            .field("router_name", &self.router_name)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        client::Reqwest,
        dispatcher::{
            event::{telegram::HandlerResult as TelegramHandlerResult, EventReturn},
            middlewares::inner::Next,
        },
        filters::Command,
    };

    use tokio;

    #[test]
    fn test_router_include() {
        let mut router = Router::<Reqwest>::new("main");

        let inner_middleware = |request, next: Next<_>| next(request);
        let outer_middleware = |request| async move { Ok((request, EventReturn::default())) };

        router.message.inner_middlewares.register(inner_middleware);
        router.message.outer_middlewares.register(outer_middleware);

        router
            .include({
                let mut router = Router::new("sub1");
                router
                    .include(Router::new("sub1.1"))
                    .include(Router::new("sub1.2"));
                router
            })
            .include({
                let mut router = Router::new("sub2");
                router
                    .include(Router::new("sub2.1"))
                    .include(Router::new("sub2.2"));
                router
            })
            .include({
                let mut router = Router::new("sub3");
                router
                    .include(Router::new("sub3.1"))
                    .include(Router::new("sub3.2"));
                router
            });

        assert_eq!(router.sub_routers.len(), 3);
        assert_eq!(router.router_name, "main");

        let message_observer_name = UpdateType::Message.as_str();

        router.sub_routers.into_iter().for_each(|router| {
            assert_eq!(router.sub_routers.len(), 2);

            router
                .telegram_observers()
                .into_iter()
                .for_each(|observer| {
                    if observer.event_name == message_observer_name {
                        assert_eq!(observer.inner_middlewares.middlewares.len(), 1);
                    } else {
                        assert_eq!(observer.inner_middlewares.middlewares.len(), 0);
                    }
                    // Router outer middlewares don't clone to children routers
                    assert_eq!(observer.outer_middlewares.middlewares.len(), 0);
                });

            router.sub_routers.into_iter().for_each(|router| {
                assert_eq!(router.sub_routers.len(), 0);

                router
                    .telegram_observers()
                    .into_iter()
                    .for_each(|observer| {
                        if observer.event_name == message_observer_name {
                            assert_eq!(observer.inner_middlewares.middlewares.len(), 1);
                        } else {
                            assert_eq!(observer.inner_middlewares.middlewares.len(), 0);
                        }
                        // Router outer middlewares don't clone to children routers
                        assert_eq!(observer.outer_middlewares.middlewares.len(), 0);
                    });
            });
        });
    }

    #[rustfmt::skip]
    #[test]
    fn test_router_observers_register() {
        async fn telegram_handler() -> TelegramHandlerResult {
            Ok(EventReturn::Finish)
        }

        async fn simple_handler() -> SimpleHandlerResult {
            Ok(())
        }

        let mut router = Router::<Reqwest>::new("main");
        // Telegram event observers
        router.message.register(telegram_handler);
        router.edited_message.register(telegram_handler);
        router.channel_post.register(telegram_handler);
        router.edited_channel_post.register(telegram_handler);
        router.inline_query.register(telegram_handler);
        router.chosen_inline_result.register(telegram_handler);
        router.callback_query.register(telegram_handler);
        router.shipping_query.register(telegram_handler);
        router.pre_checkout_query.register(telegram_handler);
        router.poll.register(telegram_handler);
        router.poll_answer.register(telegram_handler);
        router.my_chat_member.register(telegram_handler);
        router.chat_member.register(telegram_handler);
        router.chat_join_request.register(telegram_handler);
        router.update.register(telegram_handler);
        // Event observers
        router.startup.register(simple_handler, ());
        router.shutdown.register(simple_handler, ());

        // Check telegram event observers
        router
            .telegram_observers()
            .into_iter()
            .for_each(|observer| {
                assert_eq!(observer.handlers.len(), 1);

                observer.handlers.iter().for_each(|handler| {
                    assert!(handler.filters.is_empty());
                });
            });

        // Check event observers
        router.event_observers().into_iter().for_each(|observer| {
            assert_eq!(observer.handlers.len(), 1);
        });

        let inner_middleware = |request, next: Next<_>| next(request);
        let outer_middleware = |request| async move { Ok((request, EventReturn::Finish)) };

        router.message.inner_middlewares.register(inner_middleware);
        router.message.outer_middlewares.register(outer_middleware);

        assert_eq!(router.message.inner_middlewares.middlewares.len(), 1);
        assert_eq!(router.message.outer_middlewares.middlewares.len(), 1);
    }

    #[tokio::test]
    async fn test_router_propagate_event() {
        let bot = Bot::<Reqwest>::default();
        let context = Context::new();
        let update = Update::default();

        let mut router = Router::new("test1");
        router
            .update
            .outer_middlewares
            .register(|request: Request<Reqwest>| async move {
                request.context.insert("test", Box::new("test"));

                Ok((request, EventReturn::Finish))
            });
        router.message.register(|context: Arc<Context>| async move {
            assert_eq!(
                context.get("test").unwrap().downcast_ref::<&str>().unwrap(),
                &"test"
            );

            Ok(EventReturn::Finish)
        });

        let router_service = router.to_service_provider_default().unwrap();

        let request = Request::new(bot, update, context);
        let response = router_service
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Event should be handled, because there is a message handler registered
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }

        let response = router_service
            .propagate_event(UpdateType::CallbackQuery, request.clone())
            .await
            .unwrap();

        // Event shouldn't be handled, because there is no callback query handler registered
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("test2");
        router.message.filter(Command::default());
        router
            .message
            .register(|| async { Ok(EventReturn::Finish) });

        let router_service = router.to_service_provider_default().unwrap();

        let response = router_service
            .propagate_event(UpdateType::Message, request.clone())
            .await
            .unwrap();

        // Event shouldn't be handled, because there is a message handler registered, but it has filter, which doesn't pass
        match response.propagate_result {
            PropagateEventResult::Unhandled => {}
            _ => panic!("Unexpected result"),
        }

        let mut router = Router::new("test3");
        router
            .callback_query
            .register(|| async { Ok(EventReturn::Skip) });
        router
            .callback_query
            .register(|| async { Ok(EventReturn::Finish) });

        let router_service = router.to_service_provider_default().unwrap();

        let response = router_service
            .propagate_event(UpdateType::CallbackQuery, request)
            .await
            .unwrap();

        // Callback query event should be handled, because there is a callback query handler registered
        // and it returns `EventReturn::Skip`, so next handler should be called
        match response.propagate_result {
            PropagateEventResult::Handled(response) => match response.handler_result {
                Ok(EventReturn::Finish) => {}
                _ => panic!("Unexpected result"),
            },
            _ => panic!("Unexpected result"),
        }
    }

    #[test]
    fn test_resolve_used_update_types() {
        let mut router = Router::<Reqwest>::new("test");

        router
            .message
            .register(|| async { Ok(EventReturn::Finish) });
        router
            .edited_message
            .register(|| async { Ok(EventReturn::Finish) });

        let update_types = router.resolve_used_update_types();

        assert_eq!(update_types.len(), 2);
        assert!(update_types.contains(&UpdateType::Message));
        assert!(update_types.contains(&UpdateType::EditedMessage));

        let mut router2 = Router::<Reqwest>::new("test2");

        router2
            .message
            .register(|| async { Ok(EventReturn::Finish) });
        router2
            .channel_post
            .register(|| async { Ok(EventReturn::Finish) });

        assert_eq!(router2.resolve_used_update_types().len(), 2);

        router.include(router2);

        let update_types = router.resolve_used_update_types();

        assert_eq!(update_types.len(), 3);
        assert!(update_types.contains(&UpdateType::Message));
        assert!(update_types.contains(&UpdateType::EditedMessage));
        assert!(update_types.contains(&UpdateType::ChannelPost));

        let update_types = router.resolve_used_update_types_with_skip([UpdateType::Message]);

        assert_eq!(update_types.len(), 2);
        assert!(update_types.contains(&UpdateType::EditedMessage));
        assert!(update_types.contains(&UpdateType::ChannelPost));
    }
}
