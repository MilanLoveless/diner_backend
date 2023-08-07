pub mod discord;
pub mod oauth;
pub mod session;

pub struct Connectors {
    pub discord: discord::DiscordApi,
    pub oauth: oauth::OauthClient,
}
