use crate::error::{ControllerError, ControllerResult};
use crate::services::season::matchday::emergency_roster::ensure_minimum_roster;
use crate::services::season::matchday::matchday_catalog_cache::get_or_load_matchday_catalogs;
use arlo_domain::{Formation, Player};
use arlo_engine::manager_ai::LineupSelectionEngine;
use arlo_persistence::models::condition::PlayerInjuryHistoryRow;
use arlo_persistence::repositories::condition::player_injury_history;
use arlo_recovery::injury_recovery::return_to_play_evaluator::is_available_for_selection;
use arlo_recovery::InjuryStatusKind;
use arlo_tactics::TacticalLineup;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

async fn filter_available_players(
    pool: &SqlitePool,
    players: Vec<Player>,
) -> ControllerResult<Vec<Player>> {
    if players.is_empty() {
        return Ok(Vec::new());
    }

    let player_ids: Vec<Uuid> = players.iter().map(|p| p.id()).collect();
    let active_injuries = player_injury_history::list_active_by_player_ids(pool, &player_ids).await?;

    let mut injuries_by_player: HashMap<Uuid, PlayerInjuryHistoryRow> =
        HashMap::with_capacity(active_injuries.len());
    for inj in active_injuries {
        if let Ok(pid) = Uuid::parse_str(&inj.player_id) {
            injuries_by_player.entry(pid).or_insert(inj);
        }
    }

    let available = players
        .into_iter()
        .filter(|p| {
            let status = if let Some(inj) = injuries_by_player.get(&p.id()) {
                if inj.days_remaining > 0 || inj.status == "Injured" {
                    InjuryStatusKind::Injured
                } else if inj.observation_days_remaining > 0 || inj.status == "Observation" {
                    InjuryStatusKind::Observation
                } else {
                    InjuryStatusKind::Healthy
                }
            } else {
                InjuryStatusKind::Healthy
            };
            is_available_for_selection(status)
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

    let existing_lineups = arlo_tactics::tactical_lineup::list_by_team_id(pool, team_id)
        .await
        .unwrap_or_default();

    let player_id_set: HashSet<Uuid> = players.iter().map(|p| p.id()).collect();

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
                    if !player_id_set.contains(&pid) || !assigned_players.insert(pid) {
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
        &players,
        &formations,
        &manager,
        &catalogs.attribute_keys_by_id,
    )
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