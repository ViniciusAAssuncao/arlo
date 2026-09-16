use crate::connection::open_pool;
use crate::error::DbResult;
use crate::save::paths::{saves_dir, template_database_url, template_dir, template_path};
use std::path::Path;

pub async fn ensure_template_database() -> DbResult<()> {
    let dir = template_dir();
    tokio::fs::create_dir_all(&dir).await?;
    let template_url = template_database_url();
    let pool = open_pool(&template_url).await?;
    pool.close().await;
    Ok(())
}

pub async fn create_save_copy(destination: &Path) -> DbResult<()> {
    create_save_copy_from_path(&template_path(), destination).await
}

pub async fn create_save_copy_from_path(template_path: &Path, destination: &Path) -> DbResult<()> {
    tokio::fs::create_dir_all(template_dir()).await?;
    tokio::fs::create_dir_all(saves_dir()).await?;

    if !template_path.exists() {
        ensure_template_database().await?;
    }

    if let Some(parent) = destination.parent() {
        tokio::fs::create_dir_all(parent).await?;
    }

    tokio::fs::copy(template_path, destination).await?;

    Ok(())
}