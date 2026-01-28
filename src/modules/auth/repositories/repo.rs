use sea_orm::{
    ActiveValue::{NotSet, Set},
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter,
};

use crate::{entities::user, modules::auth::dto::req::RegisterReq};

pub struct AuthRepository;

impl AuthRepository {
    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, DbErr> {
        user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn create_user(
        db: &DatabaseConnection,
        req: RegisterReq,
        hashed_password: String,
    ) -> Result<user::Model, DbErr> {
        let new_user = user::ActiveModel {
            id: NotSet,
            user_name: Set(req.username),
            email: Set(req.email),
            password: Set(hashed_password),
            ..Default::default()
        };

        user::Entity::insert(new_user).exec_with_returning(db).await
    }
}
