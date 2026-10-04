use crate::error::{PersistenceError, PersistenceResult};
use crate::models::{MatchPlayerPerformanceCategoryRow, MatchPlayerPerformanceRow};
use crate::repositories::{match_player_performance, match_player_performance_category};
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

    let snapshots = aggregator.all_player_snapshots();

    let rows: Vec<_> = snapshots
        .iter()
        .map(|snapshot| {
            MatchPlayerPerformanceRow::from_snapshot(Uuid::new_v4(), match_id, snapshot)
        })
        .collect();

    let category_rows: Vec<_> = snapshots
        .iter()
        .flat_map(|snapshot| {
            snapshot
                .diagnostics()
                .category_contributions()
                .iter()
                .map(move |contribution| {
                    MatchPlayerPerformanceCategoryRow::from_contribution(
                        Uuid::new_v4(),
                        match_id,
                        snapshot,
                        contribution,
                    )
                })
        })
        .collect();

    match_player_performance::insert_batch(tx, &rows).await?;
    for player in aggregator.players().values() {
        for (position_code, seconds_played) in player.position_seconds() {
            sqlx::query("INSERT INTO match_player_position_seconds (match_id, player_id, position_code, seconds_played) VALUES (?, ?, ?, ?)")
                .bind(match_id.to_string())
                .bind(player.player_id().to_string())
                .bind(position_code)
                .bind(seconds_played)
                .execute(&mut **tx)
                .await?;
        }
    }
    match_player_performance_category::insert_batch(tx, &category_rows).await
}
