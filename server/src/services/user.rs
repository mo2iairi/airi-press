use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::user::{CreateUserRequest, Permission, UpdateUserRequest, User};

pub async fn create_admin_user(pool: &PgPool, config: &Config) -> AppResult<()> {
    let existing = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&config.admin_username)
    .fetch_optional(pool)
    .await?;

    if existing.is_none() {
        let password_hash = hash_password(&config.admin_password)?;
        
        sqlx::query(
            "INSERT INTO users (username, password_hash, permission) VALUES ($1, $2, $3)"
        )
        .bind(&config.admin_username)
        .bind(&password_hash)
        .bind(Permission::ADMIN.0)
        .execute(pool)
        .await?;

        tracing::info!("Admin user created: {}", config.admin_username);
    } else {
        tracing::info!("Admin user already exists: {}", config.admin_username);
    }

    Ok(())
}

pub async fn create_user(pool: &PgPool, req: CreateUserRequest) -> AppResult<User> {
    // Check if username already exists
    let existing = sqlx::query_as::<_, User>(
        "SELECT * FROM users WHERE username = $1"
    )
    .bind(&req.username)
    .fetch_optional(pool)
    .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("Username already exists".to_string()));
    }

    let password_hash = hash_password(&req.password)?;
    let permission = req.permission.unwrap_or(Permission::VIEWER.0);

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (username, password_hash, permission)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
    .bind(&req.username)
    .bind(&password_hash)
    .bind(permission)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_user_by_id(pool: &PgPool, id: Uuid) -> AppResult<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".to_string()))
}

pub async fn get_user_by_username(pool: &PgPool, username: &str) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(user)
}

pub async fn get_all_users(pool: &PgPool) -> AppResult<Vec<User>> {
    let users = sqlx::query_as::<_, User>(
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

pub async fn update_user(pool: &PgPool, id: Uuid, req: UpdateUserRequest) -> AppResult<User> {
    let existing = get_user_by_id(pool, id).await?;

    let username = req.username.unwrap_or(existing.username);
    let password_hash = if let Some(password) = req.password {
        hash_password(&password)?
    } else {
        existing.password_hash
    };
    let permission = req.permission.unwrap_or(existing.permission);

    let user = sqlx::query_as::<_, User>(
        r#"
        UPDATE users
        SET username = $1, password_hash = $2, permission = $3, updated_at = NOW()
        WHERE id = $4
        RETURNING *
        "#
    )
    .bind(&username)
    .bind(&password_hash)
    .bind(permission)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn delete_user(pool: &PgPool, id: Uuid) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("User not found".to_string()));
    }

    Ok(())
}

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> AppResult<bool> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::InternalError(format!("Failed to parse password hash: {}", e)))?;
    
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
