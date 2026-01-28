use std::env;

use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn connect_db() -> Result<DatabaseConnection, DbErr> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");

    let db = Database::connect(&db_url).await?;

    println!("Database Connected Successfully!");

    Ok(db)
}
