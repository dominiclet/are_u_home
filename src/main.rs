use std::env;

mod telegram_apis;
mod types;
mod constants;
mod are_u_home_bot;

#[macro_use] extern crate log;

fn main() {
    pretty_env_logger::init();

    let bot_token = match env::var("BOT_TOKEN") {
        Ok(token) => token,
        Err(error) => panic!("Error retrieving BOT_TOKEN environment variable: {:?}", error)
    };

    are_u_home_bot::start_bot(bot_token);
}
