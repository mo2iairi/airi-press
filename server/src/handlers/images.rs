use axum::{
    extract::{Multipart, Path, State},
    Json,
};

use crate::error::{AppError, AppResult};
use crate::models::auth::AuthUser;
use crate::models::image::{ImageResponse, UploadImageResponse};
use crate::models::user::Permission;
use crate::models::AppState;
use crate::services::image;

pub async fn get_all_images(
    auth_user: AuthUser,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<ImageResponse>>> {
    if !Permission::from(auth_user.permission).can_create_post() {
        return Err(AppError::Forbidden);
    }

    let images = image::get_all_images(&state.pool, &state.config).await?;
    Ok(Json(images))
}

pub async fn get_image(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ImageResponse>> {
    let img = image::get_image_by_id(&state.pool, &state.config, id).await?;
    Ok(Json(img))
}

pub async fn upload_image(
    auth_user: AuthUser,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> AppResult<Json<UploadImageResponse>> {
    if !Permission::from(auth_user.permission).can_create_post() {
        return Err(AppError::Forbidden);
    }

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to read multipart: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();
        if name == "file" {
            let file_name = field
                .file_name()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "upload.png".to_string());
            let content_type = field.content_type().map(|s| s.to_string());
            let data = field
                .bytes()
                .await
                .map_err(|e| AppError::BadRequest(format!("Failed to read file: {}", e)))?;

            // Validate file size (max 10MB)
            if data.len() > 10 * 1024 * 1024 {
                return Err(AppError::BadRequest("File size exceeds 10MB limit".to_string()));
            }

            // Validate file type
            let allowed_types = ["image/png", "image/jpeg", "image/gif", "image/webp", "image/svg+xml"];
            if let Some(ref ct) = content_type {
                if !allowed_types.contains(&ct.as_str()) {
                    return Err(AppError::BadRequest(format!(
                        "Invalid file type: {}. Allowed types: {:?}",
                        ct, allowed_types
                    )));
                }
            }

            let response = image::upload_image(
                &state.pool,
                &state.config,
                &file_name,
                content_type.as_deref(),
                &data,
            )
            .await?;

            return Ok(Json(response));
        }
    }

    Err(AppError::BadRequest("No file provided".to_string()))
}

pub async fn delete_image(
    auth_user: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<serde_json::Value>> {
    if !Permission::from(auth_user.permission).is_admin() {
        return Err(AppError::Forbidden);
    }

    image::delete_image(&state.pool, &state.config, id).await?;
    Ok(Json(serde_json::json!({"message": "Image deleted successfully"})))
}
