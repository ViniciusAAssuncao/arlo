use crate::error::RecoveryResult;
use crate::orchestration::match_condition_bridge::load_conditions_for_players;
use crate::readiness::match_readiness_score::{
    calculate_match_readiness, MatchReadinessAssessment,
};
use crate::tuning::ReadinessTuningProfile;
use arlo_persistence::models::condition::PlayerInjuryHistoryRow;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn resolve_batch_readiness(
    pool: &SqlitePool,
    player_ids: &[Uuid],
    current_unix_seconds: i64,
    tuning: &ReadinessTuningProfile,
) -> RecoveryResult<HashMap<Uuid, MatchReadinessAssessment>> {
    if player_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let conditions = load_conditions_for_players(pool, player_ids).await?;

    let window_seconds = (tuning.recent_return_window_days as i64) * 86_400;
    let since_unix_seconds = current_unix_seconds.saturating_sub(window_seconds);

    let resolved_rows = arlo_persistence::repositories::condition::player_injury_history::list_latest_resolved_by_player_ids(
        pool,
        player_ids,
        since_unix_seconds,
    )
    .await?;

    let mut resolved_by_player: HashMap<Uuid, PlayerInjuryHistoryRow> =
        HashMap::with_capacity(resolved_rows.len());
    for row in resolved_rows {
        if let Ok(pid) = Uuid::parse_str(&row.player_id) {
            resolved_by_player.entry(pid).or_insert(row);
        }
    }

    let mut results = HashMap::with_capacity(player_ids.len());

    for &player_id in player_ids {
        let condition = match conditions.get(&player_id) {
            Some(c) => c,
            None => continue,
        };

        let days_since_resolution = resolved_by_player.get(&player_id).and_then(|row| {
            row.resolved_at_unix_seconds.map(|resolved_at| {
                let elapsed = current_unix_seconds.saturating_sub(resolved_at);
                (elapsed / 86_400) as u32
            })
        });

        let assessment = calculate_match_readiness(condition, days_since_resolution, tuning);
        results.insert(player_id, assessment);
    }

    Ok(results)
}

pub async fn resolve_player_readiness(
    pool: &SqlitePool,
    player_id: Uuid,
    current_unix_seconds: i64,
    tuning: &ReadinessTuningProfile,
) -> RecoveryResult<MatchReadinessAssessment> {
    let mut map = resolve_batch_readiness(pool, &[player_id], current_unix_seconds, tuning).await?;
    map.remove(&player_id)
        .ok_or_else(|| crate::error::RecoveryError::NotFound(player_id))
}