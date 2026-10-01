use crate::error::{ControllerError, ControllerResult};
use crate::services::season::matchday::automatic_lineup::select_lineup;
use crate::services::season::matchday::manager_lineup::choose_lineup;
use crate::services::season::matchday::manager_match_context::ManagerMatchContext;
use crate::services::season::matchday::medical_caution_resolver::resolve_lineup_candidates_with_caution;
use arlo_domain::{AttributeKey, Formation, Manager, Player};
use arlo_recovery::PlayerCondition;
use arlo_tactics::TacticalLineup;
use sqlx::SqlitePool;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub async fn resolve_team_lineup(
    pool: &SqlitePool,
    team_id: Uuid,
    available_players: &[Player],
    formations: &[Formation],
    manager: &Manager,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    conditions: &HashMap<Uuid, PlayerCondition>,
    context: &ManagerMatchContext,
) -> ControllerResult<(TacticalLineup, Formation)> {
    if formations.is_empty() {
        return Err(ControllerError::NotFound(
            "No formations available in database".to_string(),
        ));
    }

    let candidate_players =
        resolve_lineup_candidates_with_caution(pool, available_players, formations).await?;
    let candidate_id_set: HashSet<Uuid> = candidate_players.iter().map(|p| p.id()).collect();

    if !manager.is_human_controlled() {
        let previous = arlo_tactics::tactical_lineup::get_latest_by_team_id(pool, team_id)
            .await.map_err(|error| ControllerError::InvalidData(error.to_string()))?;
        let (lineup, formation) = choose_lineup(
            team_id, &candidate_players, formations, manager, attribute_keys, conditions, context,
            previous.as_ref(),
        ).or_else(|_| choose_lineup(
            team_id, available_players, formations, manager, attribute_keys, conditions, context,
            previous.as_ref(),
        ))?;
        if let Some(existing) = previous {
            if existing.formation_id() == formation.id()
                && existing.assignments() == lineup.assignments() {
                return Ok((existing, formation));
            }
        }
        arlo_tactics::tactical_lineup::insert(pool, &lineup).await
            .map_err(|error| ControllerError::InvalidData(error.to_string()))?;
        return Ok((lineup, formation));
    }

    let existing_lineups = arlo_tactics::tactical_lineup::list_by_team_id(pool, team_id)
        .await.unwrap_or_default();

    for lineup in existing_lineups {
        if let Some(formation) = formations
            .iter()
            .find(|f| f.id() == lineup.formation_id())
            .cloned()
        {
            let slots_count = formation.slots().len();
            let assignments = lineup.assignments();

            if lineup.team_id() == team_id
                && assignments.len() == slots_count
                && slots_count == 14
                && arlo_tactics::validate_tactical_lineup(
                    &lineup,
                    &formation,
                    &candidate_players,
                )
                .is_ok()
            {
                let mut assigned_players = HashSet::new();
                let mut assigned_slots = HashSet::new();
                let mut valid = true;

                for assignment in assignments {
                    let pid = assignment.player_id();
                    let index = assignment.formation_slot_index();
                    if !candidate_id_set.contains(&pid)
                        || !assigned_players.insert(pid)
                        || !assigned_slots.insert(index)
                        || formation.slots().get(index).is_none()
                    {
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

    let (lineup, formation) = select_lineup(team_id, &candidate_players, formations)
        .or_else(|_| select_lineup(team_id, available_players, formations))?;

    let _ = arlo_tactics::tactical_lineup::insert(pool, &lineup).await;

    Ok((lineup, formation))
}
