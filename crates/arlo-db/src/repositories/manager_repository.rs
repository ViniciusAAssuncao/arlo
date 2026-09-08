use crate::error::{DbError, DbResult};
use crate::models::{ManagerAttributeRow, ManagerRow};
use crate::repositories::attribute_definition_repository;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use crate::repositories::person_repository;
use arlo_domain::{AttributeDefinition, AttributeTarget, Manager, Person};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

async fn load_definitions_map(pool: &SqlitePool) -> DbResult<HashMap<Uuid, AttributeDefinition>> {
    let defs =
        attribute_definition_repository::list_by_applies_to(pool, AttributeTarget::Manager).await?;
    let mut map = HashMap::new();
    for d in defs {
        map.insert(d.id(), d);
    }
    Ok(map)
}

async fn assemble_manager(
    pool: &SqlitePool,
    manager_row: &ManagerRow,
    person: Person,
    def_map: &HashMap<Uuid, AttributeDefinition>,
) -> DbResult<Manager> {
    let attr_rows = fetch_all_by_param::<ManagerAttributeRow>(
        pool,
        "SELECT manager_id, attribute_definition_id, value FROM manager_attributes WHERE manager_id = ?",
        &manager_row.id,
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

    manager_row.to_domain(person, attributes)
}

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

    let def_map = load_definitions_map(pool).await?;
    let manager = assemble_manager(pool, &row, person, &def_map).await?;
    Ok(Some(manager))
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Manager>> {
    let rows = fetch_all::<ManagerRow>(pool, "SELECT id, team_id FROM managers").await?;
    let def_map = load_definitions_map(pool).await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in &rows {
        let id = Uuid::parse_str(&row.id)?;
        let person = person_repository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| DbError::NotFound(format!("Person not found for manager {}", id)))?;
        results.push(assemble_manager(pool, row, person, &def_map).await?);
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
    let def_map = load_definitions_map(pool).await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in &rows {
        let id = Uuid::parse_str(&row.id)?;
        let person = person_repository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| DbError::NotFound(format!("Person not found for manager {}", id)))?;
        results.push(assemble_manager(pool, row, person, &def_map).await?);
    }
    Ok(results)
}
