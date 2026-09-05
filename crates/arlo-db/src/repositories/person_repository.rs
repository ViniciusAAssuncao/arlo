use crate::error::DbResult;
use crate::models::PersonRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::Person;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Person>> {
    let row = fetch_optional_by_param::<PersonRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id FROM persons WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Person>> {
    let rows = fetch_all::<PersonRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id FROM persons",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_nationality_id(
    pool: &SqlitePool,
    nationality_id: Uuid,
) -> DbResult<Vec<Person>> {
    let rows = fetch_all_by_param::<PersonRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id FROM persons WHERE nationality_id = ?",
        &nationality_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}