use sqlx::{Row, SqlitePool};

use crate::error::BackendError;

pub async fn setup(pool: &SqlitePool) -> Result<(), BackendError> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS images (
            id           TEXT    PRIMARY KEY,
            encrypted    BLOB    NOT NULL,
            nonce        BLOB    NOT NULL,
            created_at   INTEGER NOT NULL
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn store(
    pool: &SqlitePool,
    id: &str,
    encrypted: &[u8],
    nonce: &[u8],
) -> Result<(), BackendError> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock is before Unix epoch")
        .as_secs() as i64;

    sqlx::query(
        "INSERT INTO images (id, encrypted, nonce, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(id)
    .bind(encrypted)
    .bind(nonce)
    .bind(now)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn fetch(pool: &SqlitePool, id: &str) -> Result<(Vec<u8>, Vec<u8>), BackendError> {
    let row = sqlx::query("SELECT encrypted, nonce FROM images WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(BackendError::NotFound)?;

    let encrypted: Vec<u8> = row.try_get("encrypted")?;
    let nonce: Vec<u8> = row.try_get("nonce")?;

    Ok((encrypted, nonce))
}
