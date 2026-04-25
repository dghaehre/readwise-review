use crate::model::AppState;
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

fn state_path() -> Result<PathBuf> {
    let home = std::env::var("HOME").context("HOME not set")?;
    Ok(PathBuf::from(home).join(".cache/readwise-review-state.json"))
}

pub fn load_state() -> Result<AppState> {
    let path = state_path()?;
    if !path.exists() {
        return Ok(AppState::default());
    }
    let data = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read state file: {}", path.display()))?;
    serde_json::from_str(&data)
        .with_context(|| format!("Failed to parse state file: {}", path.display()))
}

pub fn save_state(state: &AppState) -> Result<()> {
    let path = state_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let data = serde_json::to_string_pretty(state)?;
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, &data)
        .with_context(|| format!("Failed to write temp state file: {}", tmp.display()))?;
    fs::rename(&tmp, &path)
        .with_context(|| format!("Failed to rename state file: {}", path.display()))?;
    Ok(())
}
