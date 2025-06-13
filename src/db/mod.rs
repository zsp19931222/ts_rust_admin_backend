use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use log::info;

pub async fn init_db(database_url: &str) -> Result<SqlitePool, sqlx::Error> {
    info!("Connecting to database: {}", database_url);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    info!("Database connected successfully!");

    // 创建用户表
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS T_USER (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            role TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create T_USER table");

    // 创建固定数据配置表
    info!("Creating T_FIXED_DATA_CONFIG table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS T_FIXED_DATA_CONFIG(
            key TEXT NOT NULL,
            value TEXT,
            type TEXT(255),
            PRIMARY KEY (key)
        )
        "#,
    )
    .execute(&pool)
    .await?;

    info!("T_FIXED_DATA_CONFIG table created successfully!");

    Ok(pool)
}
