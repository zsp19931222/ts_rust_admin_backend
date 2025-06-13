use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use sqlx::sqlite::SqlitePool;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, Header, EncodingKey};
use md5::{Md5, Digest};

use crate::enums::user::UserRole;
use crate::utils::response::ApiResponse;
use crate::middleware::auth::Claims;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub role: UserRole,
    pub username: String,
}

#[post("/login")]
pub async fn login(
    pool: web::Data<SqlitePool>,
    login_data: web::Json<LoginRequest>,
) -> HttpResponse {
    // 将用户输入的密码转换为 MD5 哈希值
    let mut hasher = Md5::new();
    hasher.update(login_data.password.as_bytes());
    let result = hasher.finalize();
    let password_md5 = format!("{:x}", result);

    let result = sqlx::query!(
        r#"
        SELECT username, role FROM T_USER 
        WHERE username = ? AND password = ?
        "#,
        login_data.username,
        password_md5 // 使用 MD5 哈希值进行查询
    )
    .fetch_optional(&**pool)
    .await;

    match result {
        Ok(Some(row)) => {
            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("valid timestamp")
                .timestamp();

            let claims = Claims {
                sub: row.username.clone(),
                role: UserRole::Admin, // 你可以根据 row.role 做映射
                exp: expiration,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(
                    std::env::var("JWT_SECRET")
                        .unwrap_or_else(|_| "your-secret-key".to_string())
                        .as_bytes(),
                ),
            )
            .unwrap();

            let response = LoginResponse {
                token,
                role: UserRole::Admin,
                username: row.username,
            };

            HttpResponse::Ok().json(ApiResponse::success(response))
        }
        _ => HttpResponse::Unauthorized().json(ApiResponse::<()>::error(401, "用户名或密码错误".to_string())),
    }
}
