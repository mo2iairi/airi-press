use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::category::Category;
use crate::models::post::{
    CreatePostRequest, PaginatedResponse, Post, PostListItem, PostQuery, PostResponse, UpdatePostRequest,
};
use crate::models::tag::Tag;
use crate::models::user::UserResponse;
use crate::services::user::get_user_by_id;

pub async fn create_post(pool: &PgPool, author_id: Uuid, req: CreatePostRequest) -> AppResult<Post> {
    let mut tx = pool.begin().await?;

    let post = sqlx::query_as::<_, Post>(
        r#"
        INSERT INTO posts (title, summary, content, author_id, published)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(&req.title)
    .bind(&req.summary)
    .bind(&req.content)
    .bind(author_id)
    .bind(req.published.unwrap_or(false))
    .fetch_one(&mut *tx)
    .await?;

    // Add categories
    if let Some(category_ids) = &req.category_ids {
        for category_id in category_ids {
            sqlx::query("INSERT INTO posts_categories (post_id, category_id) VALUES ($1, $2)")
                .bind(post.id)
                .bind(category_id)
                .execute(&mut *tx)
                .await?;
        }
    }

    // Add tags
    if let Some(tag_ids) = &req.tag_ids {
        for tag_id in tag_ids {
            sqlx::query("INSERT INTO posts_tags (post_id, tag_id) VALUES ($1, $2)")
                .bind(post.id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await?;
        }
    }

    tx.commit().await?;

    Ok(post)
}

pub async fn get_post_by_id(pool: &PgPool, id: i64) -> AppResult<PostResponse> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    let author = get_user_by_id(pool, post.author_id).await?;
    let categories = get_post_categories(pool, post.id).await?;
    let tags = get_post_tags(pool, post.id).await?;

    Ok(PostResponse {
        id: post.id,
        title: post.title,
        summary: post.summary,
        content: post.content,
        author: UserResponse::from(author),
        published: post.published,
        categories,
        tags,
        created_at: post.created_at,
        updated_at: post.updated_at,
    })
}

pub async fn get_posts(pool: &PgPool, query: PostQuery) -> AppResult<PaginatedResponse<PostListItem>> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(10).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let mut sql = String::from("SELECT p.* FROM posts p");
    let mut count_sql = String::from("SELECT COUNT(DISTINCT p.id) FROM posts p");
    let mut where_clauses = Vec::new();
    let mut joins = Vec::new();

    if query.category_id.is_some() {
        joins.push("JOIN posts_categories pc ON p.id = pc.post_id");
    }
    if query.tag_id.is_some() {
        joins.push("JOIN posts_tags pt ON p.id = pt.post_id");
    }

    if let Some(published) = query.published {
        where_clauses.push(format!("p.published = {}", published));
    }
    if let Some(author_id) = query.author_id {
        where_clauses.push(format!("p.author_id = '{}'", author_id));
    }
    if let Some(category_id) = query.category_id {
        where_clauses.push(format!("pc.category_id = {}", category_id));
    }
    if let Some(tag_id) = query.tag_id {
        where_clauses.push(format!("pt.tag_id = {}", tag_id));
    }
    if let Some(search) = &query.search {
        where_clauses.push(format!(
            "(p.title ILIKE '%{}%' OR p.summary ILIKE '%{}%')",
            search.replace("'", "''"),
            search.replace("'", "''")
        ));
    }

    for join in &joins {
        sql.push_str(&format!(" {}", join));
        count_sql.push_str(&format!(" {}", join));
    }

    if !where_clauses.is_empty() {
        let where_clause = format!(" WHERE {}", where_clauses.join(" AND "));
        sql.push_str(&where_clause);
        count_sql.push_str(&where_clause);
    }

    sql.push_str(" ORDER BY p.created_at DESC");
    sql.push_str(&format!(" LIMIT {} OFFSET {}", per_page, offset));

    let posts: Vec<Post> = sqlx::query_as(&sql).fetch_all(pool).await?;

    let total: (i64,) = sqlx::query_as(&count_sql).fetch_one(pool).await?;

    let mut items = Vec::new();
    for post in posts {
        let author = get_user_by_id(pool, post.author_id).await?;
        let categories = get_post_categories(pool, post.id).await?;
        let tags = get_post_tags(pool, post.id).await?;

        items.push(PostListItem {
            id: post.id,
            title: post.title,
            summary: post.summary,
            author: UserResponse::from(author),
            published: post.published,
            categories,
            tags,
            created_at: post.created_at,
            updated_at: post.updated_at,
        });
    }

    Ok(PaginatedResponse::new(items, total.0, page, per_page))
}

pub async fn update_post(
    pool: &PgPool,
    id: i64,
    author_id: Uuid,
    can_edit_any: bool,
    req: UpdatePostRequest,
) -> AppResult<Post> {
    let existing = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    // Check permission
    if !can_edit_any && existing.author_id != author_id {
        return Err(AppError::Forbidden);
    }

    let mut tx = pool.begin().await?;

    let title = req.title.unwrap_or(existing.title);
    let summary = req.summary.or(existing.summary);
    let content = req.content.unwrap_or(existing.content);
    let published = req.published.unwrap_or(existing.published);

    let post = sqlx::query_as::<_, Post>(
        r#"
        UPDATE posts
        SET title = $1, summary = $2, content = $3, published = $4, updated_at = NOW()
        WHERE id = $5
        RETURNING *
        "#
    )
    .bind(&title)
    .bind(&summary)
    .bind(&content)
    .bind(published)
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;

    // Update categories
    if let Some(category_ids) = &req.category_ids {
        sqlx::query("DELETE FROM posts_categories WHERE post_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        for category_id in category_ids {
            sqlx::query("INSERT INTO posts_categories (post_id, category_id) VALUES ($1, $2)")
                .bind(id)
                .bind(category_id)
                .execute(&mut *tx)
                .await?;
        }
    }

    // Update tags
    if let Some(tag_ids) = &req.tag_ids {
        sqlx::query("DELETE FROM posts_tags WHERE post_id = $1")
            .bind(id)
            .execute(&mut *tx)
            .await?;

        for tag_id in tag_ids {
            sqlx::query("INSERT INTO posts_tags (post_id, tag_id) VALUES ($1, $2)")
                .bind(id)
                .bind(tag_id)
                .execute(&mut *tx)
                .await?;
        }
    }

    tx.commit().await?;

    Ok(post)
}

pub async fn delete_post(pool: &PgPool, id: i64, author_id: Uuid, can_edit_any: bool) -> AppResult<()> {
    let existing = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    if !can_edit_any && existing.author_id != author_id {
        return Err(AppError::Forbidden);
    }

    sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

async fn get_post_categories(pool: &PgPool, post_id: i64) -> AppResult<Vec<Category>> {
    let categories = sqlx::query_as::<_, Category>(
        r#"
        SELECT c.* FROM categories c
        JOIN posts_categories pc ON c.id = pc.category_id
        WHERE pc.post_id = $1
        "#
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;

    Ok(categories)
}

async fn get_post_tags(pool: &PgPool, post_id: i64) -> AppResult<Vec<Tag>> {
    let tags = sqlx::query_as::<_, Tag>(
        r#"
        SELECT t.* FROM tags t
        JOIN posts_tags pt ON t.id = pt.tag_id
        WHERE pt.post_id = $1
        "#
    )
    .bind(post_id)
    .fetch_all(pool)
    .await?;

    Ok(tags)
}
