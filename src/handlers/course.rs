use actix_web::{get, post, web, HttpResponse};
use crate::models::course::{Course, CourseList};
use crate::utils::response::ApiResponse;
use crate::db;
use chrono::{DateTime, Utc};

#[get("/courses")]
pub async fn get_courses() -> ApiResponse<CourseList> {
    let pool = db::get_pool();
    
    // 从数据库获取课程列表
    let courses = sqlx::query_as!(
        Course,
        r#"
        SELECT id, title, content, created_at as "created_at: _"
        FROM courses
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    ApiResponse::success(CourseList { courses })
}

#[post("/courses")]
pub async fn create_course(course: web::Json<Course>) -> ApiResponse<Course> {
    let pool = db::get_pool();
    
    // 插入新课程
    let result = sqlx::query!(
        r#"
        INSERT INTO courses (title, content, created_at)
        VALUES ($1, $2, CURRENT_TIMESTAMP)
        RETURNING id, title, content, created_at
        "#,
        course.title,
        course.content
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(record) => {
            let new_course = Course {
                id: Some(record.id),
                title: record.title,
                content: record.content,
                created_at: record.created_at.map(|dt| DateTime::from_utc(dt, Utc)),
            };
            ApiResponse::success(new_course)
        },
        Err(e) => ApiResponse::error(500, format!("Failed to create course: {}", e)),
    }
}