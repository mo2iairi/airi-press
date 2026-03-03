use anyhow::Result;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub admin_username: String,
    pub admin_password: String,
    pub server_port: u16,
    pub github_owner: String,
    pub github_repo: String,
    pub github_token: String,
    pub github_branch: String,
    pub web_domain: Option<String>,
    pub server_url: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
            admin_username: env::var("ADMIN_USERNAME")
                .unwrap_or_else(|_| "admin".to_string()),
            admin_password: env::var("ADMIN_PASSWORD")
                .expect("ADMIN_PASSWORD must be set"),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .expect("SERVER_PORT must be a valid port number"),
            github_owner: env::var("GITHUB_OWNER")
                .expect("GITHUB_OWNER must be set"),
            github_repo: env::var("GITHUB_REPO")
                .expect("GITHUB_REPO must be set"),
            github_token: env::var("GITHUB_TOKEN")
                .expect("GITHUB_TOKEN must be set"),
            github_branch: env::var("GITHUB_BRANCH")
                .unwrap_or_else(|_| "main".to_string()),
            web_domain: env::var("WEB_DOMAIN").ok(),
            server_url: env::var("SERVER_URL").ok(),
        })
    }
}
