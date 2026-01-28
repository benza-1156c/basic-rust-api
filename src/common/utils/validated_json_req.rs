use axum::{
    Json,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use serde_json::json;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|_| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "message": "รูปแบบข้อมูลไม่ถูกต้อง",
                })),
            )
                .into_response()
        })?;

        if let Err(errors) = data.validate() {
            let error_message = errors
                .field_errors()
                .iter()
                .next()
                .and_then(|(_field, errs)| errs.first())
                .map(|e| e.message.as_ref().unwrap_or(&e.code))
                .unwrap_or(&std::borrow::Cow::Borrowed("ข้อมูลไม่ถูกต้อง"))
                .to_string();
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "success": false,
                    "message": error_message,
                })),
            )
                .into_response());
        }
        Ok(ValidatedJson(data))
    }
}
