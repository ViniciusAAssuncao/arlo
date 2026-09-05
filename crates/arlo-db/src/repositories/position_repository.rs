use crate::error::DbResult;
use crate::models::PositionRow;
use crate::repositories::fetch::{fetch_all, fetch_optional_by_param};
use arlo_domain::Position;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Position>> {
    let row = fetch_optional_by_param::<PositionRow>(
        pool,
        "SELECT id, code, name FROM positions WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Position>> {
    let rows = fetch_all::<PositionRow>(pool, "SELECT id, code, name FROM positions").await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}