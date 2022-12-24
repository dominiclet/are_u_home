use core::time;
use std::{sync::{Arc, Mutex}, collections::HashMap, thread::{self, sleep}, time::SystemTime};

use crate::{telegram_apis::TelegramClient, types::Update, constants::{UPDATE_INTERVAL, HELP_COMMAND}};

pub enum Command {
    Help,
    Start,
    Homed
}

struct GroupInfo {
    pub no_ppl: i32,
    pub accounted: Vec<String>,
    pub last_bumped: SystemTime,
}

pub fn start_bot(token: String) {
    let bot_token = Arc::new(token);
    let mut telegram_client = TelegramClient::new(Arc::clone(&bot_token).to_string());

    // Create hashmap to store active going home chat groups
    let hashmap: HashMap<i64, GroupInfo> = HashMap::new();
    let going_home_groups = Arc::new(Mutex::new(hashmap));

    let going_home_groups_clone = Arc::clone(&going_home_groups);
    thread::spawn(move || {
        let checker_telegram_client = TelegramClient::new(bot_token.to_string());

        loop {
            {
                let mut hashmap_addr = going_home_groups_clone.lock().unwrap();
                let mut bumped_chats: Vec<i64> = Vec::new();

                for (chat_id, group_info) in &*hashmap_addr {
                    let message_str: String;
                    if group_info.accounted.len() == 0 {
                        message_str = String::from("Are you home?\n\
                                                   Currently no one has updated that they are home...");
                    } else {
                        let unaccounted_no = group_info.no_ppl - group_info.accounted.len() as i32;
                        
                        message_str = format!("Are you home?\n\
                                            Currently, only {} has updated that they are home.\n\
                                            We still have {} people in this group that are unaccounted for.",
                                            group_info.accounted.join(", "),
                                            unaccounted_no);
                    }

                    let time_since_last_bump = match group_info.last_bumped.elapsed() {
                        Err(err) => {
                            error!("Error getting elapsed time since last bump: {:?}", err);
                            continue
                        },
                        Ok(elapsed) => elapsed
                    };

                    // If elapsed time is long enough, then send message
                    if time_since_last_bump >= UPDATE_INTERVAL {
                        let resp = checker_telegram_client.send_message(*chat_id, message_str);
                        match resp {
                            Err(_err) => error!("Encountered error sending are you home message."),
                            Ok(_resp) => {
                                bumped_chats.push(*chat_id);
                            }
                        }
                    }
                }

                // Update last bumped for all bumped chats
                for chat_id in bumped_chats {
                    let info = match hashmap_addr.get_mut(&chat_id) {
                        Some(info) => info,
                        None => {
                            error!("Encountered error updating last_bumped");
                            continue
                        }
                    };
                    info.last_bumped = SystemTime::now();
                }
            }
            sleep(time::Duration::from_secs(2));
        }
    });

    info!("Starting update loop");
    loop {
        let updates = match telegram_client.get_updates() {
            Ok(updates) => updates,
            Err(err) => {
                error!("Failed to get update: {:?}", err);
                continue
            }
        };

        info!("Retrieved updates: {:?}", updates);

        for update in updates {
            let command = match update.get_command() {
                Some(command) => command,
                None => continue
            };

            match command {
                Command::Help => help_handler(&update,&telegram_client),
                Command::Start => start_handler(&update, &going_home_groups, &telegram_client),
                Command::Homed => homed_handler(&update, &going_home_groups, &telegram_client)
            };
        }

        sleep(time::Duration::from_secs(2));
    }
}

// Handlers

fn help_handler(update: &Update, telegram_client: &TelegramClient) {
    let message = match &update.message {
        Some(message) => message,
        None => {
            info!("Incoming update is not a message, ignoring update.");
            return
        }
    };

    match telegram_client.send_message(message.chat.id,
                                       String::from(HELP_COMMAND)) {
        Err(_) => {
            error!("Help handler: Failed to send help message.");
        }
        Ok(_) => ()
    }
}

fn homed_handler(update: &Update, going_home_groups: &Arc<Mutex<HashMap<i64, GroupInfo>>>, telegram_client: &TelegramClient) {
    let message = match &update.message {
        Some(message) => message,
        None => {
            info!("Incoming update is not a message, ignoring update.");
            return
        }
    };
    let mut going_home_hashmap = going_home_groups.lock().unwrap();

    // Check if a going home session for this chat is active
    if !going_home_hashmap.contains_key(&message.chat.id) {
        let message_str = "This group does not have an active going home session.\
        To start one, use /start.";
        let resp = telegram_client.send_message(message.chat.id, String::from(message_str));
        match resp {
            Err(_) => error!("Homed handler: Failed to send message"),
            Ok(_) => ()
        }
        return
    }

    let user = &message.from;
    let group_info = going_home_hashmap.get_mut(&message.chat.id).unwrap();
    if !group_info.accounted.contains(&user.username) {
        group_info.accounted.push(user.username.clone());

        let resp = telegram_client.send_message(message.chat.id,
                                     format!("{} updated that they are home.", user.username));
        match resp {
            Err(_) => error!("Homed handler: Failed to send message"),
            Ok(_) => ()
        }
    }

    // If everyone is accounted for, remove entry from hashmap
    if group_info.accounted.len() as i32 == group_info.no_ppl {
        going_home_hashmap.remove(&message.chat.id);

        let resp = telegram_client.send_message(message.chat.id,
                                                format!("Congrats! Everyone is home. I will stop sending updates now."));
        match resp {
            Err(_) => error!("Homed handler: Failed to send message"),
            Ok(_) => ()
        }
    }
}

fn start_handler(update: &Update, going_home_groups: &Arc<Mutex<HashMap<i64, GroupInfo>>>, telegram_client: &TelegramClient) {
    let message = match &update.message {
        Some(message) => message,
        None => return
    };
    let mut going_home_hashmap = going_home_groups.lock().unwrap();

    // Check if going home session is already active
    if going_home_hashmap.contains_key(&message.chat.id) {
        match telegram_client.
            send_message(message.chat.id, 
                         String::from("A going home session is already active for this group.")) {
                Err(err) => error!("Start handler: Failed to send already active going home session message - {:?}", err),
                Ok(_resp) => ()
            }
        return
    }

    let no_ppl = match telegram_client.get_chat_member_count(message.chat.id) {
        Ok(no_ppl) => no_ppl - 1, // Minus one to exclude bot
        Err(_err) => {
            error!("Start handler: Failed to get chat member count.");
            return
        }
    };

    going_home_hashmap.insert(message.chat.id,
                             GroupInfo{
                                 no_ppl,
                                 accounted: Vec::new(),
                                 last_bumped: SystemTime::now()
                             });

    match telegram_client.send_message(message.chat.id,
                                 format!("Started going home session, expecting {} people to update that they're home.", no_ppl)) {
        Err(err) => error!("Start handler: Failed to send start going home session message - {:?}", err),
        Ok(_resp) => ()
    }

    info!("Started going home session");
}

