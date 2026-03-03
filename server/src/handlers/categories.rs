use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::auth::AuthUser;
use crate::models::category::{Category, CategoryWithChildren, CreateCategoryRequest, UpdateCategoryRequest};
use crate::models::user::Permission;
use crate::models::AppState;
use crate::services::category;

pub async fn get_all_categories(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<Category>>> {
    let categories = category::get_all_categories(&state.pool).await?;
    Ok(Json(categories))
}

pub async fn get_categories_tree(
    State(state): State<AppState>,
) -> AppResult<Json<Vec<CategoryWithChildren>>> {
    let tree = category::get_categories_tree(&state.pool).await?;
    Ok(Json(tree))
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<Category>> {
    let cat = category::get_category_by_id(&state.pool, id).await?;
    Ok(Json(cat))
}

pub async fn create_category(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Json(req): Json<CreateCategoryRequest>,
) -> AppResult<Json<Category>> {
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let cat = category::create_category(&state.pool, req).await?;
    Ok(Json(cat))
}

pub async fn update_category(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCategoryRequest>,
) -> AppResult<Json<Category>> {
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;

    let cat = category::update_category(&state.pool, id, req).await?;
    Ok(Json(cat))
}

pub async fn delete_category(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<serde_json::Value>> {
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    category::delete_category(&state.pool, id).await?;
    Ok(Json(serde_json::json!({"message": "Category deleted successfully"})))
}
