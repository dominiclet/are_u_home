use crate::types::{GetUpdatesResp, UpdateList};

const TELEGRAM_DOMAIN: &str = "https://api.telegram.org";

pub struct TelegramClient {
    http_client: reqwest::Client,
    token: String
}

impl TelegramClient {
    pub fn new(token: String) -> TelegramClient {
        TelegramClient {
            http_client: reqwest::Client::new(),
            token 
        }
    }

    fn construct_endpoint(&self, method: &str) -> String {
        format!("{}/bot{}/{}", TELEGRAM_DOMAIN, self.token, method)
    }

    pub async fn get_updates(&self) -> Result<UpdateList, reqwest::Error> {
        let resp = reqwest::get(self.construct_endpoint("getUpdates")).await?;

        let get_updates_resp = resp.json::<GetUpdatesResp>().await?;

        Ok(get_updates_resp.result)
    }

}

