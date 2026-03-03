use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::config::Config;
use crate::error::{AppError, AppResult};
use crate::models::image::{Image, ImageResponse, UploadImageResponse};
use crate::services::github::GithubService;

pub async fn upload_image(
    pool: &PgPool,
    config: &Config,
    file_name: &str,
    content_type: Option<&str>,
    data: &[u8],
) -> AppResult<UploadImageResponse> {
    // Generate unique file name
    let extension = file_name
        .rsplit('.')
        .next()
        .unwrap_or("png");
    let unique_name = format!("{}.{}", Uuid::new_v4(), extension);
    let relative_path = format!("images/{}", unique_name);

    // Upload to GitHub
    let github = GithubService::new(config);
    let _download_url = github.upload_file(&relative_path, data).await?;

    // Save metadata to database
    let image = sqlx::query_as::<_, Image>(
        r#"
        INSERT INTO images (relative_path, original_name, mime_type, size)
        VALUES ($1, $2, $3, $4)
        RETURNING *
        "#
    )
    .bind(&relative_path)
    .bind(file_name)
    .bind(content_type)
    .bind(data.len() as i64)
    .fetch_one(pool)
    .await?;

    let url = github.get_raw_url(&relative_path);

    Ok(UploadImageResponse {
        id: image.id,
        url,
        relative_path: image.relative_path,
    })
}

pub async fn get_image_by_id(pool: &PgPool, config: &Config, id: i64) -> AppResult<ImageResponse> {
    let image = sqlx::query_as::<_, Image>("SELECT * FROM images WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Image not found".to_string()))?;

    let github = GithubService::new(config);
    let url = github.get_raw_url(&image.relative_path);

    Ok(ImageResponse {
        id: image.id,
        relative_path: image.relative_path,
        url,
        original_name: image.original_name,
        mime_type: image.mime_type,
        size: image.size,
        created_at: image.created_at,
    })
}

pub async fn get_all_images(pool: &PgPool, config: &Config) -> AppResult<Vec<ImageResponse>> {
    let images = sqlx::query_as::<_, Image>(
        "SELECT * FROM images ORDER BY created_at DESC"
    )
    .fetch_all(pool)
    .await?;

    let github = GithubService::new(config);
    let responses = images
        .into_iter()
        .map(|image| {
            let url = github.get_raw_url(&image.relative_path);
            ImageResponse {
                id: image.id,
                relative_path: image.relative_path,
                url,
                original_name: image.original_name,
                mime_type: image.mime_type,
                size: image.size,
                created_at: image.created_at,
            }
        })
        .collect();

    Ok(responses)
}

pub async fn delete_image(pool: &PgPool, config: &Config, id: i64) -> AppResult<()> {
    let image = sqlx::query_as::<_, Image>("SELECT * FROM images WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Image not found".to_string()))?;

    // Delete from GitHub
    let github = GithubService::new(config);
    github.delete_file(&image.relative_path).await?;

    // Delete from database
    sqlx::query("DELETE FROM images WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
