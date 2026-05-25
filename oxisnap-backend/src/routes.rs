use axum::{
    body::Bytes,
    extract::{Path, State},
    http::header,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

use crate::{AppState, crypto, db, error::BackendError};

pub async fn upload(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<String, BackendError> {
    let id = Uuid::new_v4().to_string();
    let (encrypted, nonce) = crypto::encrypt(&state.key, &body)?;
    db::store(&state.pool, &id, &encrypted, &nonce).await?;
    Ok(format!("{}/image/{}", state.base_url, id))
}

pub async fn get_image(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response, BackendError> {
    let (encrypted, nonce_vec) = db::fetch(&state.pool, &id).await?;
    let nonce: [u8; 12] = nonce_vec.try_into().map_err(|_| BackendError::InvalidData)?;
    let plaintext = crypto::decrypt(&state.key, &encrypted, &nonce)?;
    Ok(([(header::CONTENT_TYPE, "image/png")], plaintext).into_response())
}
