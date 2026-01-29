use crate::{
    common::{
        AppState, ValidatedJson,
        errors::AppError,
        utils::{cookies::set_token_cookie, jwt::createjwt_token},
    },
    modules::auth::{
        dto::req::{LoginReq, RegisterReq},
        usecases::usecases::AuthUsecases,
    },
};
use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use axum_cookie::CookieManager;
use serde_json::json;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

async fn register(
    State(state): State<AppState>,
    cookie: CookieManager,
    ValidatedJson(req): ValidatedJson<RegisterReq>,
) -> Result<impl IntoResponse, AppError> {
    let user = AuthUsecases::create_user(&state.db, req).await?;

    let access_token = createjwt_token(
        user.id.to_string(),
        user.email.clone(),
        user.role.clone(),
        1,
    )?;

    let refresh_token = createjwt_token(user.id.to_string(), user.email, user.role, 7)?;

    set_token_cookie(&cookie, access_token, "access_token".to_owned(), 1);
    set_token_cookie(&cookie, refresh_token, "refresh_token".to_owned(), 7);

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "message": "สมัครสมาชิกสำเร็จ",
        })),
    ))
}

async fn login(
    State(state): State<AppState>,
    cookie: CookieManager,
    ValidatedJson(req): ValidatedJson<LoginReq>,
) -> Result<impl IntoResponse, AppError> {
    let user = AuthUsecases::login(&state.db, req).await?;

    let access_token = createjwt_token(
        user.id.to_string(),
        user.email.clone(),
        user.role.clone(),
        1,
    )?;

    let refresh_token = createjwt_token(
        user.id.to_string(),
        user.email.clone(),
        user.role.clone(),
        7,
    )?;

    set_token_cookie(&cookie, access_token, "access_token".to_owned(), 1);
    set_token_cookie(&cookie, refresh_token, "refresh_token".to_owned(), 7);

    Ok((
        StatusCode::OK,
        Json(json!({
            "success": true,
            "message": "เข้าสู่ระบบสำเร็จ",
            "data": {
                "id": user.id,
                "email": user.email,
                "role": user.role
            },
        })),
    ))
}
