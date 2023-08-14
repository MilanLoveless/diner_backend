use diner_backend::configuration::get_configuration;
use diner_backend::connectors::{discord::DiscordApi, oauth::OauthClient, session::SessionStore};
use diner_backend::startup::run;
use diner_backend::telemetry::*;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Config
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Tracing Registry
    let subscriber = get_subscriber("diner_backend".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Discord Api
    let discord_api = DiscordApi::new(configuration.discord_api.url);
    // Discord Oauth
    let discord_oauth = OauthClient::new(&configuration.discord_oauth);
    // PG
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());
    // Session
    let session_store = SessionStore::new("something".to_string());
    // Server
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(
        listener,
        connection_pool,
        discord_oauth,
        discord_api,
        session_store,
    )?
    .await
}
