use crate::error::DbResult;
use crate::models::PlayerRow;
use crate::repositories::attribute_definition_repository;
use arlo_domain::{
    AttributeDefinition, AttributeTarget, Player, PlayerAttributeValue, PlayerPosition,
};
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn load_definitions_map(pool: &SqlitePool) -> DbResult<HashMap<Uuid, AttributeDefinition>> {
    let defs =
        attribute_definition_repository::list_by_applies_to(pool, AttributeTarget::Player).await?;
    let mut map = HashMap::new();
    for d in defs {
        map.insert(d.id(), d);
    }
    Ok(map)
}

pub fn assemble_player(
    player_row: &PlayerRow,
    positions_by_player: &HashMap<Uuid, Vec<PlayerPosition>>,
    attributes_by_player: &HashMap<Uuid, Vec<PlayerAttributeValue>>,
) -> DbResult<Player> {
    let player_id = Uuid::parse_str(&player_row.id)?;
    let positions = positions_by_player
        .get(&player_id)
        .cloned()
        .unwrap_or_default();
    let attributes = attributes_by_player
        .get(&player_id)
        .cloned()
        .unwrap_or_default();

    player_row.to_domain(positions, attributes)
}
