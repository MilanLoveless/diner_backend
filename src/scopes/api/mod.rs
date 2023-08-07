use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::Scope;
use actix_web::{
    cookie::{self, Key},
    web,
};

mod games;
mod middleware;
mod models;
mod types;
mod users;

pub fn api_sesion_middleware() -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
        .cookie_secure(false)
        // customize session and cookie expiration
        .session_lifecycle(
            PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
        )
        .build()
}

pub fn get_api_scope() -> Scope {
    web::scope("/api")
        .route("/users", web::post().to(users::create))
        .route("/users", web::get().to(users::get))
        .route("/users", web::delete().to(users::update))
        .route("/users", web::delete().to(users::delete))
        .route("/games", web::post().to(games::create))
        .route("/games/{id}", web::get().to(games::get))
        .route("/games/{id}", web::put().to(games::update))
        .route("/games/{id}", web::delete().to(games::delete))
}
