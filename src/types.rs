use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetUpdatesResp {
    pub ok: bool,
    pub result: UpdateList
}

pub type UpdateList = Vec<Update>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
   pub update_id: i32,
   #[serde(default)]
   pub message: Option<Message>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub date: i32,
    pub chat: Chat,
    #[serde(default)]
    pub text: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
    #[serde(default)]
    pub active_usernames: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: String,
    pub username: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMessageReq {
    pub chat_id: i64,
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatReq {
    pub chat_id: i64
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatResp {
    pub ok: bool,
    pub result: Chat
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetChatMemberCountResp {
    pub ok: bool,
    pub result: i32
}

impl Update {
    pub fn get_command(&self) -> Option<Command> {
        let message = match &self.message {
            None => return None,
            Some(message) => message
        };
        let text = match &message.text {
            None => return None,
            Some(text) => text
        };
        let mut split_str = text.split_whitespace();
        let command = match split_str.next() {
            Some(command) => String::from(command),
            None => return None
        };
        // Check if command begins with '/'
        let first_char = match command.chars().next() {
            Some(first_char) => first_char,
            None => return None
        };
        if first_char != '/' {
            return None;
        }

        info!("Retrieved command {}", command);

        match &command[1..].to_lowercase()[..] {
            "help" => Some(Command::Help),
            "start" => Some(Command::Start),
            "homed" => Some(Command::Homed),
            _ => None
        }
    }
}

pub enum Command {
    Help,
    Start,
    Homed
}

pub struct GroupInfo {
    pub no_ppl: i32,
    pub accounted: Vec<String>,
    pub last_bumped: SystemTime,
}
