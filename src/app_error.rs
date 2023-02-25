use axum::{http::StatusCode, response::IntoResponse, Json};
use firestore::errors::FirestoreError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    DbError(#[from] FirestoreError),
    #[error("Invalid Parameters :{0}")]
    InvalidParametersError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::DbError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("{:?}", err),
                }),
            )
                .into_response(),
            AppError::InvalidParametersError(err) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("{:?}", err),
                }),
            )
                .into_response(),
        }
    }
}
