use super::super::connectors::{discord::DiscordApi, oauth::OauthClient, session::SessionStore};
use super::super::domain::UserFormData;
use super::super::models::user;
use actix_web::{web, HttpResponse, Scope};
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
    name = "Incoming redirect from Discord Oauth2 flow", skip(data, oauth, pool, discord_api, session_store),
    fields(
        code = %data.code,
        state = %data.state,
    )
)]
async fn redirect(
    data: web::Query<OauthRedirectData>,
    pool: web::Data<PgPool>,
    oauth: web::Data<OauthClient>,
    discord_api: web::Data<DiscordApi>,
    session_store: web::Data<SessionStore>,
) -> HttpResponse {
    if let Ok(token) = oauth
        .get_token(data.code.to_owned(), data.state.to_owned())
        .await
    {
        if let Ok(discord_user) = discord_api.get_user(token).await {
            let mut user_id: Option<Uuid> = None;
            if let Ok(user_record) = user::get_by_username(&pool, &discord_user.username).await {
                user_id = Some(user_record.id);
            } else {
                let user = UserFormData {
                    username: discord_user.username,
                    avatar: discord_user.avatar,
                    banner: discord_user.banner,
                    global_name: discord_user.global_name,
                };
                if let Ok(id) = user::insert(&pool, &user).await {
                    user_id = Some(id);
                }
            }
            if let Some(id) = user_id {
                if let Ok(session) = session_store.create(id) {
                    return HttpResponse::Ok().json(session);
                }
            }
        }
    }
    HttpResponse::InternalServerError().finish()
}

async fn revoke() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn get_oauth2_scope() -> Scope {
    web::scope("/oauth2")
        .route("/url", web::get().to(get_url))
        .route("/redirect", web::get().to(redirect))
        .route("/revoke", web::get().to(revoke))
}
