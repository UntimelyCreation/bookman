use std::fmt::Display;

use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tags(pub Vec<String>);

impl Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(", "))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBookmarkForm {
    pub name: String,
    pub url: String,
    pub tags: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBookmark {
    pub name: String,
    pub url: String,
    pub tags: Tags,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Bookmark {
    pub id: RecordId,
    pub name: String,
    pub url: String,
    pub tags: Tags,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchForm {
    pub content: String,
}
