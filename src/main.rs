mod config;
mod handlers;
mod utils;
mod db;
mod enums;
mod entity;
mod middleware;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use std::env;

use sqlx::sqlite::SqlitePool;
use actix_web::middleware::Logger;

use handlers::user::login;
use handlers::fixed_data;
use crate::middleware::auth::AuthMiddleware;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // 加载环境变量
    dotenv().ok();
    env_logger::init();

    // 获取数据库连接地址
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:data.db".to_string());

    // 初始化数据库连接池
    let pool: SqlitePool = db::init_db(&database_url)
        .await
        .expect("Failed to initialize database");

    // 获取运行端口
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("✅ Server running at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone())) // ✅ 注入 SqlitePool
            .service(login)                         // ✅ 注册 login 接口（白名单）
            .wrap(AuthMiddleware)                   // ✅ 添加中间件（拦截 /admin/**）
            .configure(fixed_data::config)          // ✅ 注册其它接口
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
