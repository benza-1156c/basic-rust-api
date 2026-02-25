use sea_orm::*;

use crate::common::errors::{AppError, IntoAppResult};
use crate::entities::user;
use crate::modules::auth::dto::req::RegisterReq;

pub async fn find_by_email(db: &DatabaseConnection, email: &str) -> Result<user::Model, AppError> {
    user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .into_app_result()
}

pub async fn create_user(
    db: &DatabaseConnection,
    dto: RegisterReq,
) -> Result<user::Model, AppError> {
    let new_user = user::ActiveModel {
        id: Set(uuid::Uuid::new_v4()),
        email: Set(dto.email),
        user_name: Set(dto.username),
        password: Set(dto.password),
        avatar: Set(None),
        role: Set("user".to_string()),
    };

    Ok(new_user.insert(db).await?)
}
