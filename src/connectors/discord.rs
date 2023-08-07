pub struct DiscordUserResponse {
    username: String,
    avatar: String,
}

pub struct DiscordApi {
    url: String,
}

impl DiscordApi {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn get_user(token: String) -> DiscordUserResponse {
        return DiscordUserResponse {
            username: "@10xmilan".to_string(),
            avatar: "https://example.com/images/milan".to_string(),
        };
    }
}
