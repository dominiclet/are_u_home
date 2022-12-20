use std::collections::HashMap;

use crate::types::{GetUpdatesResp, UpdateList, SendMessageReq};

const TELEGRAM_DOMAIN: &str = "https://api.telegram.org";

pub struct TelegramClient {
    http_client: reqwest::blocking::Client,
    token: String,
    update_offset: Option<i32>
}

impl TelegramClient {
    pub fn new(token: String) -> TelegramClient {
        TelegramClient {
            http_client: reqwest::blocking::Client::new(),
            token,
            update_offset: None
        }
    }

    fn construct_endpoint(&self, method: &str, params: Option<HashMap<String, i32>>) -> String {
        let endpoint_string: String;
        match params {
            Some(params) => {
                let mut parameters: Vec<String> = Vec::new();
                for (key, value) in params {
                    parameters.push(format!("{}={}", key, value));
                }
                let param_string = parameters.join("&");
                endpoint_string = format!("{}/bot{}/{}?{}", TELEGRAM_DOMAIN, self.token, method, param_string);
            }

            None => {
                endpoint_string = format!("{}/bot{}/{}", TELEGRAM_DOMAIN, self.token, method);
            }
        }

        endpoint_string
    }

    pub fn get_updates(&mut self) -> Result<UpdateList, Box<dyn std::error::Error>> {
        let endpoint: String;

        match self.update_offset {
            Some(update_offset) => {
                let param_map = HashMap::from([
                    (String::from("offset"), update_offset)
                ]);
                endpoint = self.construct_endpoint("getUpdates", Some(param_map));
            }

            None => {
                endpoint = self.construct_endpoint("getUpdates", None);
            }
        }

        let resp = self.http_client.get(endpoint)
            .send()?;

        let get_updates_resp = resp.json::<GetUpdatesResp>()?;

        // Handle not OK response
        if get_updates_resp.ok == false {
            return Err("getUpdates did not return a successful response.".into());
        }

        // Update offset so that previous updates are no longer returned
        let mut max_update_id: i32 = 0;
        for update in &get_updates_resp.result {
            if update.update_id > max_update_id {
                max_update_id = update.update_id;
            }
        }
        self.update_offset = Some(max_update_id + 1);

        Ok(get_updates_resp.result)
    }

    pub fn send_message(&self, chat_id: i64, text: String) -> Result<(), Box<dyn std::error::Error>> {
        let message_req = SendMessageReq {
            chat_id,
            text
        };

        self.http_client.post(self.construct_endpoint("sendMessage", None))
            .json(&message_req).send()?;

        Ok(())
    }
}

