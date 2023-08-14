use super::models::game;
use super::types::games::GameFormData;
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;
use unicode_segmentation::UnicodeSegmentation;
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
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().finish();
    }
    match game::insert(&pool, &form.name, &form.description, &form.link).await {
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
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().finish();
    }
    let id: Uuid = Uuid::parse_str(request.match_info().get("id").unwrap()).expect("oh no!");
    match game::update(&pool, &id, &form).await {
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

/// Returns `true` if the input satisfies all our validation constraints
/// on subscriber names, `false` otherwise.
pub fn is_valid_name(s: &str) -> bool {
    // `.trim()` returns a view over the input `s` without trailing
    // whitespace-like characters.
    // `.is_empty` checks if the view contains any character.
    let is_empty_or_whitespace = s.trim().is_empty();
    // A grapheme is defined by the Unicode standard as a "user-perceived"
    // character: `å` is a single grapheme, but it is composed of two characters
    // (`a` and `̊`).
    //
    // `graphemes` returns an iterator over the graphemes in the input `s`.
    // `true` specifies that we want to use the extended grapheme definition set,
    // the recommended one.
    let is_too_long = s.graphemes(true).count() > 256;
    // Iterate over all characters in the input `s` to check if any of them matches
    // one of the characters in the forbidden array.
    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
    // Return `false` if any of our conditions have been violated
    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}
