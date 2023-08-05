use actix_web::HttpResponse;

pub async fn get_oauth_url() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn oauth_redirect() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub async fn oauth_revoke() -> HttpResponse {
    HttpResponse::Ok().finish()
}
