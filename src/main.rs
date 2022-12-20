use core::time;
use std::{env, thread::{sleep, self}, collections::HashMap, sync::{Arc, Mutex}};

use telegram_apis::TelegramClient;
use types::{GroupInfo, Update};

mod telegram_apis;
mod types;
mod constants;

#[macro_use] extern crate log;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let bot_token = match env::var("BOT_TOKEN") {
        Ok(token) => Arc::new(token),
        Err(error) => panic!("Error retrieving BOT_TOKEN environment variable: {:?}", error)
    };
    let mut telegram_client = TelegramClient::new(Arc::clone(&bot_token).to_string());

    // Create hashmap to store active going home chat groups
    let hashmap: HashMap<i64, GroupInfo> = HashMap::new();
    let going_home_groups = Arc::new(Mutex::new(hashmap));

    let going_home_groups_clone = Arc::clone(&going_home_groups);
    thread::spawn(move || {
        let checker_telegram_client = TelegramClient::new(bot_token.to_string());

        loop {
            let hashmap_addr = going_home_groups_clone.lock().unwrap();

            for (chat_id, group_info) in &*hashmap_addr {
                let resp = checker_telegram_client.send_message(*chat_id, String::from("Are you home?"));
                match resp {
                    Err(_err) => continue,
                    Ok(_resp) => continue
                }
            }

        }
    });

    info!("Starting update loop");
    loop {
        let updates = telegram_client.get_updates()?;

        println!("{:?}", updates);

        for update in updates {
            let command = match update.get_command() {
                Some(command) => command,
                None => continue
            };

            let text = match command {
                types::Command::Help => String::from(crate::constants::HELP_COMMAND),
                types::Command::Start => {
                    let mut hashmap_addr = going_home_groups.lock().unwrap();
                    start_handler(&update, &mut *hashmap_addr);

                    String::from("Started going home session...")
                }
            };
            telegram_client.send_message(update.message.chat.id, 
                                         text)?;
        }

        sleep(time::Duration::from_secs(2));
    }
}


fn start_handler(update: &Update, going_home_groups: &mut HashMap<i64, GroupInfo>) {
    going_home_groups.insert(update.message.chat.id,
                             GroupInfo{
                                 no_ppl: 1,
                                 unaccounted_ppl: vec![String::from("abcd")],
                                 last_bumped: 1
                             });
    info!("Started going home session");
}
