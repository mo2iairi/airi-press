use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// Permission levels for users
/// 0: Viewer (can only view published posts)
/// 1: Commenter (can view and comment)
/// 2: Author (can create/edit own posts)
/// 3: Editor (can edit any post)
/// 4: Admin (full access)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Permission(pub i16);

impl Permission {
    pub const VIEWER: Self = Self(0);
    pub const COMMENTER: Self = Self(1);
    pub const AUTHOR: Self = Self(2);
    pub const EDITOR: Self = Self(3);
    pub const ADMIN: Self = Self(4);

    pub fn can_comment(&self) -> bool {
        self.0 >= Self::COMMENTER.0
    }

    pub fn can_create_post(&self) -> bool {
        self.0 >= Self::AUTHOR.0
    }

    pub fn can_edit_any_post(&self) -> bool {
        self.0 >= Self::EDITOR.0
    }

    pub fn is_admin(&self) -> bool {
        self.0 >= Self::ADMIN.0
    }
}

impl From<i16> for Permission {
    fn from(value: i16) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub permission: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub permission: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            permission: user.permission,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 255, message = "Username must be between 3 and 255 characters"))]
    pub username: String,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,
    pub permission: Option<i16>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 255, message = "Username must be between 3 and 255 characters"))]
    pub username: Option<String>,
    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: Option<String>,
    pub permission: Option<i16>,
}
