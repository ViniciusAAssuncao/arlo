use crate::connection::open_pool;
use crate::error::{DbError, DbResult};
use crate::save::paths::{template_database_url, template_path, TEMPLATE_DIRECTORY};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Connection, SqliteConnection};
use std::path::Path;
use std::str::FromStr;

pub async fn ensure_template_database() -> DbResult<()> {
    tokio::fs::create_dir_all(TEMPLATE_DIRECTORY).await?;
    let template_url = template_database_url();
    let pool = open_pool(&template_url).await?;
    pool.close().await;
    Ok(())
}

pub async fn create_save_copy(destination: &Path) -> DbResult<()> {
    ensure_template_database().await?;

    let t_path = template_path();
    if !t_path.exists() {
        return Err(DbError::NotFound(
            "Template database must exist before creating save".to_string(),
        ));
    }

    let template_url = template_database_url();
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
