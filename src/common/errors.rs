use axum::{Json, http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;
use serde_json::json;

pub const ERR_NOT_FOUND: &str = "ไม่พบข้อมูลที่ต้องการ";
pub const ERR_UNAUTHORIZED: &str = "คุณไม่มีสิทธิ์เข้าถึง";
pub const ERR_INTERNAL: &str = "เกิดข้อผิดพลาดภายในระบบ";
pub const ERR_DUPLICATE: &str = "ข้อมูลนี้มีอยู่ในระบบแล้ว";
pub const ERR_INVALID: &str = "ข้อมูลที่ส่งมาไม่ถูกต้อง";
pub const ERR_PROCESSING: &str = "ไม่สามารถดำเนินการได้ในขณะนี้ กรุณาลองใหม่ภายหลัง";

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Duplicate,
    ValidationError(String),

    Unauthorized,
    Internal { debug: String },
    Processing,

    Custom { msg: String, debug: Option<String> },
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NotFound => ERR_NOT_FOUND,
            Self::Duplicate => ERR_DUPLICATE,
            Self::ValidationError(e) => return write!(f, "{}: {}", ERR_INVALID, e),
            Self::Unauthorized => ERR_UNAUTHORIZED,
            Self::Internal { .. } => ERR_INTERNAL,
            Self::Processing => ERR_PROCESSING,
            Self::Custom { msg, .. } => msg.as_str(),
        };
        write!(f, "{}", msg)
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message, debug) = parse_error(&self);

        let body = Json(json!({
            "success": false,
            "message": message,
            "debug": debug,
        }));

        (status, body).into_response()
    }
}

pub fn parse_error(err: &AppError) -> (StatusCode, String, Option<String>) {
    match err {
        AppError::NotFound => (StatusCode::NOT_FOUND, ERR_NOT_FOUND.to_string(), None),
        AppError::Duplicate => (StatusCode::CONFLICT, ERR_DUPLICATE.to_string(), None),
        AppError::ValidationError(field) => (
            StatusCode::BAD_REQUEST,
            ERR_INVALID.to_string(),
            Some(format!("field: {}", field)),
        ),
        AppError::Unauthorized => (StatusCode::UNAUTHORIZED, ERR_UNAUTHORIZED.to_string(), None),
        AppError::Internal { debug } => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ERR_INTERNAL.to_string(),
            Some(debug.clone()),
        ),
        AppError::Processing => (
            StatusCode::UNPROCESSABLE_ENTITY,
            ERR_PROCESSING.to_string(),
            None,
        ),
        AppError::Custom { msg, debug } => (StatusCode::BAD_REQUEST, msg.clone(), debug.clone()),
    }
}

impl AppError {
    pub fn new(status: StatusCode, msg: impl Into<String>) -> Self {
        Self::Custom {
            msg: msg.into(),
            debug: Some(format!("status: {:?}", status)),
        }
    }

    pub fn wrap(msg: impl Into<String>, source: impl std::fmt::Display) -> Self {
        Self::Custom {
            msg: msg.into(),
            debug: Some(source.to_string()),
        }
    }

    pub fn wrap_msg(
        msg: impl Into<String>,
        code: StatusCode,
        source: impl std::fmt::Display,
    ) -> Self {
        Self::Custom {
            msg: msg.into(),
            debug: Some(source.to_string()),
        }
    }
}

impl From<DbErr> for AppError {
    fn from(err: DbErr) -> Self {
        let err_str = err.to_string();

        match err {
            DbErr::RecordNotFound(_) => Self::NotFound,
            _ => {
                if err_str.contains("unique constraint") || err_str.contains("duplicate key") {
                    Self::Duplicate
                } else {
                    Self::Internal { debug: err_str }
                }
            }
        }
    }
}

pub trait DbErrExt {
    fn into_app_err(self) -> AppError;
}

impl DbErrExt for DbErr {
    fn into_app_err(self) -> AppError {
        self.into()
    }
}
