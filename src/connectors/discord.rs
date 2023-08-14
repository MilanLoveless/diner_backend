use oauth2::basic::BasicTokenType;
use oauth2::{EmptyExtraTokenFields, StandardTokenResponse, TokenResponse};
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
    url: String,
}

impl DiscordApi {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn get_user(
        &self,
        token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<DiscordUserRecord, String> {
        let response = reqwest::Client::new()
            .get("https://discordapp.com/api/users/@me")
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
