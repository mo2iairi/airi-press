use sqlx::PgPool;
use uuid::Uuid;

use crate::frontmatter::parse_frontmatter;
use crate::github::GitHubClient;

/// Scan the `posts/` directory on GitHub, parse frontmatter from each `.md` file,
/// and upsert metadata into PostgreSQL. Skips files without valid frontmatter.
pub async fn import_posts_from_github(pool: &PgPool, github: &GitHubClient) {
    tracing::info!("� Scanning GitHub for existing posts to import...");

    let items = match github.list_directory("posts").await {
        Ok(items) => items,
        Err(e) => {
            tracing::warn!("⚠️ Could not list posts directory on GitHub: {}", e);
            return;
        }
    };

    let md_files: Vec<_> = items
        .iter()
        .filter(|item| item.item_type == "file" && item.name.ends_with(".md"))
        .collect();

    if md_files.is_empty() {
        tracing::info!("ℹ️ No markdown files found in posts/ directory");
        return;
    }

    tracing::info!("📄 Found {} markdown file(s), checking for import...", md_files.len());

    let mut imported = 0u32;
    let mut skipped = 0u32;

    for item in md_files {
        let github_path = format!("posts/{}", item.name);

        // Download file content
        let raw_content = match github.get_file(&github_path).await {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("⚠️ Failed to fetch {}: {}", github_path, e);
                skipped += 1;
                continue;
            }
        };

        // Parse frontmatter
        let parsed = match parse_frontmatter(&raw_content) {
            Some(p) => p,
            None => {
                tracing::debug!("⏭️ Skipping {} (no valid frontmatter)", github_path);
                skipped += 1;
                continue;
            }
        };

        let meta = &parsed.meta;

        // Check if post already exists in DB
        let exists = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM posts WHERE id = $1",
        )
        .bind(meta.id)
        .fetch_one(pool)
        .await;

        match exists {
            Ok(count) if count > 0 => {
                // Already in DB — update metadata
                let category_id = resolve_category(pool, &meta.category).await;

                let _ = sqlx::query(
                    r#"UPDATE posts SET title = $1, summary = $2, author_name = $3,
                       category_id = $4, published = $5, updated_at = $6
                       WHERE id = $7"#,
                )
                .bind(&meta.title)
                .bind(&meta.summary)
                .bind(&meta.author)
                .bind(category_id)
                .bind(meta.published)
                .bind(meta.updated_at)
                .bind(meta.id)
                .execute(pool)
                .await;

                // Sync tags
                sync_tags(pool, meta.id, &meta.tags).await;

                tracing::debug!("� Updated existing post: {} ({})", meta.title, meta.id);
                imported += 1;
            }
            _ => {
                // New post — need an author_id; look up by name or use the first admin
                let author_id = resolve_author(pool, &meta.author).await;
                let category_id = resolve_category(pool, &meta.category).await;

                let result = sqlx::query(
                    r#"INSERT INTO posts (id, title, summary, author_id, author_name, category_id, published, github_path, created_at, updated_at)
                       VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                       ON CONFLICT (id) DO NOTHING"#,
                )
                .bind(meta.id)
                .bind(&meta.title)
                .bind(&meta.summary)
                .bind(author_id)
                .bind(&meta.author)
                .bind(category_id)
                .bind(meta.published)
                .bind(&github_path)
                .bind(meta.created_at)
                .bind(meta.updated_at)
                .execute(pool)
                .await;

                match result {
                    Ok(_) => {
                        // Sync tags
                        sync_tags(pool, meta.id, &meta.tags).await;
                        tracing::info!("✅ Imported post: {} ({})", meta.title, meta.id);
                        imported += 1;
                    }
                    Err(e) => {
                        tracing::warn!("⚠️ Failed to import {}: {}", meta.title, e);
                        skipped += 1;
                    }
                }
            }
        }
    }

    tracing::info!(
        "📥 Import complete: {} imported/updated, {} skipped",
        imported,
        skipped
    );
}

/// Resolve an author username to a user ID. Falls back to the first admin (permission=255).
async fn resolve_author(pool: &PgPool, username: &str) -> Uuid {
    // Try exact match
    if let Ok(Some(id)) =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE username = $1")
            .bind(username)
            .fetch_optional(pool)
            .await
    {
        return id;
    }

    // Fallback: first admin user
    if let Ok(Some(id)) =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE permission = 255 LIMIT 1")
            .fetch_optional(pool)
            .await
    {
        return id;
    }

    // Last resort: any user
    sqlx::query_scalar::<_, Uuid>("SELECT id FROM users LIMIT 1")
        .fetch_one(pool)
        .await
        .expect("No users exist in the database; cannot import posts")
}

/// Resolve an optional category name to a category ID, creating it if needed.
async fn resolve_category(pool: &PgPool, category: &Option<String>) -> Option<Uuid> {
    let name = category.as_deref()?.trim();
    if name.is_empty() {
        return None;
    }

    // Try to find existing
    if let Ok(Some(id)) =
        sqlx::query_scalar::<_, Uuid>("SELECT id FROM categories WHERE name = $1")
            .bind(name)
            .fetch_optional(pool)
            .await
    {
        return Some(id);
    }

    // Auto-create
    match sqlx::query_scalar::<_, Uuid>(
        "INSERT INTO categories (name) VALUES ($1) RETURNING id",
    )
    .bind(name)
    .fetch_one(pool)
    .await
    {
        Ok(id) => {
            tracing::info!("📁 Auto-created category: {}", name);
            Some(id)
        }
        Err(_) => None,
    }
}

/// Sync tags for a post — resolves tag names, creates missing tags, and updates the junction table.
async fn sync_tags(pool: &PgPool, post_id: Uuid, tag_names: &[String]) {
    // Clear existing associations
    let _ = sqlx::query("DELETE FROM post_tags WHERE post_id = $1")
        .bind(post_id)
        .execute(pool)
        .await;

    for tag_name in tag_names {
        let trimmed = tag_name.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Find or create tag
        let tag_id = match sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM tags WHERE name = $1",
        )
        .bind(trimmed)
        .fetch_optional(pool)
        .await
        {
            Ok(Some(id)) => id,
            _ => {
                match sqlx::query_scalar::<_, Uuid>(
                    "INSERT INTO tags (name) VALUES ($1) RETURNING id",
                )
                .bind(trimmed)
                .fetch_one(pool)
                .await
                {
                    Ok(id) => {
                        tracing::info!("🏷️ Auto-created tag: {}", trimmed);
                        id
                    }
                    Err(_) => continue,
                }
            }
        };

        let _ = sqlx::query(
            "INSERT INTO post_tags (post_id, tag_id) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        )
        .bind(post_id)
        .bind(tag_id)
        .execute(pool)
        .await;
    }
}