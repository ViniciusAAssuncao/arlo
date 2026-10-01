use crate::error::{DbError, DbResult};
use crate::models::{
    ManagerAttributeRow, ManagerPreferredFormationRow, ManagerRow, ManagerTacticalProfileRow,
};
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use crate::repositories::person_repository;
use arlo_domain::{AttributeDefinition, Manager, Person};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

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

    let profile_row = fetch_optional_by_param::<ManagerTacticalProfileRow>(
        pool,
        "SELECT id, manager_id, offensive_approach, defensive_approach, rotation_policy, artrine_dependency, flexibility_tendency, passing_range_preference, aeriality_preference, structure_preference, physicality_preference, transition_pace_preference, press_block_shape_preference FROM manager_tactical_profiles WHERE manager_id = ?",
        &manager_row.id,
    )
    .await?;

    let tactical_profile = match profile_row {
        Some(pr) => {
            let formation_rows = fetch_all_by_param::<ManagerPreferredFormationRow>(
                pool,
                "SELECT id, manager_tactical_profile_id, formation_id FROM manager_preferred_formations WHERE manager_tactical_profile_id = ?",
                &pr.id,
            )
            .await?;
            Some(pr.to_domain(&formation_rows)?)
        }
        None => None,
    };

    manager_row.to_domain(person, attributes, tactical_profile)
}

pub async fn get_by_id(
    pool: &SqlitePool,
    id: Uuid,
    def_map: &HashMap<Uuid, AttributeDefinition>,
) -> DbResult<Option<Manager>> {
    let row = fetch_optional_by_param::<ManagerRow>(
        pool,
        "SELECT id, team_id, control_mode FROM managers WHERE id = ?",
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

    let manager = assemble_manager(pool, &row, person, def_map).await?;
    Ok(Some(manager))
}

pub async fn list_all(
    pool: &SqlitePool,
    def_map: &HashMap<Uuid, AttributeDefinition>,
) -> DbResult<Vec<Manager>> {
    let rows =
        fetch_all::<ManagerRow>(pool, "SELECT id, team_id, control_mode FROM managers").await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in &rows {
        let id = Uuid::parse_str(&row.id)?;
        let person = person_repository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| DbError::NotFound(format!("Person not found for manager {}", id)))?;
        results.push(assemble_manager(pool, row, person, def_map).await?);
    }
    Ok(results)
}

pub async fn list_by_team_id(
    pool: &SqlitePool,
    team_id: Uuid,
    def_map: &HashMap<Uuid, AttributeDefinition>,
) -> DbResult<Vec<Manager>> {
    let rows = fetch_all_by_param::<ManagerRow>(
        pool,
        "SELECT id, team_id, control_mode FROM managers WHERE team_id = ?",
        &team_id.to_string(),
    )
    .await?;

    let mut results = Vec::with_capacity(rows.len());
    for row in &rows {
        let id = Uuid::parse_str(&row.id)?;
        let person = person_repository::get_by_id(pool, id)
            .await?
            .ok_or_else(|| DbError::NotFound(format!("Person not found for manager {}", id)))?;
        results.push(assemble_manager(pool, row, person, def_map).await?);
    }
    Ok(results)
}
