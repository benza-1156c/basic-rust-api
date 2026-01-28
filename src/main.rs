use crate::common::{AppState, database::postgres::connect_db};
use axum_cookie::CookieLayer;
use dotenvy::dotenv;

mod common;
mod entities;
mod modules;
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db = connect_db().await.expect("Failed to initialize database");

    let state = AppState { db };

    let app = routes::routes::app_router()
        .with_state(state)
        .layer(CookieLayer::strict());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
