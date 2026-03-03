use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::auth::AuthUser;
use crate::models::tag::{CreateTagRequest, Tag, UpdateTagRequest};
use crate::models::user::Permission;
use crate::models::AppState;
use crate::services::tag;

pub async fn get_all_tags(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Tag>>> {
    let tags = tag::get_all_tags(&state.pool).await?;
    Ok(Json(tags))
}

pub async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<Tag>> {
    let t = tag::get_tag_by_id(&state.pool, id).await?;
    Ok(Json(t))
}

pub async fn create_tag(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateTagRequest>,
) -> AppResult<Json<Tag>> {
    if !Permission::from(auth_user.permission).can_create_post() {
        return Err(AppError::Forbidden);
    }

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let t = tag::create_tag(&state.pool, req).await?;
    Ok(Json(t))
}

pub async fn update_tag(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateTagRequest>,
) -> AppResult<Json<Tag>> {
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let t = tag::update_tag(&state.pool, id, req).await?;
    Ok(Json(t))
}

pub async fn delete_tag(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<serde_json::Value>> {
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    tag::delete_tag(&state.pool, id).await?;
    Ok(Json(serde_json::json!({"message": "Tag deleted successfully"})))
}
