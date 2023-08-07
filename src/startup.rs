use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};

use super::connectors::discord::DiscordApi;
use super::scopes::api::api_sesion_middleware;

// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use super::connectors::oauth::*;
use super::scopes::{api, auth, health_check, oauth};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    pool: PgPool,
    oauth: OauthClient,
    discord_api: DiscordApi,
) -> Result<Server, std::io::Error> {
    // SSL
    // let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // ssl_builder
    //     .set_private_key_file("secrets/ssl/key.pem", SslFiletype::PEM)
    //     .unwrap();
    // ssl_builder
    //     .set_certificate_chain_file("secrets/ssl/cert.pem")
    //     .unwrap();

    // PG
    let pool = web::Data::new(pool);

    // Server
    let server = HttpServer::new(move || {
        // App
        App::new()
            .wrap(TracingLogger::default())
            .service(api::get_api_scope().wrap(api_sesion_middleware()))
            .service(auth::get_auth_scope())
            .service(oauth::get_oauth2_scope())
            .route("/health_check", web::get().to(health_check::health_check))
            .app_data(pool.clone())
            .app_data(oauth.clone())
    })
    // .bind_openssl("127.0.0.1:8080", ssl_builder)?
    .listen(listener)?
    .run();
    Ok(server)
}
