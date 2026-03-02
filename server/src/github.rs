use base64::Engine;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct GitHubClient {
    client: reqwest::Client,
    token: String,
    owner: String,
    repo: String,
    branch: String,
}

#[derive(Debug, Serialize)]
struct CreateOrUpdateFileRequest {
    message: String,
    content: String,
    branch: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GitHubFileResponse {
    sha: String,
    content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubTreeItem {
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
struct DeleteFileRequest {
    message: String,
    sha: String,
    branch: String,
}

impl GitHubClient {
    pub fn new(token: &str, owner: &str, repo: &str, branch: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            token: token.to_string(),
            owner: owner.to_string(),
            repo: repo.to_string(),
            branch: branch.to_string(),
        }
    }

    fn api_url(&self, path: &str) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/contents/{}",
            self.owner, self.repo, path
        )
    }

    /// Create or update a file in the GitHub repository
    pub async fn put_file(&self, path: &str, content: &str, message: &str) -> Result<(), String> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(content.as_bytes());

        // Try to get existing file SHA
        let sha = self.get_file_sha(path).await.ok();

        let body = CreateOrUpdateFileRequest {
            message: message.to_string(),
            content: encoded,
            branch: self.branch.clone(),
            sha,
        };

        let response = self
            .client
            .put(&self.api_url(path))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, "airi-press")
            .header(ACCEPT, "application/vnd.github+json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("GitHub API request failed: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(format!("GitHub API error ({}): {}", status, text))
        }
    }

    /// Get file content from the GitHub repository
    pub async fn get_file(&self, path: &str) -> Result<String, String> {
        let response = self
            .client
            .get(&self.api_url(path))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, "airi-press")
            .header(ACCEPT, "application/vnd.github+json")
            .query(&[("ref", &self.branch)])
            .send()
            .await
            .map_err(|e| format!("GitHub API request failed: {}", e))?;

        if !response.status().is_success() {
            return Err(format!("File not found: {}", path));
        }

        let file_info: GitHubFileResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse GitHub response: {}", e))?;

        let content = file_info.content.unwrap_or_default();
        let cleaned = content.replace('\n', "").replace('\r', "");
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(&cleaned)
            .map_err(|e| format!("Failed to decode content: {}", e))?;

        String::from_utf8(decoded).map_err(|e| format!("Invalid UTF-8: {}", e))
    }

    /// Get file SHA (needed for updates and deletes)
    async fn get_file_sha(&self, path: &str) -> Result<String, String> {
        let response = self
            .client
            .get(&self.api_url(path))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, "airi-press")
            .header(ACCEPT, "application/vnd.github+json")
            .query(&[("ref", &self.branch)])
            .send()
            .await
            .map_err(|e| format!("GitHub API request failed: {}", e))?;

        if !response.status().is_success() {
            return Err("File not found".to_string());
        }

        let file_info: GitHubFileResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        Ok(file_info.sha)
    }

    /// List files in a directory of the GitHub repository
    pub async fn list_directory(&self, path: &str) -> Result<Vec<GitHubTreeItem>, String> {
        let response = self
            .client
            .get(&self.api_url(path))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, "airi-press")
            .header(ACCEPT, "application/vnd.github+json")
            .query(&[("ref", &self.branch)])
            .send()
            .await
            .map_err(|e| format!("GitHub API request failed: {}", e))?;

        if !response.status().is_success() {
            // Directory may not exist yet, return empty
            return Ok(vec![]);
        }

        let items: Vec<GitHubTreeItem> = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse directory listing: {}", e))?;

        Ok(items)
    }

    /// Delete a file from the GitHub repository
    pub async fn delete_file(&self, path: &str, message: &str) -> Result<(), String> {
        let sha = self.get_file_sha(path).await?;

        let body = DeleteFileRequest {
            message: message.to_string(),
            sha,
            branch: self.branch.clone(),
        };

        let response = self
            .client
            .delete(&self.api_url(path))
            .header(AUTHORIZATION, format!("Bearer {}", self.token))
            .header(USER_AGENT, "airi-press")
            .header(ACCEPT, "application/vnd.github+json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("GitHub API request failed: {}", e))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            Err(format!("GitHub API error ({}): {}", status, text))
        }
    }
}
