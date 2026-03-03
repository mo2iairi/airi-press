use axum::{extract::State, Json};
use validator::Validate;

use crate::error::{AppError, AppResult};
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::models::AppState;
use crate::services::auth::login;

pub async fn login_handler(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> AppResult<Json<LoginResponse>> {
    req.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    
    let response = login(&state.pool, &state.config, req).await?;
    Ok(Json(response))
}
