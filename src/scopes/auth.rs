use actix_web::web;
use actix_web::{HttpResponse, Scope};

async fn get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn renew() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Might not be necessary with JWT
async fn revoke() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn get_auth_scope() -> Scope {
    web::scope("/auth")
        .route("/session", web::post().to(get))
        .route("/session", web::put().to(renew))
        .route("/session", web::delete().to(revoke))
}
