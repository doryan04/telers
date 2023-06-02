<div align="center">

<h1><code>telers</code></h1>

<h3>
An asynchronous framework for Telegram Bot API written in Rust
</h3>

</div>

</p>

<b>Telers make it easy to create Telegram bots</b> in Rust.

Make sure you have a basic understanding of the [Telegram Bot API](https://core.telegram.org/bots/api) before you start, because **all types and methods in telers have the same fields and types as in Telegram Bot API**.

## Highlights
 - **Asynchronous**. Telers is built on top of [Tokio](https://tokio.rs/), a powerful asynchronous runtime for Rust.
 - **Easy to use**. Telers provides a simple and intuitive API to create Telegram bots.
 - **Based on** [aiogram](https://github.com/aiogram/aiogram). Telers is inspired by [aiogram](https://github.com/aiogram/aiogram), a Python framework for Telegram Bot API. Telers tries to provide the same API as aiogram, so if you know aiogram, you can easily start using this framework.
 - **Middlewares**, **Filters** and **Handlers**. Telers provides a powerful system of middlewares, filters and handlers. You can use middlewares to modify incoming/outgoing updates (logging, database connections, etc.), filters to filter incoming updates and handlers to handle incoming updates.
 - **Powerful extractors**. Telers provides a simple system of extractors. You can use extractors to extract data from incoming updates and context (middlewares, filters, etc.), and pass it to your handlers directly.
 - **Multiple bots**. Telers allows you to create multiple bots in one application without any problems.

## Examples
 - [Echo bot](examples/echo_bot). This example shows how to create a simple echo bot.
 - [Finite state machine](examples/finite_state_machine). This example shows how to create a simple finite state machine and use it in your bot.
 - [Custom http client](examples/custom_http_client). This example shows how to set a custom client for sending http requests.
 - [Custom filter](examples/custom_filter). This example shows how to create a custom filter.
 - [Custom middleware](examples/custom_middleware). This example shows how to create a custom middleware.

You may consider checking out [this directory](examples) for more examples.

## License
This project is licensed under either of the following licenses, at your option:
 - Apache License, Version 2.0
 - MIT License
