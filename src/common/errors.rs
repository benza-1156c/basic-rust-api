use axum::{Json, http::StatusCode, response::IntoResponse};
use jsonwebtoken::errors::Error as JwtError;
use sea_orm::DbErr;
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    DbError(DbErr),
    BadRequest(String),
    NotFound(String),
    AuthError(String),
    Unauthorized(String),
    JwtError(String),
    InternalServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::DbError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::JwtError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::AuthError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "success": false,
            "error": message
        }));

        (status, body).into_response()
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        AppError::DbError(err)
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        AppError::JwtError(err.to_string())
    }
}