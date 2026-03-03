pub mod user;
pub mod post;
pub mod category;
pub mod tag;
pub mod comment;
pub mod image;
pub mod auth;

use crate::config::Config;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
}
