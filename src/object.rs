use serde::{Deserialize, Serialize};
use surrealdb::opt::RecordId;

#[derive(Debug, Serialize, Deserialize)]
pub struct NewBookmark {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Bookmark {
    pub id: RecordId,
    pub name: String,
    pub url: String,
}
