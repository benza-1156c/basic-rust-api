use axum::Router;

use crate::{common::AppState, modules};

pub fn app_router() -> Router<AppState> {
    Router::new().nest("/api/auth", modules::auth::router())
}
