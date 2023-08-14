use super::super::super::{domain::*, models::game};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(
    name = "Adding a new game", skip(form, pool),
    fields(
        game_name = %form.name,
        game_description = %form.description,
        game_link = %form.link
    )
)]
pub async fn create(form: web::Form<GameFormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let game_data = match GameData::try_from(form.0) {
        Ok(g) => g,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match game::insert(&pool, &game_data).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn get(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let id = Uuid::parse_str(request.match_info().get("id").unwrap()).expect("oh no!");
    if let Ok(game) = game::get(&pool, id).await {
        return HttpResponse::Ok().json(game);
    }
    HttpResponse::InternalServerError().finish()
}

pub async fn update(
    request: HttpRequest,
    form: web::Form<GameFormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let game_data = match GameData::try_from(form.0) {
        Ok(g) => g,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    let id: Uuid = Uuid::parse_str(request.match_info().get("id").unwrap()).expect("oh no!");
    match game::update(&pool, &id, &game_data).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn delete(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let id: Uuid = Uuid::parse_str(request.match_info().get("id").unwrap()).expect("oh no!");
    match game::delete(&pool, id).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
