use sea_orm::DatabaseConnection;

use crate::common::errors::AppError;
use crate::common::utils::jwt::createjwt_token;

use super::super::dto::req::{LoginReq, RegisterReq};
use super::super::repositories::repo;

pub async fn login(db: &DatabaseConnection, req: LoginReq) -> Result<String, AppError> {
    let user = repo::find_by_email(db, &req.email).await?;

    let valid = bcrypt::verify(&req.password, &user.password)
        .map_err(|e| AppError::wrap("bcrypt error", e))?;

    if !valid {
        return Err(AppError::Unauthorized);
    }

    let token = createjwt_token(user.id.to_string(), user.email, user.role, 7)
        .map_err(|e| AppError::wrap("jwt error", e))?;

    Ok(token)
}

pub async fn register(db: &DatabaseConnection, req: RegisterReq) -> Result<String, AppError> {
    let hashed = bcrypt::hash(&req.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::wrap("bcrypt error", e))?;

    let mut new_user_data = req;
    new_user_data.password = hashed;

    let user = repo::create_user(db, new_user_data).await?;

    let token = createjwt_token(user.id.to_string(), user.email, user.role, 7)
        .map_err(|e| AppError::wrap("jwt error", e))?;

    Ok(token)
}
