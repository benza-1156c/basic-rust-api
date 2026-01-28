pub mod controllers;
pub mod dto;
pub mod repositories;
pub mod usecases;

use axum::Router;
use controllers::con;

use crate::common::AppState;

pub fn router() -> Router<AppState> {
    Router::new().merge(con::router())
}
