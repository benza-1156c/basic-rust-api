pub mod ctx;
pub mod database;
pub mod errors;
pub mod utils;

pub use ctx::Ctx;
pub use errors::AppError;

use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

pub type ApiResult<T = axum::Json<serde_json::Value>> = Result<T, AppError>;