use diner_backend::configuration::get_configuration;
use diner_backend::connectors::oauth::*;
use diner_backend::startup::run;
use diner_backend::telemetry::*;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Config
    let configuration = get_configuration().expect("Failed to read configuration.");
    // Tracing Registry
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Discord
    let discord_oauth = OauthClient::new(&configuration.discord_oauth);
    // PG
    let pool = PgPool::connect_with(configuration.database.with_db())
        .await
        .expect("Failed to connect to Postgres.");
    // Server
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;
    run(listener, pool, discord_oauth)?.await
}
