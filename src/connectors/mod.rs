use self::{discord::DiscordApi, oauth::OauthClient};

pub mod discord;
pub mod oauth;

pub struct Connectors {
    pub discord: DiscordApi,
    pub oauth: OauthClient,
}
