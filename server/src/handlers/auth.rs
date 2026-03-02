use std::sync::Arc;

use axum::{extract::State, Extension, Json};

use crate::errors::AppError;
use crate::middleware::create_token;
use crate::models::*;
use crate::AppState;

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid username or password".to_string()))?;

    let valid = bcrypt::verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::Internal("Password verification failed".to_string()))?;

    if !valid {
        return Err(AppError::Unauthorized("Invalid username or password".to_string()));
    }

    let token = create_token(
        &user.id.to_string(),
        &user.username,
        user.permission,
        &state.config.jwt_secret,
    )?;

    Ok(Json(ApiResponse::success(AuthResponse {
        token,
        user: user.into(),
    })))
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    if payload.username.len() < 3 {
        return Err(AppError::BadRequest("Username must be at least 3 characters".to_string()));
    }
    if payload.password.len() < 6 {
        return Err(AppError::BadRequest("Password must be at least 6 characters".to_string()));
    }

    // Check if username already exists
    let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE username = $1")
        .bind(&payload.username)
        .fetch_one(&state.db)
        .await?;

    if existing > 0 {
        return Err(AppError::BadRequest("Username already exists".to_string()));
    }

    let password_hash = bcrypt::hash(&payload.password, bcrypt::DEFAULT_COST)
        .map_err(|e| AppError::Internal(format!("Password hashing failed: {}", e)))?;

    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password_hash, permission) VALUES ($1, $2, 0) RETURNING *",
    )
    .bind(&payload.username)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await?;

    let token = create_token(
        &user.id.to_string(),
        &user.username,
        user.permission,
        &state.config.jwt_secret,
    )?;

    Ok(Json(ApiResponse::success(AuthResponse {
        token,
        user: user.into(),
    })))
}

pub async fn me(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(uuid::Uuid::parse_str(&claims.sub).map_err(|_| AppError::Internal("Invalid user ID".to_string()))?)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(ApiResponse::success(user.into())))
}
