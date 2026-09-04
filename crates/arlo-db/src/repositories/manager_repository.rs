use crate::error::{DbError, DbResult};
use crate::models::ManagerRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use crate::repositories::person_repository;
use arlo_domain::Manager;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Manager>> {
    let row = fetch_optional_by_param::<ManagerRow>(
        pool,
        "SELECT id, team_id FROM managers WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let person = person_repository::get_by_id(pool, id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("Person not found for manager {}", id)))?;

    Ok(Some(row.to_domain(person)?))
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Manager>> {
    let rows = fetch_all::<ManagerRow>(pool, "SELECT id, team_id FROM managers").await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        let id = Uuid::parse_str(&row.id)?;
        let person = person_repository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| DbError::NotFound(format!("Person not found for manager {}", id)))?;
        results.push(row.to_domain(person)?);
    }
    Ok(results)
}

pub async fn list_by_team_id(pool: &SqlitePool, team_id: Uuid) -> DbResult<Vec<Manager>> {
    let rows = fetch_all_by_param::<ManagerRow>(
        pool,
        "SELECT id, team_id FROM managers WHERE team_id = ?",
        &team_id.to_string(),
    )
    .await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        let id = Uuid::parse_str(&row.id)?;
        let person = person_repository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| DbError::NotFound(format!("Person not found for manager {}", id)))?;
        results.push(row.to_domain(person)?);
    }
    Ok(results)
}