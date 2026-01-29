use crate::{
    common::errors::AppError,
    entities::user,
    modules::auth::{dto::req::RegisterReq, repositories::repo::AuthRepository},
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
