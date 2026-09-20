use crate::error::{ControllerError, ControllerResult};
use crate::services::season::matchday::emergency_roster::ensure_minimum_roster;
use crate::services::season::matchday::matchday_catalog_cache::get_or_load_matchday_catalogs;
use crate::services::season::matchday::medical_caution_resolver::resolve_lineup_candidates_with_caution;
use arlo_domain::{Formation, Player};
use arlo_engine::manager_ai::LineupSelectionEngine;
use arlo_recovery::availability::resolve_batch_player_statuses;
use arlo_tactics::TacticalLineup;
use sqlx::SqlitePool;
use std::collections::HashSet;
use uuid::Uuid;

async fn filter_available_players(
    pool: &SqlitePool,
    players: Vec<Player>,
) -> ControllerResult<Vec<Player>> {
    if players.is_empty() {
        return Ok(Vec::new());
    }

    let player_ids: Vec<Uuid> = players.iter().map(|p| p.id()).collect();
    let statuses = resolve_batch_player_statuses(pool, &player_ids)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let available = players
        .into_iter()
        .filter(|p| {
            statuses
                .get(&p.id())
                .map(|s| s.is_available_for_selection())
                .unwrap_or(true)
        })
        .collect();

    Ok(available)
}

pub async fn resolve_team_lineup(
    pool: &SqlitePool,
    team_id: Uuid,
) -> ControllerResult<(TacticalLineup, Formation)> {
    let formations = arlo_db::repositories::formation::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    if formations.is_empty() {
        return Err(ControllerError::NotFound(
            "No formations available in database".to_string(),
        ));
    }

    let raw_players = arlo_db::repositories::player::list_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut players = filter_available_players(pool, raw_players).await?;

    if players.len() < 14 {
        ensure_minimum_roster(pool, team_id, &formations[0]).await?;
        let refreshed = arlo_db::repositories::player::list_by_team_id(pool, team_id)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        players = filter_available_players(pool, refreshed).await?;
    }

    let candidate_players =
        resolve_lineup_candidates_with_caution(pool, &players, &formations).await?;
    let candidate_id_set: HashSet<Uuid> = candidate_players.iter().map(|p| p.id()).collect();

    let existing_lineups = arlo_tactics::tactical_lineup::list_by_team_id(pool, team_id)
        .await
        .unwrap_or_default();

    for lineup in existing_lineups {
        if let Ok(Some(formation)) =
            arlo_db::repositories::formation::get_by_id(pool, lineup.formation_id()).await
        {
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

    let managers = arlo_db::repositories::manager::list_by_team_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let manager = managers.into_iter().next().ok_or_else(|| {
        ControllerError::NotFound(format!("Manager for team {} not found", team_id))
    })?;

    let catalogs = get_or_load_matchday_catalogs(pool).await?;

    let (lineup, _bench) = LineupSelectionEngine::select(
        &candidate_players,
        &formations,
        &manager,
        &catalogs.attribute_keys_by_id,
    )
    .or_else(|_| {
        LineupSelectionEngine::select(
            &players,
            &formations,
            &manager,
            &catalogs.attribute_keys_by_id,
        )
    })
    .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let formation = arlo_db::repositories::formation::get_by_id(pool, lineup.formation_id())
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| {
            ControllerError::NotFound(format!("Formation {} not found", lineup.formation_id()))
        })?;

    let _ = arlo_tactics::tactical_lineup::insert(pool, &lineup).await;

    Ok((lineup, formation))
}