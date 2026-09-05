use crate::error::{DbError, DbResult};
use crate::models::{PlayerAttributeRow, PlayerPositionRow, PlayerRow};
use crate::repositories::attribute_definition_repository;
use crate::repositories::fetch::{fetch_all, fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::{AttributeDefinition, AttributeTarget, Player};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

async fn load_definitions_map(
    pool: &SqlitePool,
) -> DbResult<HashMap<Uuid, AttributeDefinition>> {
    let defs = attribute_definition_repository::list_by_applies_to(
        pool,
        AttributeTarget::Player,
    )
    .await?;
    let mut map = HashMap::new();
    for d in defs {
        map.insert(d.id(), d);
    }
    Ok(map)
}

async fn assemble_player(
    pool: &SqlitePool,
    player_row: &PlayerRow,
    def_map: &HashMap<Uuid, AttributeDefinition>,
) -> DbResult<Player> {
    let pos_rows = fetch_all_by_param::<PlayerPositionRow>(
        pool,
        "SELECT player_id, position, proficiency FROM player_positions WHERE player_id = ?",
        &player_row.id,
    )
    .await?;

    let mut positions = Vec::with_capacity(pos_rows.len());
    for pr in pos_rows {
        positions.push(pr.to_domain()?);
    }

    let attr_rows = fetch_all_by_param::<PlayerAttributeRow>(
        pool,
        "SELECT player_id, attribute_definition_id, value FROM player_attributes WHERE player_id = ?",
        &player_row.id,
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

    player_row.to_domain(positions, attributes)
}

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Player>> {
    let player_row = fetch_optional_by_param::<PlayerRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id, team_id, squad_number, captaincy_role FROM players WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    let player_row = match player_row {
        Some(pr) => pr,
        None => return Ok(None),
    };

    let def_map = load_definitions_map(pool).await?;
    let player = assemble_player(pool, &player_row, &def_map).await?;
    Ok(Some(player))
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Player>> {
    let player_rows = fetch_all::<PlayerRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id, team_id, squad_number, captaincy_role FROM players",
    )
    .await?;

    let def_map = load_definitions_map(pool).await?;
    let mut results = Vec::with_capacity(player_rows.len());
    for pr in &player_rows {
        results.push(assemble_player(pool, pr, &def_map).await?);
    }
    Ok(results)
}

pub async fn list_by_team_id(pool: &SqlitePool, team_id: Uuid) -> DbResult<Vec<Player>> {
    let player_rows = fetch_all_by_param::<PlayerRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id, team_id, squad_number, captaincy_role FROM players WHERE team_id = ?",
        &team_id.to_string(),
    )
    .await?;

    let def_map = load_definitions_map(pool).await?;
    let mut results = Vec::with_capacity(player_rows.len());
    for pr in &player_rows {
        results.push(assemble_player(pool, pr, &def_map).await?);
    }
    Ok(results)
}

pub async fn list_by_nationality_id(
    pool: &SqlitePool,
    nationality_id: Uuid,
) -> DbResult<Vec<Player>> {
    let player_rows = fetch_all_by_param::<PlayerRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id, team_id, squad_number, captaincy_role FROM players WHERE nationality_id = ?",
        &nationality_id.to_string(),
    )
    .await?;

    let def_map = load_definitions_map(pool).await?;
    let mut results = Vec::with_capacity(player_rows.len());
    for pr in &player_rows {
        results.push(assemble_player(pool, pr, &def_map).await?);
    }
    Ok(results)
}