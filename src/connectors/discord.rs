use crate::configuration::DiscordApiSettings;
use oauth2::basic::BasicTokenType;
use oauth2::{EmptyExtraTokenFields, StandardTokenResponse, TokenResponse};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DiscordUserRecord {
    pub username: String,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub global_name: String,
}

#[derive(Clone)]
pub struct DiscordApi {
    uri: String,
    user_path: String,
    client: Client,
}

impl DiscordApi {
    pub fn new(config: DiscordApiSettings) -> Self {
        Self {
            uri: config.uri,
            user_path: config.user_path,
            client: Client::new(),
        }
    }

    pub async fn get_user(
        &self,
        token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<DiscordUserRecord, String> {
        let response = self
            .client
            .get(format!("{}{}", self.uri, self.user_path))
            .header("Accept", "application/json")
            .bearer_auth(token.access_token().secret())
            .send()
            .await
            .expect("on no!");
        if response.status() == reqwest::StatusCode::OK {
            let user_record = response.json::<DiscordUserRecord>().await.expect("oh no!");
            return Ok(user_record);
        }
        Err("oh no!".to_string())
    }
}
