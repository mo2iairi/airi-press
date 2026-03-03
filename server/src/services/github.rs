use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::Config;
use crate::error::{AppError, AppResult};

#[derive(Debug, Serialize)]
struct CreateFileRequest {
    message: String,
    content: String,
    branch: String,
}

#[derive(Debug, Deserialize)]
struct CreateFileResponse {
    content: ContentInfo,
}

#[derive(Debug, Deserialize)]
struct ContentInfo {
    sha: String,
    download_url: String,
}

#[derive(Debug, Deserialize)]
struct FileInfo {
    sha: String,
}

#[derive(Debug, Serialize)]
struct DeleteFileRequest {
    message: String,
    sha: String,
    branch: String,
}

pub struct GithubService {
    client: Client,
    owner: String,
    repo: String,
    token: String,
    branch: String,
}

impl GithubService {
    pub fn new(config: &Config) -> Self {
        Self {
            client: Client::new(),
            owner: config.github_owner.clone(),
            repo: config.github_repo.clone(),
            token: config.github_token.clone(),
            branch: config.github_branch.clone(),
        }
    }

    pub async fn upload_file(&self, path: &str, content: &[u8]) -> AppResult<String> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );

        let encoded_content = STANDARD.encode(content);

        let request_body = CreateFileRequest {
            message: format!("Upload {}", path),
            content: encoded_content,
            branch: self.branch.clone(),
        };

        let response = self
            .client
            .put(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "AiriPress")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| AppError::GithubError(format!("Failed to upload file: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::GithubError(format!(
                "GitHub API error: {} - {}",
                status, text
            )));
        }

        let result: CreateFileResponse = response
            .json()
            .await
            .map_err(|e| AppError::GithubError(format!("Failed to parse response: {}", e)))?;

        Ok(result.content.download_url)
    }

    pub async fn delete_file(&self, path: &str) -> AppResult<()> {
        // First, get the file SHA
        let url = format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "AiriPress")
            .query(&[("ref", &self.branch)])
            .send()
            .await
            .map_err(|e| AppError::GithubError(format!("Failed to get file info: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            if status == reqwest::StatusCode::NOT_FOUND {
                return Ok(()); // File doesn't exist, consider it deleted
            }
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::GithubError(format!(
                "GitHub API error: {} - {}",
                status, text
            )));
        }

        let file_info: FileInfo = response
            .json()
            .await
            .map_err(|e| AppError::GithubError(format!("Failed to parse response: {}", e)))?;

        // Delete the file
        let delete_request = DeleteFileRequest {
            message: format!("Delete {}", path),
            sha: file_info.sha,
            branch: self.branch.clone(),
        };

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "AiriPress")
            .json(&delete_request)
            .send()
            .await
            .map_err(|e| AppError::GithubError(format!("Failed to delete file: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AppError::GithubError(format!(
                "GitHub API error: {} - {}",
                status, text
            )));
        }

        Ok(())
    }

    pub fn get_raw_url(&self, path: &str) -> String {
        format!(
            "https://raw.githubusercontent.com/{}/{}/{}/{}",
            self.owner, self.repo, self.branch, path
        )
    }
}
