use super::configuration::{DatabaseSettings, Settings};
use super::connectors::oauth::OauthClient;
use super::connectors::{discord::DiscordApi, session::SessionStore};
// use super::scopes::api::api_sesion_middleware;
use super::scopes::{api, auth, health_check, oauth};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // Discord Api
        let discord_api = DiscordApi::new(configuration.discord_api);
        // Discord Oauth
        let discord_oauth = OauthClient::new(&configuration.discord_oauth);
        // PG
        let connection_pool = get_connection_pool(&configuration.database);
        // Session
        let session_store = SessionStore {};
        // Server
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();

        // Server
        let server = run(
            listener,
            connection_pool,
            discord_oauth,
            discord_api,
            session_store,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // A more expressive name that makes it clear that
    // this function only returns when the application is stopped.
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

fn run(
    listener: TcpListener,
    connection_pool: PgPool,
    discord_oauth: OauthClient,
    discord_api: DiscordApi,
    session_store: SessionStore,
) -> Result<Server, std::io::Error> {
    // PG
    let pool = web::Data::new(connection_pool);
    let d_oauth = web::Data::new(discord_oauth);
    let d_api = web::Data::new(discord_api);
    let s_store = web::Data::new(session_store);
    let server = HttpServer::new(move || {
        // App
        App::new()
            .wrap(TracingLogger::default())
            .service(api::get_api_scope())
            .service(auth::get_auth_scope())
            .service(oauth::get_oauth2_scope())
            .route("/health_check", web::get().to(health_check::health_check))
            .app_data(pool.clone())
            .app_data(d_oauth.clone())
            .app_data(d_api.clone())
            .app_data(s_store.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
