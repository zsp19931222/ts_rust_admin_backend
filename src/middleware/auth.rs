use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header,
    Error, HttpResponse,
    body::BoxBody, body::MessageBody,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use crate::enums::user::UserRole;
use crate::utils::response::ApiResponse;

// 定义白名单路径
const WHITELIST_PATHS: [&str; 1] = ["/login"];

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: UserRole,
    pub exp: i64,
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let path = req.path();
    
        // ✅ 跳过非 /admin 路径
        if !path.starts_with("/manage") {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?.map_into_boxed_body();
                Ok(res)
            });
        }
    
        // ✅ 可选白名单检查
        if WHITELIST_PATHS.contains(&path) {
            let fut = self.service.call(req);
            return Box::pin(async move {
                let res = fut.await?.map_into_boxed_body();
                Ok(res)
            });
        }
    
        // ✅ 提取 token
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
    
        let token = match auth_header {
            Some(token) if token.starts_with("Bearer ") => {
                token.trim_start_matches("Bearer ")
            }
            _ => {
                return Box::pin(async move {
                    let response = ApiResponse::<()>::error(401, "未提供有效的认证令牌".to_string());
                    let res = req.into_response(
                        HttpResponse::Ok()
                            .json(response)
                            .map_into_boxed_body()
                    );
                    Ok(res)
                });
            }
        };
    
        // ✅ 验证 token
        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(_claims) => {
                let fut = self.service.call(req);
                Box::pin(async move {
                    let res = fut.await?.map_into_boxed_body();
                    Ok(res)
                })
            }
            Err(_) => Box::pin(async move {
                let response = ApiResponse::<()>::error(401, "无效的认证令牌".to_string());
                let res = req.into_response(
                    HttpResponse::Ok()
                        .json(response)
                        .map_into_boxed_body()
                );
                Ok(res)
            }),
        }
    }
} 