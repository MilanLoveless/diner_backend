use super::super::connectors::oauth::OauthClient;
use actix_web::{http, web, HttpResponse, Scope};
use chrono::Utc;
use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize)]
struct OauthUrlResponse {
    url: String,
}

#[tracing::instrument(name = "requesting new oauth url", skip(oauth))]
async fn get_url(oauth: web::Data<OauthClient>) -> HttpResponse {
    let url_result = oauth.get_oauth_url();
    match url_result {
        Ok(url) => HttpResponse::Ok().json(OauthUrlResponse {
            url: url.to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[derive(serde::Deserialize)]
pub struct OauthRedirectData {
    pub code: String,
    pub state: String,
}

#[tracing::instrument(
    name = "Incoming redirect from Discord Oauth2 flow", skip(data, oauth),
    fields(
        code = %data.code,
        state = %data.state,
    )
)]
async fn redirect(
    data: web::Query<OauthRedirectData>,
    pool: web::Data<PgPool>,
    oauth: web::Data<OauthClient>,
) -> HttpResponse {
    let token_result = oauth
        .get_token(data.code.to_owned(), data.state.to_owned())
        .await;
    if let Ok(token) = token_result {
        return HttpResponse::Ok().finish();
    }
    HttpResponse::InternalServerError().finish()
}

async fn revoke() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn get_oauth2_scope() -> Scope {
    web::scope("/oauth2")
        .route("/url", web::get().to(get_url))
        .route("/redirect", web::delete().to(redirect))
        .route("/revoke", web::get().to(revoke))
}
// #[tracing::instrument(
//     name = "Saving new subscriber details in the database",
//     skip(form, pool)
// )]
// pub async fn insert_game(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
//     sqlx::query!(
//         r#"
// INSERT INTO games (id, name, description, link, created_at, updated_at)
// VALUES ($1, $2, $3, $4, $5, $6)
//     "#,
//         Uuid::new_v4(),
//         form.name,
//         form.description,
//         form.link,
//         Utc::now(),
//         Utc::now()
//     )
//     .execute(pool)
//     .await
//     .map_err(|e| {
//         tracing::error!("Failed to execute query: {:?}", e);
//         e
//         // Using the `?` operator to return early
//         // if the function failed, returning a sqlx::Error
//         // We will talk about error handling in depth later!
//     })?;
//     Ok(())
// }