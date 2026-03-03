use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Image {
    pub id: i64,
    pub relative_path: String,
    pub original_name: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageResponse {
    pub id: i64,
    pub relative_path: String,
    pub url: String,
    pub original_name: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadImageResponse {
    pub id: i64,
    pub url: String,
    pub relative_path: String,
}
