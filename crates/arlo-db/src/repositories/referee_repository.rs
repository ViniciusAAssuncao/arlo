use crate::error::{DbError, DbResult};
use crate::models::{RefereeAttributeRow, RefereeRow};
use crate::repositories::attribute_definition_repository;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use crate::repositories::person_repository;
use arlo_domain::{AttributeDefinition, AttributeTarget, Person, Referee};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

async fn load_definitions_map(pool: &SqlitePool) -> DbResult<HashMap<Uuid, AttributeDefinition>> {
    let defs =
        attribute_definition_repository::list_by_applies_to(pool, AttributeTarget::Referee).await?;
    let mut map = HashMap::new();
    for d in defs {
        map.insert(d.id(), d);
    }
    Ok(map)
}

async fn assemble_referee(
    pool: &SqlitePool,
    referee_row: &RefereeRow,
    person: Person,
    def_map: &HashMap<Uuid, AttributeDefinition>,
) -> DbResult<Referee> {
    let attr_rows = fetch_all_by_param::<RefereeAttributeRow>(
        pool,
        "SELECT referee_id, attribute_definition_id, value FROM referee_attributes WHERE referee_id = ?",
        &referee_row.id,
    )
    .await?;

    let mut attributes = Vec::with_capacity(attr_rows.len());
    for ar in attr_rows {
        let def_id = Uuid::parse_str(&ar.attribute_definition_id)?;
        let def = def_map.get(&def_id).ok_or_else(|| {
            DbError::NotFound(format!("AttributeDefinition {} not found", def_id))
        })?;
        attributes.push(ar.to_domain(def)?);
    }

    referee_row.to_domain(person, attributes)
}

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Referee>> {
    let row = fetch_optional_by_param::<RefereeRow>(
        pool,
        "SELECT id, primary_league_id, tier FROM referees WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let row = match row {
        Some(r) => r,
        None => return Ok(None),
    };

    let person = person_repository::get_by_id(pool, id)
        .await?
        .ok_or_else(|| DbError::NotFound(format!("Person not found for referee {}", id)))?;

    let def_map = load_definitions_map(pool).await?;
    let referee = assemble_referee(pool, &row, person, &def_map).await?;
    Ok(Some(referee))
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Referee>> {
    let rows =
        fetch_all::<RefereeRow>(pool, "SELECT id, primary_league_id, tier FROM referees").await?;
    let def_map = load_definitions_map(pool).await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in &rows {
        let id = Uuid::parse_str(&row.id)?;
        let person = person_repository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| DbError::NotFound(format!("Person not found for referee {}", id)))?;
        results.push(assemble_referee(pool, row, person, &def_map).await?);
    }
    Ok(results)
}

pub async fn list_by_primary_league_id(
    pool: &SqlitePool,
    league_id: Uuid,
) -> DbResult<Vec<Referee>> {
    let rows = fetch_all_by_param::<RefereeRow>(
        pool,
        "SELECT id, primary_league_id, tier FROM referees WHERE primary_league_id = ?",
        &league_id.to_string(),
    )
    .await?;
    let def_map = load_definitions_map(pool).await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in &rows {
        let id = Uuid::parse_str(&row.id)?;
        let person = person_repository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| DbError::NotFound(format!("Person not found for referee {}", id)))?;
        results.push(assemble_referee(pool, row, person, &def_map).await?);
    }
    Ok(results)
}