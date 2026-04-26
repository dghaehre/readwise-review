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
            out.push_str(&format!("- [ ] {}\n", h.text.trim()));
            if let Some(ref note) = h.note {
                if !note.is_empty() {
                    out.push_str(&format!("  **Note:** {}\n", note.trim()));
                }
            }
            out.push_str(&format!("  <!-- rw:{} -->\n", h.id));
        }
    }

    out
}

pub fn parse_done_ids(markdown: &str) -> HashSet<u64> {
    let mut ids = HashSet::new();
    let lines: Vec<&str> = markdown.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            // Scan this line and subsequent continuation lines for the rw ID
            for j in i..lines.len() {
                if j > i {
                    let t = lines[j].trim_start();
                    if t.starts_with("- [") || t.starts_with("## ") {
                        break;
                    }
                }
                if let Some(id) = extract_rw_id(lines[j]) {
                    ids.insert(id);
                    break;
                }
            }
        }
        i += 1;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_done_multiline_highlight() {
        let md = r#"# Readwise Review
3 highlights (0 reviewed)

## Some Book
*Some Author*

- [x] What if self-help is similar?

  Obsessing over the self never provides peace.
  <!-- rw:111 -->
- [ ] Short one
  <!-- rw:222 -->
- [x] Another checked
  <!-- rw:333 -->
"#;
        let done = parse_done_ids(md);
        assert!(done.contains(&111));
        assert!(!done.contains(&222));
        assert!(done.contains(&333));
        assert_eq!(done.len(), 2);
    }

    #[test]
    fn generate_puts_id_after_text() {
        let books = vec![Book {
            user_book_id: 1,
            title: "Test".to_string(),
            author: Some("Author".to_string()),
            category: None,
            highlights: vec![Highlight {
                id: 42,
                text: "Line one\n\nLine two".to_string(),
                note: None,
                highlighted_at: None,
                url: None,
                tags: vec![],
                book_id: 1,
                is_deleted: None,
            }],
        }];
        let md = generate_markdown(&books, &HashSet::new());
        assert!(md.contains("- [ ] Line one\n"));
        assert!(md.contains("<!-- rw:42 -->"));
    }
}
