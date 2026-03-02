use std::sync::Arc;

use axum::{
    middleware as axum_middleware,
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers;
use crate::middleware::require_auth;
use crate::AppState;

pub fn create_router(state: Arc<AppState>) -> Router {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/api/v1/health", get(handlers::health::health_check))
        .route("/api/v1/auth/login", post(handlers::auth::login))
        .route("/api/v1/auth/register", post(handlers::auth::register))
        // Public read-only
        .route("/api/v1/posts", get(handlers::posts::list_posts))
        .route("/api/v1/posts/:id", get(handlers::posts::get_post))
        .route("/api/v1/categories", get(handlers::categories::list_categories))
        .route("/api/v1/categories/:id", get(handlers::categories::get_category))
        .route("/api/v1/tags", get(handlers::tags::list_tags))
        .route("/api/v1/tags/:id", get(handlers::tags::get_tag));

    // All authenticated routes share a single require_auth layer
    // Permission checks (author/admin) are done inside the handlers
    let protected_routes = Router::new()
        // Auth
        .route("/api/v1/auth/me", get(handlers::auth::me))
        // Manage posts
        .route(
            "/api/v1/manage/posts",
            get(handlers::posts::list_posts)
                .post(handlers::posts::create_post),
        )
        .route(
            "/api/v1/manage/posts/:id",
            get(handlers::posts::get_post)
                .put(handlers::posts::update_post)
                .delete(handlers::posts::delete_post),
        )
        // Manage categories
        .route(
            "/api/v1/manage/categories",
            get(handlers::categories::list_categories)
                .post(handlers::categories::create_category),
        )
        .route(
            "/api/v1/manage/categories/:id",
            get(handlers::categories::get_category)
                .put(handlers::categories::update_category)
                .delete(handlers::categories::delete_category),
        )
        // Manage tags
        .route(
            "/api/v1/manage/tags",
            get(handlers::tags::list_tags)
                .post(handlers::tags::create_tag),
        )
        .route(
            "/api/v1/manage/tags/:id",
            get(handlers::tags::get_tag)
                .put(handlers::tags::update_tag)
                .delete(handlers::tags::delete_tag),
        )
        // Admin: users
        .route("/api/v1/users", get(handlers::users::list_users))
        .route("/api/v1/users/:id/permission", put(handlers::users::update_permission))
        .route("/api/v1/users/:id", delete(handlers::users::delete_user))
        .layer(axum_middleware::from_fn(require_auth));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(state)
}