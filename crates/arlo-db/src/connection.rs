use crate::error::DbResult;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use sqlx::SqlitePool;
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

pub const TEMPLATE_DATABASE_URL: &str = "sqlite://database/arlo.db";
pub const TEMPLATE_DATABASE_PATH: &str = "database/arlo.db";

pub async fn open_pool(db_url: &str) -> DbResult<SqlitePool> {
    let options = SqliteConnectOptions::from_str(db_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5))
        .pragma("foreign_keys", "ON")
        .pragma("temp_store", "MEMORY")
        .pragma("cache_size", "-64000")
        .pragma("mmap_size", "268435456");

    let pool = SqlitePoolOptions::new()
        .max_connections(8)
        .connect_with(options)
        .await?;

    let mut migrator = sqlx::migrate!("../../migrations");
    migrator.set_ignore_missing(true);
    migrator.run(&pool).await?;

    Ok(pool)
}

pub async fn provision_database(path: &Path) -> DbResult<SqlitePool> {
    let path_str = path.to_str().ok_or_else(|| {
        crate::error::DbError::InvalidData("Invalid database path".to_string())
    })?;
    let db_url = format!("sqlite://{path_str}");
    open_pool(&db_url).await
}