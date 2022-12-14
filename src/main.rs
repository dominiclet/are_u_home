use std::env;

use telegram_apis::TelegramClient;

mod telegram_apis;
mod types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot_token = match env::var("BOT_TOKEN") {
        Ok(token) => token,
        Err(error) => panic!("Error retrieving BOT_TOKEN environment variable: {:?}", error)
    };
    let telegram_client = TelegramClient::new(bot_token);

    let updates = telegram_client.get_updates().await?;

    println!("{:?}", updates);
    Ok(())
}

