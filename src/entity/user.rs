use serde::{Deserialize, Serialize};
use crate::enums::user::UserRole;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub role: UserRole,
    pub username: String,
} 