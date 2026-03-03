use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::PgPool;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::auth::{Claims, LoginRequest, LoginResponse};
use crate::models::user::UserResponse;
use crate::services::user::{get_user_by_username, verify_password};

pub async fn login(pool: &PgPool, config: &Config, req: LoginRequest) -> AppResult<LoginResponse> {
    let user = get_user_by_username(pool, &req.username)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !verify_password(&req.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = create_token(&user.id, &user.username, user.permission, &config.jwt_secret)?;

    Ok(LoginResponse {
        token,
        user: UserResponse::from(user),
    })
}

pub fn create_token(user_id: &uuid::Uuid, username: &str, permission: i16, secret: &str) -> AppResult<String> {
    let now = Utc::now();
    let exp = (now + Duration::days(7)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: *user_id,
        username: username.to_string(),
        permission,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn verify_token(token: &str, secret: &str) -> AppResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;

    Ok(token_data.claims)
}
