pub mod utils;
pub mod database;
use sea_orm::DatabaseConnection;
pub use utils::ValidatedJson;


#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}