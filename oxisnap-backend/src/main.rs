mod crypto;
mod db;
mod error;
mod routes;

use std::sync::Arc;

use axum::{Router, extract::DefaultBodyLimit, routing::{get, post}};
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub key: Arc<[u8; 32]>,
    pub base_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()?;

    let base_url = std::env::var("OXISNAP_BASE_URL")
        .unwrap_or_else(|_| format!("http://localhost:{}", port));

    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:oxisnap.db?mode=rwc".to_string());

    let pool = SqlitePoolOptions::new().connect(&db_url).await?;
    db::setup(&pool).await?;

    let key_path = std::path::Path::new("oxisnap.key");
    let key = Arc::new(crypto::load_or_create_key(key_path)?);

    let state = AppState { pool, key, base_url };

    let app = Router::new()
        .route("/upload", post(routes::upload))
        .route("/image/{id}", get(routes::get_image))
        .layer(DefaultBodyLimit::disable())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    tracing::info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
