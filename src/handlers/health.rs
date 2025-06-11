use actix_web::get;
use crate::utils::response::ApiResponse;

#[get("/health")]
pub async fn health_check() -> ApiResponse<String> {
    ApiResponse::success("服务运行正常！".to_string())
} 