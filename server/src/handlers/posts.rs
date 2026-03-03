use axum::{
    extract::{Path, Query, State},
    Json,
};
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::middleware::auth::OptionalAuthUser;
use crate::models::auth::AuthUser;
use crate::models::post::{
    CreatePostRequest, PaginatedResponse, PostListItem, PostQuery, PostResponse, UpdatePostRequest,
};
use crate::models::user::Permission;
use crate::models::AppState;
use crate::services::post;

pub async fn get_posts(
    OptionalAuthUser(auth_user): OptionalAuthUser,
    State(state): State<AppState>,
    Query(mut query): Query<PostQuery>,
) -> AppResult<Json<PaginatedResponse<PostListItem>>> {
    // Non-authenticated users can only see published posts
    if auth_user.is_none() {
        query.published = Some(true);
    } else if let Some(user) = &auth_user {
        // Non-admin users can only see published posts or their own posts
        if !Permission::from(user.permission).can_edit_any_post() {
            if query.published.is_none() {
                query.published = Some(true);
            }
        }
    }

    let posts = post::get_posts(&state.pool, query).await?;
    Ok(Json(posts))
}

pub async fn get_post(
    OptionalAuthUser(auth_user): OptionalAuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<PostResponse>> {
    let post_response = post::get_post_by_id(&state.pool, id).await?;

    // Non-authenticated users can only see published posts
    if !post_response.published {
        if let Some(user) = auth_user {
            let permission = Permission::from(user.permission);
            if !permission.can_edit_any_post() && post_response.author.id != user.id {
                return Err(AppError::NotFound("Post not found".to_string()));
            }
        } else {
            return Err(AppError::NotFound("Post not found".to_string()));
        }
    }

    Ok(Json(post_response))
}

pub async fn create_post(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreatePostRequest>,
) -> AppResult<Json<PostResponse>> {
    let permission = Permission::from(auth_user.permission);
    if !permission.can_create_post() {
        return Err(AppError::Forbidden);
    }

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let created_post = post::create_post(&state.pool, auth_user.id, req).await?;
    let response = post::get_post_by_id(&state.pool, created_post.id).await?;
    Ok(Json(response))
}

pub async fn update_post(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdatePostRequest>,
) -> AppResult<Json<PostResponse>> {
    let permission = Permission::from(auth_user.permission);
    if !permission.can_create_post() {
        return Err(AppError::Forbidden);
    }

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let updated_post = post::update_post(
        &state.pool,
        id,
        auth_user.id,
        permission.can_edit_any_post(),
        req,
    )
    .await?;

    let response = post::get_post_by_id(&state.pool, updated_post.id).await?;
    Ok(Json(response))
}

pub async fn delete_post(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<serde_json::Value>> {
    let permission = Permission::from(auth_user.permission);
    if !permission.can_create_post() {
        return Err(AppError::Forbidden);
    }

    post::delete_post(&state.pool, id, auth_user.id, permission.can_edit_any_post()).await?;
    Ok(Json(serde_json::json!({"message": "Post deleted successfully"})))
}
