use crate::error::{DbError, DbResult};
use crate::models::{PlayerAttributeRow, PlayerPositionRow, PlayerRow};
use crate::repositories::fetch::{fetch_all_by_param, fetch_optional_by_param};
use crate::repositories::player::common::{assemble_player, load_definitions_map};
use arlo_domain::{Player, PlayerAttributeValue, PlayerPosition};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

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

    let pos_rows = fetch_all_by_param::<PlayerPositionRow>(
        pool,
        "SELECT player_id, position, proficiency FROM player_positions WHERE player_id = ?",
        &player_row.id,
    )
    .await?;

    let mut positions_by_player: HashMap<Uuid, Vec<PlayerPosition>> = HashMap::new();
    let mut positions = Vec::with_capacity(pos_rows.len());
    for pr in pos_rows {
        positions.push(pr.to_domain()?);
    }
    positions_by_player.insert(id, positions);

    let attr_rows = fetch_all_by_param::<PlayerAttributeRow>(
        pool,
        "SELECT player_id, attribute_definition_id, value FROM player_attributes WHERE player_id = ?",
        &player_row.id,
    )
    .await?;

    let mut attributes_by_player: HashMap<Uuid, Vec<PlayerAttributeValue>> = HashMap::new();
    let mut attributes = Vec::with_capacity(attr_rows.len());
    for ar in attr_rows {
        let def_id = Uuid::parse_str(&ar.attribute_definition_id)?;
        let def = def_map.get(&def_id).ok_or_else(|| {
            DbError::NotFound(format!("AttributeDefinition {} not found", def_id))
        })?;
        attributes.push(ar.to_domain(def)?);
    }
    attributes_by_player.insert(id, attributes);

    let player = assemble_player(&player_row, &positions_by_player, &attributes_by_player)?;
    Ok(Some(player))
}
