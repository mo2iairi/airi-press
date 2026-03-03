use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryWithChildren {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>,
    pub children: Vec<CategoryWithChildren>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 255, message = "Category name must be between 1 and 255 characters"))]
    pub name: String,
    pub parent_id: Option<i32>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateCategoryRequest {
    #[validate(length(min = 1, max = 255, message = "Category name must be between 1 and 255 characters"))]
    pub name: Option<String>,
    pub parent_id: Option<i32>,
}
