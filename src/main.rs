use core::time;
use std::{env, thread::sleep};

use telegram_apis::TelegramClient;

mod telegram_apis;
mod types;

#[macro_use] extern crate log;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let bot_token = match env::var("BOT_TOKEN") {
        Ok(token) => token,
        Err(error) => panic!("Error retrieving BOT_TOKEN environment variable: {:?}", error)
    };
    let mut telegram_client = TelegramClient::new(bot_token);

    info!("Starting update loop");
    loop {
        let updates = telegram_client.get_updates().await?;

        println!("{:?}", updates);

        for update in updates {
            telegram_client.send_message(update.message.chat.id, 
                                         update.message.text).await?;
        }

        sleep(time::Duration::from_secs(2));
    }
}

