use crate::config::Config;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &PgPool) {
    let statements = vec![
        r#"CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            username VARCHAR(255) UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            permission INT NOT NULL DEFAULT 0,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
        r#"CREATE TABLE IF NOT EXISTS categories (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) UNIQUE NOT NULL,
            description TEXT NOT NULL DEFAULT '',
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
        r#"CREATE TABLE IF NOT EXISTS tags (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            name VARCHAR(255) UNIQUE NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
        r#"CREATE TABLE IF NOT EXISTS posts (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            title VARCHAR(500) NOT NULL,
            summary TEXT NOT NULL DEFAULT '',
            author_id UUID NOT NULL REFERENCES users(id),
            author_name VARCHAR(255) NOT NULL,
            category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
            published BOOLEAN NOT NULL DEFAULT false,
            github_path TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )"#,
        r#"CREATE TABLE IF NOT EXISTS post_tags (
            post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
            tag_id UUID NOT NULL REFERENCES tags(id) ON DELETE CASCADE,
            PRIMARY KEY (post_id, tag_id)
        )"#,
    ];

    for statement in statements {
        sqlx::query(statement)
            .execute(pool)
            .await
            .expect("Failed to run migration");
    }

    tracing::info!("✅ Database migrations completed");
}

pub async fn init_admin(pool: &PgPool, config: &Config) {
    let existing = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM users WHERE username = $1"
    )
    .bind(&config.admin_username)
    .fetch_optional(pool)
    .await
    .expect("Failed to check admin user");

    if existing.is_none() {
        let password_hash = bcrypt::hash(&config.admin_password, bcrypt::DEFAULT_COST)
            .expect("Failed to hash admin password");

        sqlx::query(
            "INSERT INTO users (username, password_hash, permission) VALUES ($1, $2, 255)"
        )
        .bind(&config.admin_username)
        .bind(&password_hash)
        .execute(pool)
        .await
        .expect("Failed to create admin user");

        tracing::info!("✅ Admin account '{}' created", config.admin_username);
    } else {
        tracing::info!("ℹ️ Admin account '{}' already exists", config.admin_username);
    }
}
