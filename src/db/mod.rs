use sqlx::sqlite::SqlitePool;
use std::sync::OnceLock;
use log::info;

static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

pub async fn init_db(database_url: &str) -> Result<(), sqlx::Error> {
    info!("Connecting to database: {}", database_url);
    let pool = SqlitePool::connect(database_url).await?;
    info!("Database connected successfully!");
    
    // 创建课程表
    info!("Creating courses table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS courses (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;
    info!("Courses table created successfully!");

    DB_POOL.set(pool).unwrap();
    Ok(())
}

pub fn get_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database not initialized")
} 