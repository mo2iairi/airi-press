use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::*;
use crate::AppState;

pub async fn list_tags(
    State(state): State<Arc<AppState>>,
) -> Result<Json<ApiResponse<Vec<Tag>>>, AppError> {
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name ASC")
        .fetch_all(&state.db)
        .await?;

    Ok(Json(ApiResponse::success(tags)))
}

pub async fn get_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<Tag>>, AppError> {
    let tag = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    Ok(Json(ApiResponse::success(tag)))
}

pub async fn create_tag(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, AppError> {
    let tag = sqlx::query_as::<_, Tag>(
        "INSERT INTO tags (name) VALUES ($1) RETURNING *",
    )
    .bind(&payload.name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(ref db_err) if db_err.constraint().is_some() => {
            AppError::BadRequest("Tag name already exists".to_string())
        }
        _ => AppError::Database(e),
    })?;

    Ok(Json(ApiResponse::success(tag)))
}

pub async fn update_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTagRequest>,
) -> Result<Json<ApiResponse<Tag>>, AppError> {
    let name = payload.name
        .ok_or_else(|| AppError::BadRequest("Name is required".to_string()))?;

    let tag = sqlx::query_as::<_, Tag>(
        "UPDATE tags SET name = $1 WHERE id = $2 RETURNING *",
    )
    .bind(&name)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))?;

    Ok(Json(ApiResponse::success(tag)))
}

pub async fn delete_tag(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    let result = sqlx::query("DELETE FROM tags WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tag not found".to_string()));
    }

    Ok(Json(ApiResponse::success(())))
}
