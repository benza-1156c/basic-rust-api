use axum::{
    body::Bytes,
    extract::{FromRequest, FromRequestParts, Request},
};
use axum_cookie::CookieManager;
use sea_orm::DatabaseConnection;
use serde::de::DeserializeOwned;
use serde_json::json;
use validator::Validate;

use crate::common::{
    AppState,
    errors::{AppError, ERR_INVALID},
};

pub struct Ctx {
    db: DatabaseConnection,
    cookies: CookieManager,
    body_bytes: Option<Bytes>,
}

impl Ctx {
    #[inline]
    pub fn db(&self) -> &DatabaseConnection{
        &self.db
    }

    #[inline]
    pub fn cookies(&self) -> &CookieManager {
        &self.cookies
    }

    pub fn body<T>(&self) -> Result<T, AppError>
    where
        T: DeserializeOwned + Validate,
    {
        let bytes = self
            .body_bytes
            .as_ref()
            .ok_or_else(|| AppError::wrap(ERR_INVALID, "body empty"))?;

        let data: T = serde_json::from_slice(bytes).map_err(|e| AppError::wrap(ERR_INVALID, e))?;

        data.validate().map_err(|e| {
            let clean_msg = format_validation_error(&e);
            AppError::wrap(clean_msg, e)
        })?;

        Ok(data)
    }

    #[inline]
    pub fn json<T: serde::Serialize>(
        &self,
        data: T,
    ) -> Result<axum::Json<serde_json::Value>, AppError> {
        Ok(axum::Json(json!({
            "success": true,
            "data": data
        })))
    }

    #[inline]
    pub fn ok(&self) -> Result<axum::Json<serde_json::Value>, AppError> {
        Ok(axum::Json(json!({
            "success": true
        })))
    }
}

impl FromRequest<AppState> for Ctx {
    type Rejection = AppError;

    async fn from_request(req: Request, state: &AppState) -> Result<Self, Self::Rejection> {
        let (mut parts, body) = req.into_parts();

        let cookies = CookieManager::from_request_parts(&mut parts, state)
            .await
            .map_err(|_| AppError::wrap("Failed to extract cookies", "CookieManager error"))?;

        let req = axum::http::Request::from_parts(parts, body);
        let body_bytes = Bytes::from_request(req, state).await.ok();

        Ok(Ctx {
            db: state.db.clone(),
            cookies,
            body_bytes,
        })
    }
}

#[inline]
fn format_validation_error(errors: &validator::ValidationErrors) -> String {
    errors
        .field_errors()
        .iter()
        .next()
        .and_then(|(_field, errs)| errs.first())
        .map(|e| {
            e.message
                .as_ref()
                .map(|m| m.to_string())
                .unwrap_or_else(|| e.code.to_string())
        })
        .unwrap_or_else(|| "ข้อมูลไม่ถูกต้อง".to_string())
}
