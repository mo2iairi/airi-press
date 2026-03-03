use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use super::category::Category;
use super::tag::Tag;
use super::user::UserResponse;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub author_id: Uuid,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostResponse {
    pub id: i64,
    pub title: String,
    pub summary: Option<String>,
    pub content: String,
    pub author: UserResponse,
    pub published: bool,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostListItem {
    pub id: i64,
    pub title: String,
    pub summary: Option<String>,
    pub author: UserResponse,
    pub published: bool,
    pub categories: Vec<Category>,
    pub tags: Vec<Tag>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePostRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
    pub summary: Option<String>,
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
    pub published: Option<bool>,
    pub category_ids: Option<Vec<i32>>,
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePostRequest {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
    pub category_ids: Option<Vec<i32>>,
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize)]
pub struct PostQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub published: Option<bool>,
    pub author_id: Option<Uuid>,
    pub category_id: Option<i32>,
    pub tag_id: Option<i32>,
    pub search: Option<String>,
}

impl Default for PostQuery {
    fn default() -> Self {
        Self {
            page: Some(1),
            per_page: Some(10),
            published: None,
            author_id: None,
            category_id: None,
            tag_id: None,
            search: None,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

impl<T> PaginatedResponse<T> {
    pub fn new(data: Vec<T>, total: i64, page: i64, per_page: i64) -> Self {
        let total_pages = (total as f64 / per_page as f64).ceil() as i64;
        Self {
            data,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}
