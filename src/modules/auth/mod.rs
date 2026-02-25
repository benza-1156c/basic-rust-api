use axum::{Router, routing::post};

use crate::common::AppState;

pub mod controllers;
pub mod dto;
pub mod repositories;
pub mod usecases;

use controllers::con::{login, register};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}
