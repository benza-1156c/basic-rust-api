use sea_orm::DatabaseConnection;

use crate::{
    entities::user,
    modules::auth::{dto::req::RegisterReq, repositories::repo::AuthRepository},
};

pub struct AuthUsecases;

impl AuthUsecases {
    pub async fn create_user(
        db: &DatabaseConnection,
        req: RegisterReq,
    ) -> Result<user::Model, String> {
        let existing_user = AuthRepository::find_by_email(db, &req.email)
            .await
            .map_err(|e| e.to_string())?;

        if existing_user.is_some() {
            return Err("อีเมลนี้ถูกใช้งานไปแล้ว".to_string());
        }

        let hashed_password = bcrypt::hash(&req.password, 10)
            .map_err(|_| "เกิดข้อผิดพลาดในการเข้ารหัสรหัสผ่าน".to_string())?;

        let new_user = AuthRepository::create_user(db, req, hashed_password)
            .await
            .map_err(|e| e.to_string())?;

        Ok(new_user)
    }
}
