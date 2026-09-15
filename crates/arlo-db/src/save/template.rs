use crate::connection::open_pool;
use crate::error::{DbError, DbResult};
use crate::save::paths::{saves_dir, template_database_url, template_dir, template_path};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, SqliteConnection};
use std::path::Path;
use std::str::FromStr;

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

    let path_str = template_path
        .to_str()
        .ok_or_else(|| DbError::InvalidData("Invalid template path".to_string()))?;
    let normalized_template = path_str.replace('\\', "/");
    let template_url = format!("sqlite://{normalized_template}");
    let options = SqliteConnectOptions::from_str(&template_url)?.read_only(true);
    let mut conn = SqliteConnection::connect_with(&options).await?;

    let dest_str = destination
        .to_str()
        .ok_or_else(|| DbError::InvalidData("Destination path is not valid UTF-8".to_string()))?;

    sqlx::query("VACUUM INTO ?")
        .bind(dest_str)
        .execute(&mut conn)
        .await?;

    Ok(())
}