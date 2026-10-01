use crate::error::{PersistenceError, PersistenceResult};
use crate::models::MatchPlayerPerformanceRow;
use crate::repositories::match_player_performance;
use arlo_analytics::PlayerPerformanceAggregator;
use arlo_stats::AggregatorRegistry;
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_player_performance(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    aggregators: &AggregatorRegistry,
) -> PersistenceResult<()> {
    let Some(aggregator) = aggregators.get::<PlayerPerformanceAggregator>() else {
        return Err(PersistenceError::InvalidData(
            "player performance aggregator is missing from completed match".into(),
        ));
    };

    if !aggregator.is_finalized() {
        return Err(PersistenceError::InvalidData(
            "player performance aggregator is not finalized".into(),
        ));
    }

    let rows: Vec<_> = aggregator
        .all_player_snapshots()
        .iter()
        .map(|snapshot| MatchPlayerPerformanceRow::from_snapshot(Uuid::new_v4(), match_id, snapshot))
        .collect();

    match_player_performance::insert_batch(tx, &rows).await
}
