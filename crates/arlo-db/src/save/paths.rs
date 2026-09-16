use crate::error::DbResult;
use crate::save::template::ensure_template_database;
use std::path::{Path, PathBuf};

pub const TEMPLATE_FILENAME: &str = "arlo.db";

fn check_candidate_for_database(candidate: &Path) -> Option<PathBuf> {
    let db_file = candidate.join("database").join(TEMPLATE_FILENAME);
    if db_file.exists() {
        return Some(candidate.to_path_buf());
    }
    let db_dir = candidate.join("database");
    if db_dir.is_dir() {
        return Some(candidate.to_path_buf());
    }
    None
}

pub fn project_root_dir() -> PathBuf {
    if let Ok(dir) = std::env::var("ARLO_PROJECT_ROOT") {
        return PathBuf::from(dir);
    }

    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let manifest_path = PathBuf::from(manifest_dir);
        let mut current = manifest_path.as_path();
        while let Some(parent) = current.parent() {
            if let Some(found) = check_candidate_for_database(current) {
                return found;
            }
            current = parent;
        }
    }

    let candidates = [
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("../.."),
        PathBuf::from("../../.."),
        PathBuf::from("../../../.."),
        PathBuf::from("arlo"),
        PathBuf::from("../arlo"),
        PathBuf::from("../../arlo"),
        PathBuf::from("../../../arlo"),
    ];

    for candidate in &candidates {
        if let Some(found) = check_candidate_for_database(candidate) {
            return found;
        }
    }

    if let Ok(current_dir) = std::env::current_dir() {
        let mut current = current_dir.as_path();
        loop {
            if let Some(found) = check_candidate_for_database(current) {
                return found;
            }
            let arlo_sub = current.join("arlo");
            if let Some(found) = check_candidate_for_database(&arlo_sub) {
                return found;
            }
            match current.parent() {
                Some(parent) => current = parent,
                None => break,
            }
        }
    }

    if let Ok(exe_path) = std::env::current_exe() {
        let mut current = exe_path.as_path();
        while let Some(parent) = current.parent() {
            if let Some(found) = check_candidate_for_database(parent) {
                return found;
            }
            current = parent;
        }
    }

    PathBuf::from(".")
}

pub fn saves_dir() -> PathBuf {
    project_root_dir().join("saves")
}

pub fn template_dir() -> PathBuf {
    project_root_dir().join("database")
}

pub fn save_filename(timestamp: i64) -> String {
    format!("save-{timestamp}.db")
}

pub fn save_path(filename: &str) -> PathBuf {
    saves_dir().join(filename)
}

pub fn template_path() -> PathBuf {
    template_dir().join(TEMPLATE_FILENAME)
}

pub fn template_path_for(filename: &str) -> PathBuf {
    template_dir().join(filename)
}

pub fn save_database_url(filename: &str) -> String {
    let path = save_path(filename);
    let normalized = path.to_string_lossy().replace('\\', "/");
    format!("sqlite://{normalized}")
}

pub fn template_database_url() -> String {
    let path = template_path();
    let normalized = path.to_string_lossy().replace('\\', "/");
    format!("sqlite://{normalized}")
}

pub fn template_database_url_for(filename: &str) -> String {
    let path = template_path_for(filename);
    let normalized = path.to_string_lossy().replace('\\', "/");
    format!("sqlite://{normalized}")
}

pub async fn list_existing_saves() -> DbResult<Vec<PathBuf>> {
    let dir = saves_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let mut entries = tokio::fs::read_dir(&dir).await?;
    let mut saves = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let is_save_prefix = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(|n| n.starts_with("save-"))
            .unwrap_or(false);

        let is_db_ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e == "db")
            .unwrap_or(false);

        if is_save_prefix && is_db_ext {
            saves.push(path);
        }
    }

    Ok(saves)
}

pub async fn list_existing_templates() -> DbResult<Vec<PathBuf>> {
    let dir = template_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let mut entries = tokio::fs::read_dir(&dir).await?;
    let mut templates = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        let is_db_ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e == "db")
            .unwrap_or(false);

        if is_db_ext {
            templates.push(path);
        }
    }

    if templates.is_empty() {
        ensure_template_database().await?;
        templates.push(template_path());
    }

    Ok(templates)
}

pub fn most_recent_save(saves: &[PathBuf]) -> Option<&PathBuf> {
    saves
        .iter()
        .max_by(|a, b| a.file_name().cmp(&b.file_name()))
}