use serde::{Deserialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(email(message = "อีเมลไม่ถูกต้อง"))]
    pub email: String,
    pub username: String,

    #[validate(length(min = 6, message = "รหัสผ่านต้องมีอย่างน้อย 6 ตัว"))]
    pub password: String,
}
