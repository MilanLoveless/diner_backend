use oauth2::basic::BasicTokenType;
use oauth2::{EmptyExtraTokenFields, StandardTokenResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct DiscordUserRecord {
    pub username: String,
    pub avatar: String,
}

#[derive(Clone)]
pub struct DiscordApi {
    url: String,
}

impl DiscordApi {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn get_user(
        &self,
        token: StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
    ) -> Result<DiscordUserRecord, String> {
        Ok(DiscordUserRecord {
            username: "@10xmilan".to_string(),
            avatar: "https://example.com/images/milan".to_string(),
        })
    }
}
