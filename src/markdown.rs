use crate::model::{Book, Highlight};
use std::collections::HashSet;

pub fn generate_markdown(books: &[Book], done: &HashSet<u64>) -> String {
    let mut out = String::from("# Readwise Review\n");
    let mut has_highlights = false;

    for book in books {
        let highlights: Vec<&Highlight> = book
            .highlights
            .iter()
            .filter(|h| !done.contains(&h.id))
            .filter(|h| !h.is_deleted.unwrap_or(false))
            .collect();

        if highlights.is_empty() {
            continue;
        }

        has_highlights = true;
        out.push_str(&format!("\n## {}\n", book.title));
        if let Some(ref author) = book.author {
            if !author.is_empty() {
                out.push_str(&format!("*{author}*\n"));
            }
        }
        out.push('\n');

        for h in &highlights {
            out.push_str(&format!("- [ ] {} <!-- rw:{} -->\n", h.text.trim(), h.id));
        }
    }

    if !has_highlights {
        out.push_str("\nNo new highlights to review.\n");
    }

    out
}

pub fn parse_done_ids(markdown: &str) -> HashSet<u64> {
    let mut ids = HashSet::new();
    for line in markdown.lines() {
        let trimmed = line.trim_start();
        if !trimmed.starts_with("- [x]") && !trimmed.starts_with("- [X]") {
            continue;
        }
        if let Some(id) = extract_rw_id(line) {
            ids.insert(id);
        }
    }
    ids
}

fn extract_rw_id(line: &str) -> Option<u64> {
    let marker = "<!-- rw:";
    let start = line.find(marker)? + marker.len();
    let rest = &line[start..];
    let end = rest.find(" -->")?;
    rest[..end].parse().ok()
}
