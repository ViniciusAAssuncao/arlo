use crate::connection::open_pool;
use crate::error::DbResult;
use crate::save::migrations::run_migrations;
use crate::save::paths::{
    list_existing_saves, most_recent_save, save_database_url, save_filename, save_path,
    template_database_url,
};
use crate::save::template::create_save_copy;
use arlo_domain::SaveMetadata;
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn create_new_save() -> DbResult<(SqlitePool, SaveMetadata)> {
    let uuid = Uuid::new_v4();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| crate::error::DbError::InvalidData(e.to_string()))?;
    let timestamp = now.as_secs() as i64;
    let filename = save_filename(timestamp);
    let destination = save_path(&filename);

    create_save_copy(&destination).await?;

    let url = save_database_url(&filename);
    let pool = open_pool(&url).await?;
    run_migrations(&pool).await?;

    let metadata = SaveMetadata::new(uuid, timestamp, template_database_url())?;
    crate::repositories::save_metadata::insert(&pool, &metadata).await?;

    Ok((pool, metadata))
}

pub async fn resolve_current_save_pool() -> DbResult<SqlitePool> {
    let saves = list_existing_saves().await?;
    if let Some(recent) = most_recent_save(&saves) {
        let filename = recent.file_name().and_then(|n| n.to_str()).ok_or_else(|| {
            crate::error::DbError::InvalidData("Invalid save filename".to_string())
        })?;

        let url = save_database_url(filename);
        let pool = open_pool(&url).await?;
        run_migrations(&pool).await?;
        Ok(pool)
    } else {
        let (pool, _) = create_new_save().await?;
        Ok(pool)
    }
}
