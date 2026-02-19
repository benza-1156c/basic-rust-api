use crate::{common::AppState, modules::auth::auth_routes};
use axum::Router;

pub fn app_router() -> Router<AppState> {
    Router::new().nest("/api", Router::new().nest("/auth", auth_routes()))
}
