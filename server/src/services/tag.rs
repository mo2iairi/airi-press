use sqlx::PgPool;

use crate::error::{AppError, AppResult};
use crate::models::tag::{CreateTagRequest, Tag, UpdateTagRequest};

pub async fn create_tag(pool: &PgPool, req: CreateTagRequest) -> AppResult<Tag> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"
        INSERT INTO tags (name)
        VALUES ($1)
        RETURNING *
        "#
    )
    .bind(&req.name)
    .fetch_one(pool)
    .await?;

    Ok(tag)
}

pub async fn get_tag_by_id(pool: &PgPool, id: i32) -> AppResult<Tag> {
    sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Tag not found".to_string()))
}

pub async fn get_all_tags(pool: &PgPool) -> AppResult<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name")
        .fetch_all(pool)
        .await?;

    Ok(tags)
}

pub async fn update_tag(pool: &PgPool, id: i32, req: UpdateTagRequest) -> AppResult<Tag> {
    let tag = sqlx::query_as::<_, Tag>(
        r#"
        UPDATE tags
        SET name = $1
        WHERE id = $2
        RETURNING *
        "#
    )
    .bind(&req.name)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(tag)
}

pub async fn delete_tag(pool: &PgPool, id: i32) -> AppResult<()> {
    let result = sqlx::query("DELETE FROM tags WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Tag not found".to_string()));
    }

    Ok(())
}
