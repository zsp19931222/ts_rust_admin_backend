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

    // 创建统计信息表
    info!("Creating statistics table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS statistics (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            countries INTEGER NOT NULL DEFAULT 0,
            successful_students INTEGER NOT NULL DEFAULT 0,
            total_courses INTEGER NOT NULL DEFAULT 0,
            yearly_teaching_hours INTEGER NOT NULL DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .execute(&pool)
    .await?;
    info!("Statistics table created successfully!");

    // 创建固定数据配置表（只在不存在时创建，不会清空数据）
    info!("Creating T_FIXED_DATA_CONFIG table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS T_FIXED_DATA_CONFIG(
            key TEXT NOT NULL, --键
            value TEXT, --值
            type TEXT(255), --类型
            PRIMARY KEY (key)
        )
        "#,
    )
    .execute(&pool)
    .await?;
    info!("T_FIXED_DATA_CONFIG table created successfully!");

    DB_POOL.set(pool).unwrap();
    Ok(())
}

pub fn get_pool() -> &'static SqlitePool {
    DB_POOL.get().expect("Database not initialized")
} 