use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ExportResponse {
    pub count: u32,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<String>,
    pub results: Vec<Book>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Book {
    pub id: u64,
    pub title: String,
    pub author: Option<String>,
    pub highlights: Vec<Highlight>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Highlight {
    pub id: u64,
    pub text: String,
    pub note: Option<String>,
    pub highlighted_at: Option<String>,
    pub url: Option<String>,
    pub tags: Vec<Tag>,
    pub book_id: u64,
    pub is_deleted: Option<bool>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Tag {
    pub id: u64,
    pub name: String,
}

// --- Local persistent state ---

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppState {
    pub updated_after: Option<String>,
    pub done: HashSet<u64>,
}
