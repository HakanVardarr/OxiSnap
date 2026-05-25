use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BackendError {
    #[error("database error: {0}")]
    Db(#[from] sqlx::Error),

    #[error("encryption error")]
    Encryption,

    #[error("image not found")]
    NotFound,

    #[error("invalid data")]
    InvalidData,
}

impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        let status = match &self {
            BackendError::NotFound => StatusCode::NOT_FOUND,
            BackendError::InvalidData => StatusCode::BAD_REQUEST,
            BackendError::Encryption | BackendError::Db(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, self.to_string()).into_response()
    }
}
