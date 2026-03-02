use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Frontmatter metadata embedded in Markdown files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Frontmatter {
    pub id: Uuid,
    pub title: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub published: bool,
    #[serde(default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    #[serde(default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
}

/// Result of parsing a Markdown file with frontmatter
#[derive(Debug)]
pub struct ParsedPost {
    pub meta: Frontmatter,
    pub content: String,
}

/// Generate a full Markdown file with YAML frontmatter + body content
pub fn render_with_frontmatter(meta: &Frontmatter, body: &str) -> String {
    let yaml = serde_yaml::to_string(meta).unwrap_or_default();
    format!("---\n{}---\n\n{}", yaml, body)
}

/// Parse a Markdown file and extract frontmatter + body content
pub fn parse_frontmatter(raw: &str) -> Option<ParsedPost> {
    let trimmed = raw.trim_start();

    if !trimmed.starts_with("---") {
        return None;
    }

    // Find the closing "---"
    let after_open = &trimmed[3..];
    let close_pos = after_open.find("\n---")?;

    let yaml_str = &after_open[..close_pos];
    let rest = &after_open[close_pos + 4..]; // skip "\n---"

    // The body is everything after the closing ---, trimming the leading newlines
    let body = rest.trim_start_matches('\n').trim_start_matches('\r');

    let meta: Frontmatter = serde_yaml::from_str(yaml_str).ok()?;

    Some(ParsedPost {
        meta,
        content: body.to_string(),
    })
}
