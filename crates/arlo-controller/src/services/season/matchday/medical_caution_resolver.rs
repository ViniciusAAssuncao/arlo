use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_attribute_key_index;
use crate::services::player::player_ability_service::calculate_player_ability;
use arlo_domain::{Formation, Player, Position};
use arlo_recovery::{resolve_batch_readiness, ReadinessLevel, ReadinessTuningProfile};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn resolve_lineup_candidates_with_caution(
    pool: &SqlitePool,
    players: &[Player],
    formations: &[Formation],
) -> ControllerResult<Vec<Player>> {
    if players.is_empty() {
        return Ok(Vec::new());
    }

    let min_required = formations
        .iter()
        .map(|f| f.slots().len())
        .max()
        .unwrap_or(14)
        .max(14);

    if players.len() <= min_required {
        return Ok(players.to_vec());
    }

    let metadata = arlo_db::repositories::save_metadata::get(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let save_calendar_row = if let Some(meta) = &metadata {
        arlo_persistence::repositories::calendar::save_calendar_state::get_by_save_uuid(
            pool,
            meta.save_uuid(),
        )
        .await?
    } else {
        None
    };

    let current_unix_seconds = if let Some(row) = &save_calendar_row {
        (row.current_year - 1970) * 31_557_600 + (row.current_day_of_year as i64) * 86_400
    } else {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    };

    let player_ids: Vec<Uuid> = players.iter().map(|p| p.id()).collect();
    let tuning = ReadinessTuningProfile::default();
    let readiness_map = resolve_batch_readiness(pool, &player_ids, current_unix_seconds, &tuning)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let key_index = get_or_load_attribute_key_index(pool).await?;
    let mut player_ca = HashMap::with_capacity(players.len());
    for p in players {
        let ca = calculate_player_ability(p, &key_index).unwrap_or(50);
        player_ca.insert(p.id(), ca);
    }

    let mut cautious_player_ids: Vec<Uuid> = players
        .iter()
        .filter(|p| {
            readiness_map.get(&p.id()).map_or(false, |r| {
                r.level != ReadinessLevel::FullyFit || r.score < tuning.caution_threshold
            })
        })
        .map(|p| p.id())
        .collect();

    if cautious_player_ids.is_empty() {
        return Ok(players.to_vec());
    }

    cautious_player_ids.sort_by(|a, b| {
        let score_a = readiness_map.get(a).map_or(1.0, |r| r.score);
        let score_b = readiness_map.get(b).map_or(1.0, |r| r.score);
        score_a
            .partial_cmp(&score_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut available_candidates: Vec<Player> = players.to_vec();

    for cautious_id in cautious_player_ids {
        if available_candidates.len() <= min_required {
            break;
        }

        let cautious_player = available_candidates
            .iter()
            .find(|p| p.id() == cautious_id)
            .unwrap();

        let cautious_ca = player_ca.get(&cautious_id).copied().unwrap_or(50);
        let cautious_positions: Vec<Position> = cautious_player
            .positions()
            .iter()
            .filter(|pos| pos.proficiency() >= tuning.caution_min_position_proficiency)
            .map(|pos| pos.position())
            .collect();

        let target_positions = if cautious_positions.is_empty() {
            cautious_player
                .positions()
                .iter()
                .max_by_key(|pos| pos.proficiency())
                .map(|pos| vec![pos.position()])
                .unwrap_or_default()
        } else {
            cautious_positions
        };

        let has_goalguard = target_positions.contains(&Position::Goalguard);

        let has_alternative = available_candidates.iter().any(|other| {
            if other.id() == cautious_id {
                return false;
            }

            let other_readiness = readiness_map.get(&other.id());
            let is_fit = other_readiness.map_or(true, |r| {
                r.score >= tuning.caution_threshold && r.level == ReadinessLevel::FullyFit
            });
            if !is_fit {
                return false;
            }

            let other_ca = player_ca.get(&other.id()).copied().unwrap_or(50);
            if cautious_ca - other_ca > tuning.caution_ability_tolerance {
                return false;
            }

            other.positions().iter().any(|pos| {
                target_positions.contains(&pos.position())
                    && pos.proficiency() >= tuning.caution_min_position_proficiency
            })
        });

        if has_alternative {
            if has_goalguard {
                let other_gg_count = available_candidates
                    .iter()
                    .filter(|other| {
                        other.id() != cautious_id
                            && other
                                .positions()
                                .iter()
                                .any(|pos| pos.position() == Position::Goalguard)
                    })
                    .count();
                if other_gg_count == 0 {
                    continue;
                }
            }

            available_candidates.retain(|p| p.id() != cautious_id);
        }
    }

    if available_candidates.len() < min_required {
        return Ok(players.to_vec());
    }

    Ok(available_candidates)
}
