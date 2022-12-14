use serde::{Deserialize, Serialize};

use crate::are_u_home_bot::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MethodResp<T> {
    pub ok: bool,
    pub result: Option<T>,
    pub description: Option<String>
}

pub type UpdateList = Vec<Update>;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Update {
   pub update_id: i32,
   #[serde(default)]
   pub message: Option<Message>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub date: i32,
    pub chat: Chat,
    #[serde(default)]
    pub text: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
    #[serde(default)]
    pub active_usernames: Vec<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>
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

pub type GetUpdatesResp = MethodResp<UpdateList>;
pub type GetChatResp = MethodResp<Chat>;
pub type GetChatMemberCountResp = MethodResp<i32>;

impl<T> MethodResp<T> {
    pub fn get_result(&self) -> Option<&T> {
        if !self.ok {
            error!("Unable to get response result - {}", self.description.as_ref().unwrap());
            return None;
        }
        let result = match &self.result {
            Some(result) => result,
            None => return None
        };

        Some(result)
    }
}

impl Update {
    pub fn get_command(&self) -> Option<Command> {
        let message = match &self.message {
            None => {
                info!("Unable to get command: Update is not a message.");
                return None
            },
            Some(message) => message 
        };
        let text = match &message.text {
            None => {
                info!("Unable to get command: Message does not have text.");
                return None
            },
            Some(text) => text
        };

        // Check if text begins with '/'
        let first_char = match text.chars().next() {
            Some(first_char) => first_char,
            None => return None
        };
        if first_char != '/' {
            return None;
        }
        info!("Retrieved command {}", text);

        let truncated_command = &text[1..].to_lowercase();

        if truncated_command.starts_with("help") {
            Some(Command::Help)
        } else if truncated_command.starts_with("start") {
            Some(Command::Start)
        } else if truncated_command.starts_with("homed") {
            Some(Command::Homed)
        } else {
            warn!("Provided command '{}' not recognised.", truncated_command);
            None
        }
    }
}
