use crate::{
    common::errors::AppError,
    entities::user,
    modules::auth::{
        dto::req::{LoginReq, RegisterReq},
        repositories::repo::AuthRepository,
    },
};
use sea_orm::DatabaseConnection;

pub struct AuthUsecases;

impl AuthUsecases {
    pub async fn create_user(
        db: &DatabaseConnection,
        req: RegisterReq,
    ) -> Result<user::Model, AppError> {
        let existing_user = AuthRepository::find_by_email(db, &req.email).await?;

        if existing_user.is_some() {
            return Err(AppError::BadRequest("อีเมลนี้ถูกใช้งานไปแล้ว".to_string()));
        }

        let hashed_password = bcrypt::hash(&req.password, 10)
            .map_err(|_| AppError::InternalServerError("การเข้ารหัสรหัสผ่านผิดพลาด".to_string()))?;

        let new_user = AuthRepository::create_user(db, req, hashed_password).await?;

        Ok(new_user)
    }
}

impl AuthUsecases {
    pub async fn login(db: &DatabaseConnection, req: LoginReq) -> Result<user::Model, AppError> {
        let user = AuthRepository::find_by_email(db, &req.email).await?;

        let user = match user {
            Some(u) => u,
            None => return Err(AppError::AuthError("อีเมลหรือรหัสผ่านไม่ถูกต้อง".to_owned())),
        };

        let is_valid = bcrypt::verify(&req.password, &user.password).map_err(|_| {
            AppError::InternalServerError("เกิดข้อผิดพลาดในการตรวจสอบรหัสผ่าน".to_owned())
        })?;

        if !is_valid {
            return Err(AppError::AuthError("อีเมลหรือรหัสผ่านไม่ถูกต้อง".to_owned()));
        }

        Ok(user)
    }
}
