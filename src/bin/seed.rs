use sqlx::sqlite::SqlitePool;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;

    // 插入测试数据
    sqlx::query(
        r#"
        INSERT INTO courses (title, content) VALUES
        ('Rust 基础教程', 'Rust 是一门系统编程语言，专注于安全性和并发性。'),
        ('Web 开发入门', '学习使用 Rust 进行 Web 开发的基础知识。'),
        ('数据库操作', '使用 Rust 进行数据库操作和 ORM 实践。')
        "#
    )
    .execute(&pool)
    .await?;

    println!("Seed data inserted successfully!");
    Ok(())
} 