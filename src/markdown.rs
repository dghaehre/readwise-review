use crate::model::{Book, Highlight};
use std::collections::HashSet;

pub fn generate_markdown(books: &[Book], done: &HashSet<u64>) -> String {
    let filtered: Vec<(&Book, Vec<&Highlight>)> = books
        .iter()
        .filter_map(|book| {
            let hl: Vec<&Highlight> = book
                .highlights
                .iter()
                .filter(|h| !done.contains(&h.id))
                .filter(|h| !h.is_deleted.unwrap_or(false))
                .collect();
            if hl.is_empty() { None } else { Some((book, hl)) }
        })
        .collect();

    let total: usize = filtered.iter().map(|(_, hl)| hl.len()).sum();
    let all: usize = books.iter().map(|b| b.highlights.len()).sum();
    let reviewed = all - total;

    if total == 0 {
        return format!("# Readwise Review\n\nNo new highlights to review. ({reviewed} reviewed)\n");
    }

    let mut out = format!("# Readwise Review\n{total} highlights ({reviewed} reviewed)\n");

    for (book, highlights) in &filtered {
        out.push_str(&format!("\n## {}\n", book.title));
        if let Some(ref author) = book.author {
            if !author.is_empty() {
                out.push_str(&format!("*{author}*\n"));
            }
        }
        out.push('\n');

        for h in highlights {
            out.push_str(&format!("- [ ] {} <!-- rw:{} -->\n", h.text.trim(), h.id));
        }
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
