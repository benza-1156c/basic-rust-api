use axum_cookie::{CookieManager, cookie::Cookie};
use chrono::Utc;

pub fn set_token_cookie(cookies: &CookieManager, token: String, name: String, duration: i64) {
    let mut cookie = Cookie::new(name, token);

    let expiration = Utc::now() + chrono::Duration::days(duration);
    let expires_str = expiration.format("%a, %d %b %Y %H:%M:%S GMT").to_string();

    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_secure(true);
    cookie.set_expires(expires_str);

    cookies.add(cookie);
}
