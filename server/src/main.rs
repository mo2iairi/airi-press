mod config;
mod db;
mod errors;
mod frontmatter;
mod github;
mod handlers;
mod middleware;
mod models;
mod routes;
mod sync;

use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub struct AppState {
    pub db: sqlx::PgPool,
    pub config: config::Config,
    pub github: github::GitHubClient,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "airi_press_server=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = config::Config::from_env();
    let pool = db::create_pool(&config.database_url).await;
    db::run_migrations(&pool).await;

    let github = github::GitHubClient::new(
        &config.github_token,
        &config.github_owner,
        &config.github_repo,
        &config.github_branch,
    );

    // Initialize admin account
    db::init_admin(&pool, &config).await;

    // Sync posts from GitHub → PostgreSQL
    sync::import_posts_from_github(&pool, &github).await;

    let state = Arc::new(AppState {
        db: pool,
        config,
        github,
    });

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = routes::create_router(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    tracing::info!("🚀 Airi Press server running on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
