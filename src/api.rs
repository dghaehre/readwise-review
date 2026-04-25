use crate::model::{Book, ExportResponse};
use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use std::thread;
use std::time::Duration;

const EXPORT_URL: &str = "https://readwise.io/api/v2/export/";
const MAX_RETRIES: u32 = 3;

pub fn fetch_all_highlights(token: &str, updated_after: Option<&str>) -> Result<Vec<Book>> {
    let client = Client::new();
    let mut books = Vec::new();
    let mut page_cursor: Option<String> = None;

    loop {
        let response = fetch_page(&client, token, updated_after, page_cursor.as_deref())?;
        books.extend(response.results);

        match response.next_page_cursor {
            Some(cursor) => page_cursor = Some(cursor),
            None => break,
        }
    }

    Ok(books)
}

fn fetch_page(
    client: &Client,
    token: &str,
    updated_after: Option<&str>,
    page_cursor: Option<&str>,
) -> Result<ExportResponse> {
    let mut retries = 0;

    loop {
        let mut req = client
            .get(EXPORT_URL)
            .header("Authorization", format!("Token {token}"));

        if let Some(ts) = updated_after {
            req = req.query(&[("updatedAfter", ts)]);
        }
        if let Some(cursor) = page_cursor {
            req = req.query(&[("pageCursor", cursor)]);
        }

        let resp = req.send().context("Failed to send request to Readwise API")?;
        let status = resp.status();

        if status == reqwest::StatusCode::TOO_MANY_REQUESTS {
            retries += 1;
            if retries > MAX_RETRIES {
                bail!("Readwise API rate limit exceeded after {MAX_RETRIES} retries");
            }
            let wait = resp
                .headers()
                .get("Retry-After")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .unwrap_or(60);
            eprintln!("Rate limited, waiting {wait}s...");
            thread::sleep(Duration::from_secs(wait));
            continue;
        }

        if status == reqwest::StatusCode::UNAUTHORIZED {
            bail!(
                "Invalid or expired Readwise token. \
                 Get yours at https://readwise.io/access_token"
            );
        }

        if !status.is_success() {
            let body = resp.text().unwrap_or_default();
            bail!("Readwise API error {status}: {body}");
        }

        return resp
            .json::<ExportResponse>()
            .context("Failed to parse Readwise API response");
    }
}
