use axum::{Json, http::StatusCode, response::IntoResponse};
use sea_orm::DbErr;
use serde_json::json;

pub const ERR_NOT_FOUND: &str = "ไม่พบข้อมูลที่ต้องการ";
pub const ERR_UNAUTHORIZED: &str = "คุณไม่มีสิทธิ์เข้าถึง";
pub const ERR_INTERNAL: &str = "เกิดข้อผิดพลาดภายในระบบ";
pub const ERR_DUPLICATE: &str = "ข้อมูลนี้มีอยู่ในระบบแล้ว";
pub const ERR_INVALID: &str = "ข้อมูลที่ส่งมาไม่ถูกต้อง";

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Duplicate,

    Unauthorized,
    Internal { debug: String },

    Custom { msg: String, debug: Option<String> },
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NotFound => ERR_NOT_FOUND,
            Self::Duplicate => ERR_DUPLICATE,
            Self::Unauthorized => ERR_UNAUTHORIZED,
            Self::Internal { .. } => ERR_INTERNAL,
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
        AppError::Unauthorized => (StatusCode::UNAUTHORIZED, ERR_UNAUTHORIZED.to_string(), None),
        AppError::Internal { debug } => (
            StatusCode::INTERNAL_SERVER_ERROR,
            ERR_INTERNAL.to_string(),
            Some(debug.clone()),
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
        let debug = err.to_string();

        if let Some(sql_err) = err.sql_err() {
            return match sql_err {
                sea_orm::SqlErr::UniqueConstraintViolation(_) => Self::Duplicate,
                sea_orm::SqlErr::ForeignKeyConstraintViolation(_) => Self::Custom {
                    msg: "ข้อมูลอ้างอิงไม่ถูกต้อง".into(),
                    debug: Some(debug),
                },
                _ => Self::Internal { debug },
            };
        }

        match err {
            DbErr::RecordNotFound(_) => Self::Custom {
                msg: ERR_NOT_FOUND.into(),
                debug: Some(debug),
            },

            DbErr::ConnectionAcquire(_) => Self::Custom {
                msg: "ไม่สามารถเชื่อมต่อฐานข้อมูลได้".into(),
                debug: Some(debug),
            },

            DbErr::Conn(_) => Self::Custom {
                msg: "การเชื่อมต่อฐานข้อมูลมีปัญหา".into(),
                debug: Some(debug),
            },

            DbErr::Exec(_) => Self::Custom {
                msg: "ไม่สามารถดำเนินการกับฐานข้อมูลได้".into(),
                debug: Some(debug),
            },

            DbErr::Query(_) => Self::Custom {
                msg: "เกิดข้อผิดพลาดในการค้นหาข้อมูล".into(),
                debug: Some(debug),
            },

            DbErr::RecordNotInserted => Self::Custom {
                msg: "ไม่สามารถบันทึกข้อมูลได้".into(),
                debug: Some(debug),
            },

            DbErr::RecordNotUpdated => Self::Custom {
                msg: "ไม่สามารถอัปเดตข้อมูลได้".into(),
                debug: Some(debug),
            },

            DbErr::Type(_) | DbErr::Json(_) | DbErr::TryIntoErr { .. } => Self::Custom {
                msg: "ข้อมูลไม่ถูกต้องตามรูปแบบที่กำหนด".into(),
                debug: Some(debug),
            },

            _ => Self::Internal { debug },
        }
    }
}

pub trait IntoAppResult<T> {
    fn into_app_result(self) -> Result<T, AppError>;
}

impl<T> IntoAppResult<T> for Result<Option<T>, DbErr> {
    fn into_app_result(self) -> Result<T, AppError> {
        match self {
            Ok(Some(v)) => Ok(v),
            Ok(None) => Err(AppError::NotFound),
            Err(e) => Err(AppError::from(e)),
        }
    }
}
