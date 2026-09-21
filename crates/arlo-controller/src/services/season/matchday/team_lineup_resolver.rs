use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_manager_attribute_definitions;
use crate::services::season::matchday::matchday_catalog_cache::get_or_load_matchday_catalogs;
use crate::services::season::matchday::medical_caution_resolver::resolve_lineup_candidates_with_caution;
use arlo_domain::{Formation, Player};
use arlo_engine::manager_ai::LineupSelectionEngine;
use arlo_tactics::TacticalLineup;
use sqlx::SqlitePool;
use std::collections::HashSet;
use uuid::Uuid;

pub async fn resolve_team_lineup(
    pool: &SqlitePool,
    team_id: Uuid,
    available_players: &[Player],
    formations: &[Formation],
) -> ControllerResult<(TacticalLineup, Formation)> {
    if formations.is_empty() {
        return Err(ControllerError::NotFound(
            "No formations available in database".to_string(),
        ));
    }

    let candidate_players =
        resolve_lineup_candidates_with_caution(pool, available_players, formations).await?;
    let candidate_id_set: HashSet<Uuid> = candidate_players.iter().map(|p| p.id()).collect();

    let existing_lineups = arlo_tactics::tactical_lineup::list_by_team_id(pool, team_id)
        .await
        .unwrap_or_default();

    for lineup in existing_lineups {
        if let Some(formation) = formations.iter().find(|f| f.id() == lineup.formation_id()).cloned() {
            let slots_count = formation.slots().len();
            let assignments = lineup.assignments();

            if assignments.len() == slots_count && slots_count >= 14 {
                let mut assigned_players = HashSet::new();
                let mut valid = true;

                for assignment in assignments {
                    let pid = assignment.player_id();
                    if !candidate_id_set.contains(&pid) || !assigned_players.insert(pid) {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    return Ok((lineup, formation));
                }
            }
        }
    }

    let manager_defs = get_or_load_manager_attribute_definitions(pool).await?;
    let managers = arlo_db::repositories::manager::list_by_team_id(pool, team_id, &manager_defs)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let manager = managers.into_iter().next().ok_or_else(|| {
        ControllerError::NotFound(format!("Manager for team {} not found", team_id))
    })?;

    let catalogs = get_or_load_matchday_catalogs(pool).await?;

    let (lineup, _bench) = LineupSelectionEngine::select(
        &candidate_players,
        formations,
        &manager,
        &catalogs.attribute_keys_by_id,
    )
    .or_else(|_| {
        LineupSelectionEngine::select(
            available_players,
            formations,
            &manager,
            &catalogs.attribute_keys_by_id,
        )
    })
    .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let formation = formations
        .iter()
        .find(|f| f.id() == lineup.formation_id())
        .cloned()
        .ok_or_else(|| {
            ControllerError::NotFound(format!("Formation {} not found", lineup.formation_id()))
        })?;

    let _ = arlo_tactics::tactical_lineup::insert(pool, &lineup).await;

    Ok((lineup, formation))
}