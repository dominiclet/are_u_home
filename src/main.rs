use teloxide::prelude::*;

mod handlers;
mod constants;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting bot...");

    let bot = Bot::from_env();

    handlers::Command::repl(bot, handlers::command_handler).await;
}
