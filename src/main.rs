mod config;
mod models;
mod handlers;
mod utils;
mod db;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use handlers::health::health_check;
use handlers::simple_string::get_string;
use handlers::course::{get_courses, create_course};

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

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(config.clone()))
            .service(health_check)
            .service(get_string)
            .service(get_courses)
            .service(create_course)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 