use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Course {
    pub id: Option<i64>,
    pub title: String,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CourseList {
    pub courses: Vec<Course>,
} 