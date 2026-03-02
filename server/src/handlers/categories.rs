use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::*;
use crate::AppState;

pub async fn list_categories(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<Category>>>, AppError> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT * FROM categories ORDER BY name ASC",
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(ApiResponse::success(categories)))
}

pub async fn get_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Category>>, AppError> {
    let category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    Ok(Json(ApiResponse::success(category)))
}

pub async fn create_category(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateCategoryRequest>,
) -> Result<Json<ApiResponse<Category>>, AppError> {
    let description = payload.description.unwrap_or_default();

    let category = sqlx::query_as::<_, Category>(
        "INSERT INTO categories (name, description) VALUES ($1, $2) RETURNING *",
    )
    .bind(&payload.name)
    .bind(&description)
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.constraint().is_some() => {
            AppError::BadRequest("Category name already exists".to_string())
        }
        _ => AppError::Database(e),
    })?;

    Ok(Json(ApiResponse::success(category)))
}

pub async fn update_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryRequest>,
) -> Result<Json<ApiResponse<Category>>, AppError> {
    let existing = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Category not found".to_string()))?;

    let name = payload.name.unwrap_or(existing.name);
    let description = payload.description.unwrap_or(existing.description);

    let category = sqlx::query_as::<_, Category>(
        "UPDATE categories SET name = $1, description = $2 WHERE id = $3 RETURNING *",
    )
    .bind(&name)
    .bind(&description)
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(ApiResponse::success(category)))
}

pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Category not found".to_string()));
    }

    Ok(Json(ApiResponse::success(())))
}
