pub mod database;
pub mod errors;
pub mod utils;
use sea_orm::DatabaseConnection;
pub use utils::ValidatedJson;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}
