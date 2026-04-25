mod api;
mod markdown;
mod model;
mod state;

use anyhow::Result;
use std::fs;
use std::process::Command;

const INBOX_PATH: &str = "/tmp/readwise-review-inbox.md";

fn main() -> Result<()> {
    let token = std::env::var("READWISE_TOKEN").unwrap_or_else(|_| {
        eprintln!(
            "READWISE_TOKEN not set.\n\
             Get your token at https://readwise.io/access_token\n\
             Then: export READWISE_TOKEN=<your token>"
        );
        std::process::exit(1);
    });

    let mut app_state = state::load_state()?;

    eprintln!("Fetching highlights from Readwise...");
    let books = api::fetch_all_highlights(&token, app_state.updated_after.as_deref())?;

    let md = markdown::generate_markdown(&books, &app_state.done);

    if md.contains("No new highlights to review.") {
        println!("No new highlights to review.");
        return Ok(());
    }

    fs::write(INBOX_PATH, &md)?;
    eprintln!("Opening inbox in nvim...");

    Command::new("nvim")
        .arg(INBOX_PATH)
        .status()?;

    let updated_md = fs::read_to_string(INBOX_PATH)?;
    let newly_done = markdown::parse_done_ids(&updated_md);
    let count = newly_done.len();

    if count > 0 {
        app_state.done.extend(newly_done);
        state::save_state(&app_state)?;
        println!("Marked {count} highlights as reviewed ({} total).", app_state.done.len());
    }

    Ok(())
}
