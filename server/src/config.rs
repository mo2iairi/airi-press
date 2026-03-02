#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub github_owner: String,
    pub github_repo: String,
    pub github_token: String,
    pub github_branch: String,
    pub jwt_secret: String,
    pub admin_username: String,
    pub admin_password: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            github_owner: std::env::var("GITHUB_OWNER").expect("GITHUB_OWNER must be set"),
            github_repo: std::env::var("GITHUB_REPO").expect("GITHUB_REPO must be set"),
            github_token: std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set"),
            github_branch: std::env::var("GITHUB_BRANCH").unwrap_or_else(|_| "main".to_string()),
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            admin_username: std::env::var("ADMIN_USERNAME").unwrap_or_else(|_| "admin".to_string()),
            admin_password: std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set"),
        }
    }
}
