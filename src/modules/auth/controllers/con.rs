use crate::{
    common::{
        AppState, ValidatedJson,
        utils::{cookies::set_token_cookie, jwt::createjwt_token},
    },
    modules::auth::{dto::req::RegisterReq, usecases::usecases::AuthUsecases},
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use axum_cookie::CookieManager;
use serde_json::json;

pub fn router() -> Router<AppState> {
    Router::new().route("/register", post(register))
}

async fn register(
    State(state): State<AppState>,
    cookie: CookieManager,
    ValidatedJson(req): ValidatedJson<RegisterReq>,
) -> impl IntoResponse {
    match AuthUsecases::create_user(&state.db, req).await {
        Ok(user) => {
            let access_token = match createjwt_token(
                user.id.to_string(),
                user.email.clone(),
                user.role.clone(),
                1,
            ) {
                Ok(token) => token,
                Err(e) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            };

            let refresh_token = match createjwt_token(user.id.to_string(), user.email, user.role, 1)
            {
                Ok(token) => token,
                Err(e) => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
                }
            };

            set_token_cookie(&cookie, access_token, "access_token".to_owned());
            set_token_cookie(&cookie, refresh_token, "refresh_token".to_owned());

            (
                StatusCode::CREATED,
                Json(json!({
                    "success": true,
                    "message": "สมัครสมาชิกสำเร็จ",
                })),
            )
        }
        .into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": err
            })),
        )
            .into_response(),
    }
}
