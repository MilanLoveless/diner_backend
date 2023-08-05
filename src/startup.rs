use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
// use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use super::connectors::oauth::*;
use super::routes::health_check;
use super::routes::oauth::{get_oauth_url, oauth_redirect, oauth_revoke};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

pub fn run(
    listener: TcpListener,
    pool: PgPool,
    oauth: OauthClient,
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
        App::new()
            .wrap(TracingLogger::default())
            .route("/oauth/url", web::get().to(get_oauth_url))
            .route("/oauth/token", web::delete().to(oauth_revoke))
            .route("/oauth/redirect", web::get().to(oauth_redirect))
            .route("/health_check", web::get().to(health_check))
            .app_data(pool.clone())
            .app_data(oauth.clone())
    })
    // .bind_openssl("127.0.0.1:8080", ssl_builder)?
    .listen(listener)?
    .run();
    Ok(server)
}
