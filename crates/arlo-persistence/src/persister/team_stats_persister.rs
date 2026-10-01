use crate::error::PersistenceResult;
use crate::models::{MatchTeamImpulseRow, MatchTeamImpulseRunRow, MatchTeamPossessionRow};
use crate::repositories;
use arlo_stats::{AggregatorRegistry, PlayerImpulseAggregator, TeamPossessionAggregator};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_team_stats(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    aggregators: &AggregatorRegistry,
) -> PersistenceResult<()> {
    if let Some(agg) = aggregators.get::<TeamPossessionAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchTeamPossessionRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_team_possession::insert_batch(tx, &rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerImpulseAggregator>() {
        let mut team_impulse_rows = Vec::new();
        let mut team_impulse_run_rows = Vec::new();
        for (tid, s) in agg.all_team_stats() {
            team_impulse_rows.push(MatchTeamImpulseRow::from_stats(Uuid::new_v4(), match_id, s));
            for (idx, run) in s.all_completed_and_active_runs().iter().enumerate() {
                team_impulse_run_rows.push(MatchTeamImpulseRunRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *tid,
                    idx,
                    run,
                ));
            }
        }
        repositories::match_team_impulse::insert_batch(tx, &team_impulse_rows).await?;
        repositories::match_team_impulse::insert_runs_batch(tx, &team_impulse_run_rows).await?;
    }

    Ok(())
}
