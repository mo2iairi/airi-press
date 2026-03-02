use std::sync::Arc;

use axum::{
    extract::{Extension, Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::errors::AppError;
use crate::frontmatter::{self, Frontmatter};
use crate::models::*;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    pub all: Option<bool>,
}

pub async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ListPostsQuery>,
) -> Result<Json<ApiResponse<Vec<PostResponse>>>, AppError> {
    let show_all = query.all.unwrap_or(false);

    let posts = if show_all {
        sqlx::query_as::<_, Post>("SELECT * FROM posts ORDER BY created_at DESC")
            .fetch_all(&state.db)
            .await?
    } else {
        sqlx::query_as::<_, Post>(
            "SELECT * FROM posts WHERE published = true ORDER BY created_at DESC",
        )
        .fetch_all(&state.db)
        .await?
    };

    let mut responses = Vec::new();
    for post in posts {
        // For list view, we don't fetch content from GitHub (efficiency)
        responses.push(build_post_response(&state, post, None, false).await?);
    }

    Ok(Json(ApiResponse::success(responses)))
}

pub async fn get_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<PostResponse>>, AppError> {
    let post = get_post_by_id(&state, id).await?;
    // For detail view, we fetch content if available
    let response = build_post_response(&state, post, None, true).await?;
    Ok(Json(ApiResponse::success(response)))
}

pub async fn create_post(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreatePostRequest>,
) -> Result<Json<ApiResponse<PostResponse>>, AppError> {
    let post_id = Uuid::new_v4();
    let github_path = format!("posts/{}.md", post_id);
    let published = payload.published.unwrap_or(false);
    let now = chrono::Utc::now();
    let author_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Internal("Invalid user ID".to_string()))?;

    // 1. Resolve names for Frontmatter
    let category_name = resolve_category_name(&state, payload.category_id).await?;
    let tag_names = resolve_tag_names(&state, payload.tag_ids.as_ref()).await?;

    // 2. Prepare and Upload to GitHub
    let meta = Frontmatter {
        id: post_id,
        title: payload.title.clone(),
        summary: payload.summary.clone(),
        author: claims.username.clone(),
        category: category_name,
        tags: tag_names,
        published,
        created_at: now,
        updated_at: now,
    };
    
    let full_content = frontmatter::render_with_frontmatter(&meta, &payload.content);
    state
        .github
        .put_file(
            &github_path,
            &full_content,
            &format!("Create post: {}", payload.title),
        )
        .await
        .map_err(AppError::GitHub)?;

    // 3. Insert into Database
    let post = sqlx::query_as::<_, Post>(
        r#"INSERT INTO posts (id, title, summary, author_id, author_name, category_id, published, github_path, created_at, updated_at)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#,
    )
    .bind(post_id)
    .bind(&payload.title)
    .bind(&payload.summary)
    .bind(author_id)
    .bind(&claims.username)
    .bind(payload.category_id)
    .bind(published)
    .bind(&github_path)
    .bind(now)
    .bind(now)
    .fetch_one(&state.db)
    .await?;

    // 4. Update Tags Relation
    update_post_tags(&state, post_id, payload.tag_ids.as_ref()).await?;

    // 5. Build Response (use the content we just uploaded)
    let response = build_post_response(&state, post, Some(payload.content), false).await?;
    Ok(Json(ApiResponse::success(response)))
}

pub async fn update_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostRequest>,
) -> Result<Json<ApiResponse<PostResponse>>, AppError> {
    let existing = get_post_by_id(&state, id).await?;

    let title = payload.title.unwrap_or(existing.title);
    let summary = payload.summary.unwrap_or(existing.summary);
    let category_id = payload.category_id.or(existing.category_id);
    let published = payload.published.unwrap_or(existing.published);
    let now = chrono::Utc::now();

    // 1. Resolve names for Frontmatter
    let category_name = resolve_category_name(&state, category_id).await?;
    
    // Tag names: if provided in payload, resolve them; otherwise fetch existing from DB
    let tag_names = if let Some(tids) = &payload.tag_ids {
        resolve_tag_names(&state, Some(tids)).await?
    } else {
         sqlx::query_scalar::<_, String>(
            "SELECT t.name FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = $1"
        )
        .bind(id)
        .fetch_all(&state.db)
        .await?
    };

    // 2. Determine Content
    // If content is updated, use it. Else fetch from GitHub (and strip frontmatter)
    let body_content = if let Some(c) = &payload.content {
        c.clone()
    } else {
        fetch_github_content(&state, &existing.github_path).await?.unwrap_or_default()
    };

    // 3. Upload to GitHub with updated Frontmatter
    let meta = Frontmatter {
        id,
        title: title.clone(),
        summary: summary.clone(),
        author: existing.author_name.clone(),
        category: category_name,
        tags: tag_names,
        published,
        created_at: existing.created_at,
        updated_at: now,
    };

    let full_content = frontmatter::render_with_frontmatter(&meta, &body_content);
    state
        .github
        .put_file(
            &existing.github_path,
            &full_content,
            &format!("Update post: {}", title),
        )
        .await
        .map_err(AppError::GitHub)?;

    // 4. Update Database
    let post = sqlx::query_as::<_, Post>(
        r#"UPDATE posts SET title = $1, summary = $2, category_id = $3, published = $4, updated_at = $5
           WHERE id = $6 RETURNING *"#,
    )
    .bind(&title)
    .bind(&summary)
    .bind(category_id)
    .bind(published)
    .bind(now)
    .bind(id)
    .fetch_one(&state.db)
    .await?;

    // 5. Update Tags Relation (if provided)
    if let Some(tag_ids) = &payload.tag_ids {
        sqlx::query("DELETE FROM post_tags WHERE post_id = $1")
            .bind(id)
            .execute(&state.db)
            .await?;
        update_post_tags(&state, id, Some(tag_ids)).await?;
    }

    // 6. Build Response
    let response = build_post_response(&state, post, Some(body_content), false).await?;
    Ok(Json(ApiResponse::success(response)))
}

