use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers;
use crate::models::AppState;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        // Health check
        .route("/health", get(handlers::health::health_check))
        .route("/health", post(handlers::health::health_check))
        // Auth
        .route("/auth/login", post(handlers::auth::login_handler))
        // Users
        .route("/users/me", get(handlers::users::get_current_user))
        .route("/users", get(handlers::users::get_all_users))
        .route("/users", post(handlers::users::create_user))
        .route("/users/:id", get(handlers::users::get_user))
        .route("/users/:id", put(handlers::users::update_user))
        .route("/users/:id", delete(handlers::users::delete_user))
        // Posts
        .route("/posts", get(handlers::posts::get_posts))
        .route("/posts", post(handlers::posts::create_post))
        .route("/posts/:id", get(handlers::posts::get_post))
        .route("/posts/:id", put(handlers::posts::update_post))
        .route("/posts/:id", delete(handlers::posts::delete_post))
        // Categories
        .route("/categories", get(handlers::categories::get_all_categories))
        .route("/categories/tree", get(handlers::categories::get_categories_tree))
        .route("/categories", post(handlers::categories::create_category))
        .route("/categories/:id", get(handlers::categories::get_category))
        .route("/categories/:id", put(handlers::categories::update_category))
        .route("/categories/:id", delete(handlers::categories::delete_category))
        // Tags
        .route("/tags", get(handlers::tags::get_all_tags))
        .route("/tags", post(handlers::tags::create_tag))
        .route("/tags/:id", get(handlers::tags::get_tag))
        .route("/tags/:id", put(handlers::tags::update_tag))
        .route("/tags/:id", delete(handlers::tags::delete_tag))
        // Comments
        .route("/posts/:post_id/comments", get(handlers::comments::get_comments_by_post))
        .route("/posts/:post_id/comments", post(handlers::comments::create_comment))
        .route("/comments/:id", get(handlers::comments::get_comment))
        .route("/comments/:id", put(handlers::comments::update_comment))
        .route("/comments/:id", delete(handlers::comments::delete_comment))
        // Images
        .route("/images", get(handlers::images::get_all_images))
        .route("/images", post(handlers::images::upload_image))
        .route("/images/:id", get(handlers::images::get_image))
        .route("/images/:id", delete(handlers::images::delete_image))
}
