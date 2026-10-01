use crate::availability::status_resolver::{resolve_status_from_row, PlayerMedicalStatus};
use crate::error::RecoveryResult;
use arlo_persistence::models::condition::PlayerInjuryHistoryRow;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn resolve_batch_player_statuses(
    pool: &SqlitePool,
    player_ids: &[Uuid],
) -> RecoveryResult<HashMap<Uuid, PlayerMedicalStatus>> {
    if player_ids.is_empty() {
        return Ok(HashMap::new());
    }

    let rows = arlo_persistence::repositories::condition::player_injury_history::list_active_by_player_ids(
        pool,
        player_ids,
    )
    .await?;

    let mut rows_by_player: HashMap<Uuid, PlayerInjuryHistoryRow> =
        HashMap::with_capacity(rows.len());
    for row in rows {
        if let Ok(pid) = Uuid::parse_str(&row.player_id) {
            rows_by_player.entry(pid).or_insert(row);
        }
    }

    let mut results = HashMap::with_capacity(player_ids.len());
    for &player_id in player_ids {
        let status = resolve_status_from_row(player_id, rows_by_player.get(&player_id))?;
        results.insert(player_id, status);
    }

    Ok(results)
}
