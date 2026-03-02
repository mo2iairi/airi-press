use axum::{
    extract::Request,
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::errors::AppError;
use crate::models::Claims;

pub fn create_token(
    user_id: &str,
    username: &str,
    permission: i32,
    secret: &str,
) -> Result<String, AppError> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(72))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        permission,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Token creation failed: {}", e)))
}

pub fn verify_token(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
}

/// Middleware that requires authentication
pub async fn require_auth(
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_default();

    let token = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or_else(|| {
            tracing::warn!("require_auth: Missing authorization header for {} {}", request.method(), request.uri());
            AppError::Unauthorized("Missing authorization header".to_string())
        })?;

    let claims = verify_token(token, &jwt_secret)?;
    tracing::info!("require_auth: passed for user={} permission={}", claims.username, claims.permission);
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Middleware that requires author permission (permission >= 1)
pub async fn require_author(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    tracing::error!("require_author: ENTERED for {} {}", request.method(), request.uri());
    
    let claims = request
        .extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| {
            tracing::error!("require_author: No Claims in extensions for {} {}", request.method(), request.uri());
            AppError::Unauthorized("Not authenticated".to_string())
        })?;

    if claims.permission < 1 {
        tracing::error!("require_author: Insufficient permission for user={}", claims.username);
        return Err(AppError::Forbidden("Author permission required".to_string()));
    }

    tracing::error!("require_author: passed for user={}", claims.username);
    Ok(next.run(request).await)
}

/// Middleware that requires admin permission (permission == 255)
pub async fn require_admin(
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<Claims>()
        .ok_or_else(|| AppError::Unauthorized("Not authenticated".to_string()))?;

    if claims.permission != 255 {
        return Err(AppError::Forbidden("Admin permission required".to_string()));
    }

    Ok(next.run(request).await)
}
