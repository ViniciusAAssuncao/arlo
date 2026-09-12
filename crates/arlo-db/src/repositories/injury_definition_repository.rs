use crate::error::DbResult;
use crate::models::injury_mechanism_code::injury_mechanism_to_code;
use crate::models::InjuryDefinitionRow;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::{InjuryDefinition, InjuryMechanism};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<InjuryDefinition>> {
    let row = fetch_optional_by_param::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn get_by_code(pool: &SqlitePool, code: &str) -> DbResult<Option<InjuryDefinition>> {
    let row = fetch_optional_by_param::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions WHERE code = ?",
        code,
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<InjuryDefinition>> {
    let rows = fetch_all::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_mechanism(
    pool: &SqlitePool,
    mechanism: InjuryMechanism,
) -> DbResult<Vec<InjuryDefinition>> {
    let mech_str = injury_mechanism_to_code(mechanism);
    let rows = fetch_all_by_param::<InjuryDefinitionRow>(
        pool,
        "SELECT id, code, description, mechanism, body_region, relative_frequency FROM injury_definitions WHERE mechanism = ?",
        mech_str,
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}