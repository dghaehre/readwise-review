use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ExportResponse {
    pub count: u32,
    #[serde(rename = "nextPageCursor")]
    pub next_page_cursor: Option<serde_json::Value>,
    pub results: Vec<Book>,
}

impl ExportResponse {
    pub fn next_cursor_string(&self) -> Option<String> {
        self.next_page_cursor.as_ref().map(|v| match v {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        })
    }
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Book {
    pub user_book_id: u64,
    pub title: String,
    pub author: Option<String>,
    pub category: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_export_response_with_integer_cursor() {
        let json = r#"{
            "count": 503,
            "nextPageCursor": 49244648,
            "results": [{
                "user_book_id": 59973911,
                "is_deleted": false,
                "title": "Some Article",
                "author": "Test Author",
                "category": "articles",
                "highlights": [{
                    "id": 1009507315,
                    "is_deleted": false,
                    "text": "A highlight",
                    "location": 93,
                    "location_type": "offset",
                    "note": "",
                    "color": "",
                    "highlighted_at": "2026-04-25T10:05:58.054Z",
                    "created_at": "2026-04-25T10:05:58.124Z",
                    "updated_at": "2026-04-25T10:05:58.124Z",
                    "url": "https://example.com",
                    "book_id": 59973911,
                    "tags": [],
                    "is_favorite": false,
                    "is_discard": false,
                    "readwise_url": "https://readwise.io/open/1009507315"
                }]
            }]
        }"#;

        let resp: ExportResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.count, 503);
        assert_eq!(resp.next_cursor_string(), Some("49244648".to_string()));
        assert_eq!(resp.results.len(), 1);
        assert_eq!(resp.results[0].title, "Some Article");
        assert_eq!(resp.results[0].highlights[0].id, 1009507315);
    }

    #[test]
    fn parse_export_response_with_null_cursor() {
        let json = r#"{
            "count": 1,
            "nextPageCursor": null,
            "results": []
        }"#;

        let resp: ExportResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.next_cursor_string(), None);
    }
}
