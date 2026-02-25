use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginReq {
    #[validate(email(message = "อีเมลไม่ถูกต้อง"))]
    pub email: String,

    #[validate(length(min = 6, message = "รหัสผ่านต้องมีอย่างน้อย 6 ตัว"))]
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(email(message = "อีเมลไม่ถูกต้อง"))]
    pub email: String,

    #[validate(length(min = 2, message = "ชื่อผู้ใช้ต้องมีอย่างน้อย 2 ตัว"))]
    pub username: String,

    #[validate(length(min = 6, message = "รหัสผ่านต้องมีอย่างน้อย 6 ตัว"))]
    pub password: String,
}
