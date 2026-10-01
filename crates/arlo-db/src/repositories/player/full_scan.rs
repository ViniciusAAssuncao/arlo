use crate::error::{DbError, DbResult};
use crate::models::{PlayerAttributeRow, PlayerPositionRow, PlayerRow};
use crate::repositories::fetch::fetch_all;
use crate::repositories::player::common::{assemble_player, load_definitions_map};
use arlo_domain::{AttributeDefinition, Player, PlayerAttributeValue, PlayerPosition};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

async fn load_positions_map(pool: &SqlitePool) -> DbResult<HashMap<Uuid, Vec<PlayerPosition>>> {
    let pos_rows = fetch_all::<PlayerPositionRow>(
        pool,
        "SELECT player_id, position, proficiency FROM player_positions",
    )
    .await?;

    let mut positions_by_player: HashMap<Uuid, Vec<PlayerPosition>> = HashMap::new();
    for pr in pos_rows {
        let player_id = Uuid::parse_str(&pr.player_id)?;
        positions_by_player
            .entry(player_id)
            .or_default()
            .push(pr.to_domain()?);
    }

    Ok(positions_by_player)
}

async fn load_attributes_map(
    pool: &SqlitePool,
    def_map: &HashMap<Uuid, AttributeDefinition>,
) -> DbResult<HashMap<Uuid, Vec<PlayerAttributeValue>>> {
    let attr_rows = fetch_all::<PlayerAttributeRow>(
        pool,
        "SELECT player_id, attribute_definition_id, value FROM player_attributes",
    )
    .await?;

    let mut attributes_by_player: HashMap<Uuid, Vec<PlayerAttributeValue>> = HashMap::new();
    for ar in attr_rows {
        let player_id = Uuid::parse_str(&ar.player_id)?;
        let def_id = Uuid::parse_str(&ar.attribute_definition_id)?;
        let def = def_map.get(&def_id).ok_or_else(|| {
            DbError::NotFound(format!("AttributeDefinition {} not found", def_id))
        })?;
        attributes_by_player
            .entry(player_id)
            .or_default()
            .push(ar.to_domain(def)?);
    }

    Ok(attributes_by_player)
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Player>> {
    let player_rows = fetch_all::<PlayerRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id, team_id, squad_number, captaincy_role FROM players",
    )
    .await?;

    let def_map = load_definitions_map(pool).await?;
    let positions_by_player = load_positions_map(pool).await?;
    let attributes_by_player = load_attributes_map(pool, &def_map).await?;

    let mut results = Vec::with_capacity(player_rows.len());
    for pr in &player_rows {
        results.push(assemble_player(
            pr,
            &positions_by_player,
            &attributes_by_player,
        )?);
    }
    Ok(results)
}

pub async fn list_all_with_team(pool: &SqlitePool) -> DbResult<Vec<Player>> {
    let player_rows = fetch_all::<PlayerRow>(
        pool,
        "SELECT id, name, height_m, birthdate_unix_seconds, nationality_id, team_id, squad_number, captaincy_role FROM players WHERE team_id IS NOT NULL",
    )
    .await?;

    let def_map = load_definitions_map(pool).await?;
    let positions_by_player = load_positions_map(pool).await?;
    let attributes_by_player = load_attributes_map(pool, &def_map).await?;

    let mut results = Vec::with_capacity(player_rows.len());
    for pr in &player_rows {
        results.push(assemble_player(
            pr,
            &positions_by_player,
            &attributes_by_player,
        )?);
    }
    Ok(results)
}
