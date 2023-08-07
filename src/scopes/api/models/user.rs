use super::super::types::{UserFormData, UserRecord};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Fetching existing user from the database", skip(pool))]
pub async fn get(pool: &PgPool, id: Uuid) -> Result<UserRecord, sqlx::Error> {
    let record = sqlx::query!(
        r#"
SELECT id, discord_id, avatar FROM users
WHERE id = $1
    "#,
        id,
    )
    .fetch_one(pool)
    .await
    .map(|r| UserRecord {
        id: r.id,
        username: r.discord_id,
        avatar: r.avatar,
    })
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(record)
}

#[tracing::instrument(name = "Fetching existing user from the database", skip(pool))]
pub async fn get_by_discord_id(
    pool: &PgPool,
    discord_id: &String,
) -> Result<UserRecord, sqlx::Error> {
    let record = sqlx::query!(
        r#"
SELECT id, discord_id, avatar FROM users
WHERE discord_id = $1
    "#,
        discord_id,
    )
    .fetch_one(pool)
    .await
    .map(|r| UserRecord {
        id: r.id,
        username: r.discord_id,
        avatar: r.avatar,
    })
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(record)
}

#[tracing::instrument(name = "Saving new user to the database", skip(form, pool))]
pub async fn insert(pool: &PgPool, form: &UserFormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO users (id, discord_id, avatar, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5)
    "#,
        Uuid::new_v4(),
        form.username,
        form.avatar,
        Utc::now(),
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}

#[tracing::instrument(name = "Updating existing user in the database", skip(form, pool))]
pub async fn update(pool: &PgPool, id: &Uuid, form: &UserFormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
UPDATE users SET discord_id = $1, avatar = $2, updated_at = $3
WHERE id = $4
    "#,
        form.username,
        form.avatar,
        Utc::now(),
        id,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}

#[tracing::instrument(name = "Deleting existing user from the database", skip(pool))]
pub async fn delete(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
DELETE FROM games
WHERE id = $1
    "#,
        id,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
        // Using the `?` operator to return early
        // if the function failed, returning a sqlx::Error
        // We will talk about error handling in depth later!
    })?;
    Ok(())
}
