use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct RegisterReq {
    #[validate(email(message = "รูปแบบอีเมลไม่ถูกต้อง"))]
    pub email: String,

    #[validate(length(min = 1, message = "กรุณากรอกชื่อผู้ใช้"))]
    pub username: String,

    #[validate(length(min = 6, message = "รหัสผ่านต้องยาว6ตัวขึ้นไป"))]
    pub password: String,
}
