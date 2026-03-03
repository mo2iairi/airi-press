use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::auth::AuthUser;
use crate::models::user::{CreateUserRequest, Permission, UpdateUserRequest, UserResponse};
use crate::models::AppState;
use crate::services::user;

pub async fn get_current_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<UserResponse>> {
    let user = user::get_user_by_id(&state.pool, auth_user.id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn get_all_users(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<UserResponse>>> {
    // Only admin can view all users
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    let users = user::get_all_users(&state.pool).await?;
    let responses: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    Ok(Json(responses))
}

pub async fn get_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<UserResponse>> {
    // Only admin can view other users
    if !Permission::from(auth_user.permission).is_admin() && auth_user.id != id {
        return Err(AppError::Forbidden);
    }

    let user = user::get_user_by_id(&state.pool, id).await?;
    Ok(Json(UserResponse::from(user)))
}

pub async fn create_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    // Only admin can create users
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let created_user = user::create_user(&state.pool, req).await?;
    Ok(Json(UserResponse::from(created_user)))
}

pub async fn update_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<UserResponse>> {
    // Only admin can update other users, users can update themselves (except permission)
    let is_admin = Permission::from(auth_user.permission).is_admin();
    
    if !is_admin && auth_user.id != id {
        return Err(AppError::Forbidden);
    }

    // Non-admin users cannot change their permission
    let req = if !is_admin && req.permission.is_some() {
        UpdateUserRequest {
            username: req.username,
            password: req.password,
            permission: None,
        }
    } else {
        req
    };

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let updated_user = user::update_user(&state.pool, id, req).await?;
    Ok(Json(UserResponse::from(updated_user)))
}

pub async fn delete_user(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<serde_json::Value>> {
    // Only admin can delete users
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    // Cannot delete yourself
    if auth_user.id == id {
        return Err(AppError::BadRequest("Cannot delete yourself".to_string()));
    }

    user::delete_user(&state.pool, id).await?;
    Ok(Json(serde_json::json!({"message": "User deleted successfully"})))
}
