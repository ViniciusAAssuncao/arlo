use crate::error::ControllerResult;
use sqlx::SqlitePool;

pub async fn run_migrations(pool: &SqlitePool) -> ControllerResult<()> {
    let mut migrator = sqlx::migrate!("./migrations");
    migrator.set_ignore_missing(true);
    migrator.run(pool).await?;
    Ok(())
}
