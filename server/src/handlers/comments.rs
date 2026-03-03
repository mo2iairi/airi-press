use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::auth::AuthUser;
use crate::models::comment::{CommentResponse, CreateCommentRequest, UpdateCommentRequest};
use crate::models::user::Permission;
use crate::models::AppState;
use crate::services::comment;

pub async fn get_comments_by_post(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
) -> AppResult<Json<Vec<CommentResponse>>> {
    let comments = comment::get_comments_by_post(&state.pool, post_id).await?;
    Ok(Json(comments))
}

pub async fn get_comment(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<CommentResponse>> {
    let c = comment::get_comment_by_id(&state.pool, id).await?;
    Ok(Json(c))
}

pub async fn create_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
    Json(req): Json<CreateCommentRequest>,
) -> AppResult<Json<CommentResponse>> {
    if !Permission::from(auth_user.permission).can_comment() {
        return Err(AppError::Forbidden);
    }

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let c = comment::create_comment(&state.pool, post_id, auth_user.id, req).await?;
    let response = comment::get_comment_by_id(&state.pool, c.id).await?;
    Ok(Json(response))
}

pub async fn update_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateCommentRequest>,
) -> AppResult<Json<CommentResponse>> {
    let is_admin = Permission::from(auth_user.permission).is_admin();

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let c = comment::update_comment(&state.pool, id, auth_user.id, is_admin, req).await?;
    let response = comment::get_comment_by_id(&state.pool, c.id).await?;
    Ok(Json(response))
}

pub async fn delete_comment(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<serde_json::Value>> {
    let is_admin = Permission::from(auth_user.permission).is_admin();

    comment::delete_comment(&state.pool, id, auth_user.id, is_admin).await?;
    Ok(Json(serde_json::json!({"message": "Comment deleted successfully"})))
}
