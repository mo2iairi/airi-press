use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::comment::{Comment, CommentResponse, CreateCommentRequest, UpdateCommentRequest};
use crate::models::user::UserResponse;
use crate::services::user::get_user_by_id;

pub async fn create_comment(
    pool: &PgPool,
    post_id: i64,
    author_id: Uuid,
    req: CreateCommentRequest,
) -> AppResult<Comment> {
    let comment = sqlx::query_as::<_, Comment>(
        r#"
        INSERT INTO comments (post_id, author_id, content)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
    .bind(post_id)
    .bind(author_id)
    .bind(&req.content)
    .fetch_one(pool)
    .await?;

    Ok(comment)
}

pub async fn get_comment_by_id(pool: &PgPool, id: i64) -> AppResult<CommentResponse> {
    let comment = sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    let author = get_user_by_id(pool, comment.author_id).await?;

    Ok(CommentResponse {
        id: comment.id,
        post_id: comment.post_id,
        author: UserResponse::from(author),
        content: comment.content,
        created_at: comment.created_at,
    })
}

pub async fn get_comments_by_post(pool: &PgPool, post_id: i64) -> AppResult<Vec<CommentResponse>> {
    let comments = sqlx::query_as::<_, Comment>(
        "SELECT * FROM comments WHERE post_id = $1 ORDER BY created_at DESC"
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;

    let mut responses = Vec::new();
    for comment in comments {
        let author = get_user_by_id(pool, comment.author_id).await?;
        responses.push(CommentResponse {
            id: comment.id,
            post_id: comment.post_id,
            author: UserResponse::from(author),
            content: comment.content,
            created_at: comment.created_at,
        });
    }

    Ok(responses)
}

pub async fn update_comment(
    pool: &PgPool,
    id: i64,
    author_id: Uuid,
    is_admin: bool,
    req: UpdateCommentRequest,
) -> AppResult<Comment> {
    let existing = sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    if !is_admin && existing.author_id != author_id {
        return Err(AppError::Forbidden);
    }

    let comment = sqlx::query_as::<_, Comment>(
        r#"
        UPDATE comments
        SET content = $1
        WHERE id = $2
        RETURNING *
        "#
    )
    .bind(&req.content)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(comment)
}

pub async fn delete_comment(pool: &PgPool, id: i64, author_id: Uuid, is_admin: bool) -> AppResult<()> {
    let existing = sqlx::query_as::<_, Comment>("SELECT * FROM comments WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Comment not found".to_string()))?;

    if !is_admin && existing.author_id != author_id {
        return Err(AppError::Forbidden);
    }

    sqlx::query("DELETE FROM comments WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
