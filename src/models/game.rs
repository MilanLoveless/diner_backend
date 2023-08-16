use crate::types::{GameData, GameRecord};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Fetching existing game from the database", skip(pool))]
pub async fn get(pool: &PgPool, id: Uuid) -> Result<GameRecord, sqlx::Error> {
    let record = sqlx::query!(
        r#"
SELECT id, name, description, link FROM games
WHERE id = $1
    "#,
        id,
    )
    .fetch_one(pool)
    .await
    .map(|r| GameRecord {
        id: r.id,
        name: r.name,
        description: r.description,
        link: r.link,
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

#[tracing::instrument(name = "Saving new game to the database", skip(data, pool))]
pub async fn insert(pool: &PgPool, data: &GameData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO games (id, name, description, link, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5, $6)
    "#,
        Uuid::new_v4(),
        data.name.as_ref(),
        data.description.as_ref(),
        data.link.as_ref(),
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

#[tracing::instrument(name = "Updating existing game in the database", skip(data, pool))]
pub async fn update(pool: &PgPool, id: &Uuid, data: &GameData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
UPDATE games SET name = $1, description = $2, link = $3, updated_at = $4
WHERE id = $5
    "#,
        data.name.as_ref(),
        data.description.as_ref(),
        data.link.as_ref(),
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

#[tracing::instrument(name = "Deleting existing game from the database", skip(pool))]
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
