mod config;
mod handlers;
mod utils;
mod db;
mod enums;
mod entity;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use handlers::fixed_data::{get_fixed_data, get_fixed_data_config, update_fixed_data_config};
use std::env;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let config = config::Config::from_env();

    // 初始化数据库
    db::init_db(&config.database_url)
        .await
        .expect("Failed to initialize database");

    println!("Server running at http://localhost:8080");
    // 获取环境变量中的端口，默认为8080
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(get_fixed_data)
            .service(get_fixed_data_config)
            .service(update_fixed_data_config)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
} 