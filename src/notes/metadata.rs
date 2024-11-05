use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteMetadata {
    pub title: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created: DateTime<Utc>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub links: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl NoteMetadata {
    pub fn new(title: String) -> Self {
        NoteMetadata {
            title,
            created: Utc::now(),
            tags: Vec::new(),
            links: Vec::new(),
            description: None,
        }
    }
} 