pub async fn delete_post(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    tracing::info!("Request to delete post id: {}", id);
    
    let post = get_post_by_id(&state, id).await?;
    tracing::info!("Found post: {} (github: {})", post.title, post.github_path);

    // Attempt to delete from GitHub (log error but allow DB delete to proceed)
    match state
        .github
        .delete_file(&post.github_path, &format!("Delete post: {}", post.title))
        .await 
    {
        Ok(_) => tracing::info!("Deleted file from GitHub"),
        Err(e) => tracing::error!("Failed to delete file from GitHub (proceeding with DB delete): {}", e),
    }

    // Delete from DB (cascades to post_tags)
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;
        
    if result.rows_affected() == 0 {
         // Should satisfy race condition where post was deleted between get and delete
         tracing::warn!("Post ID {} was not found during DELETE execution", id);
         return Err(AppError::NotFound("Post not found".to_string()));
    }

    Ok(Json(ApiResponse::success(())))
}

// ==========================================
// Helper Functions
// ==========================================

async fn get_post_by_id(state: &AppState, id: Uuid) -> Result<Post, AppError> {
    sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))
}

async fn resolve_category_name(state: &AppState, category_id: Option<Uuid>) -> Result<Option<String>, AppError> {
    if let Some(id) = category_id {
        sqlx::query_scalar("SELECT name FROM categories WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await
            .map_err(AppError::Database)
    } else {
        Ok(None)
    }
}

async fn resolve_tag_names(state: &AppState, tag_ids: Option<&Vec<Uuid>>) -> Result<Vec<String>, AppError> {
    if let Some(ids) = tag_ids {
        if ids.is_empty() {
             return Ok(vec![]);
        }
        let mut names = Vec::new();
        for tid in ids {
             if let Some(name) = sqlx::query_scalar::<_, String>("SELECT name FROM tags WHERE id = $1")
                .bind(tid)
                .fetch_optional(&state.db)
                .await? 
            {
                names.push(name);
            }
        }
        Ok(names)
    } else {
        Ok(vec![])
    }
}

async fn update_post_tags(state: &AppState, post_id: Uuid, tag_ids: Option<&Vec<Uuid>>) -> Result<(), AppError> {
    if let Some(ids) = tag_ids {
        for tag_id in ids {
            sqlx::query("INSERT INTO post_tags (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING")
                .bind(post_id)
                .bind(tag_id)
                .execute(&state.db)
                .await?;
        }
    }
    Ok(())
}

async fn fetch_github_content(state: &AppState, path: &str) -> Result<Option<String>, AppError> {
     match state.github.get_file(path).await {
        Ok(raw) => {
            if let Some(parsed) = frontmatter::parse_frontmatter(&raw) {
                Ok(Some(parsed.content))
            } else {
                Ok(Some(raw))
            }
        }
        Err(_) => Ok(None),
    }
}

async fn build_post_response(
    state: &AppState, 
    post: Post, 
    existing_content: Option<String>,
    fetch_if_missing: bool
) -> Result<PostResponse, AppError> {
    let category = if let Some(cat_id) = post.category_id {
        sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
            .bind(cat_id)
            .fetch_optional(&state.db)
            .await?
    } else {
        None
    };

    let tags = sqlx::query_as::<_, Tag>(
        "SELECT t.* FROM tags t JOIN post_tags pt ON t.id = pt.tag_id WHERE pt.post_id = $1",
    )
    .bind(post.id)
    .fetch_all(&state.db)
    .await?;

    let content = if existing_content.is_some() {
        existing_content
    } else if fetch_if_missing {
        fetch_github_content(state, &post.github_path).await?
    } else {
        None
    };

    Ok(PostResponse {
        id: post.id,
        title: post.title,
        summary: post.summary,
        author_id: post.author_id,
        author_name: post.author_name,
        category,
        tags,
        published: post.published,
        content,
        created_at: post.created_at,
        updated_at: post.updated_at,
    })
}