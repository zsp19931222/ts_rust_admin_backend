use actix_web::get;
use crate::utils::response::ApiResponse;

#[get("/get-string")]
pub async fn get_string() -> ApiResponse<String> {
    ApiResponse::success("这是一串字符串".to_string())
} 