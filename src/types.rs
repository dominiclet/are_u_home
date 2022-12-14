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
   pub message: Message
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub message_id: i64,
    pub from: User,
    pub date: i32,
    pub chat: Chat,
    pub text: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub id: i64,
    pub r#type: String,
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
pub struct SendMessageReq{
    pub chat_id: i64,
    pub text: String
}
