use crate::error::{ControllerError, ControllerResult};
use arlo_domain::Formation;
use arlo_tactics::TacticalLineup;
use sqlx::SqlitePool;
use std::collections::HashSet;
use uuid::Uuid;

pub async fn resolve_team_lineup(
    pool: &SqlitePool,
    team_id: Uuid,
) -> ControllerResult<(TacticalLineup, Formation)> {
    let existing_lineups = arlo_tactics::tactical_lineup::list_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let formations = arlo_db::repositories::formation::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    if formations.is_empty() {
        return Err(ControllerError::NotFound(
            "No formations available in database".to_string(),
        ));
    }

    if let Some(lineup) = existing_lineups.into_iter().next() {
        let formation = arlo_db::repositories::formation::get_by_id(pool, lineup.formation_id())
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?
            .ok_or_else(|| {
                ControllerError::NotFound(format!("Formation {} not found", lineup.formation_id()))
            })?;
        return Ok((lineup, formation));
    }

    let formation = formations[0].clone();
    let players = arlo_db::repositories::player::list_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    if players.len() < formation.slots().len() {
        return Err(ControllerError::Validation(format!(
            "Team {} has {} players, but formation requires {}",
            team_id,
            players.len(),
            formation.slots().len()
        )));
    }

    let mut assigned_players = HashSet::new();
    let mut builder = TacticalLineup::builder(Uuid::new_v4(), team_id, "Escalação Padrão")
        .with_formation(&formation);

    for (slot_idx, slot) in formation.slots().iter().enumerate() {
        let required_pos = slot.position();

        let best_player = players
            .iter()
            .filter(|p| !assigned_players.contains(&p.id()))
            .filter_map(|p| {
                p.positions()
                    .iter()
                    .find(|pos| pos.position() == required_pos)
                    .map(|pos| (p, pos.proficiency()))
            })
            .max_by_key(|(_, prof)| *prof)
            .map(|(p, _)| p)
            .or_else(|| {
                players
                    .iter()
                    .find(|p| !assigned_players.contains(&p.id()))
            })
            .ok_or_else(|| {
                ControllerError::Validation(format!(
                    "Not enough players to fill slot {} in team {}",
                    slot_idx, team_id
                ))
            })?;

        assigned_players.insert(best_player.id());
        builder = builder.assign(slot_idx, best_player.id());
    }

    let lineup = builder
        .build(&players)
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    arlo_tactics::tactical_lineup::insert(pool, &lineup)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    Ok((lineup, formation))
}