use crate::error::PersistenceResult;
use crate::models::{
    MatchManagerDecisionRow, MatchManagerPlayCallByCategoryRow,
    MatchManagerSubstitutionByReasonRow, MatchPlayCallOutcomeRow, MatchRefereePerformanceRow,
};
use crate::repositories;
use arlo_engine::MatchState;
use arlo_stats::{
    AggregatorRegistry, ManagerDecisionAggregator, PlayCallOutcomeAggregator,
    RefereeStatsAggregator,
};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_manager_stats(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    aggregators: &AggregatorRegistry,
) -> PersistenceResult<()> {
    if let Some(agg) = aggregators.get::<ManagerDecisionAggregator>() {
        let mut decision_rows = Vec::new();
        let mut subs_by_reason_rows = Vec::new();
        let mut calls_by_category_rows = Vec::new();
        for (tid, log) in agg.all_logs() {
            decision_rows.push(MatchManagerDecisionRow::from_stats(
                Uuid::new_v4(),
                match_id,
                *tid,
                log,
            ));
            for (reason, &count) in log.substitutions_by_reason() {
                subs_by_reason_rows.push(MatchManagerSubstitutionByReasonRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *tid,
                    *reason,
                    count,
                ));
            }
            for (cat, &count) in log.play_calls_by_category() {
                calls_by_category_rows.push(MatchManagerPlayCallByCategoryRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *tid,
                    *cat,
                    count,
                ));
            }
        }
        repositories::match_manager_decisions::insert_batch(tx, &decision_rows).await?;
        repositories::match_manager_decisions::insert_substitutions_by_reason_batch(
            tx,
            &subs_by_reason_rows,
        )
        .await?;
        repositories::match_manager_decisions::insert_play_calls_by_category_batch(
            tx,
            &calls_by_category_rows,
        )
        .await?;
    }

    if let Some(agg) = aggregators.get::<PlayCallOutcomeAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchPlayCallOutcomeRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_play_call_outcomes::insert_batch(tx, &rows).await?;
    }

    Ok(())
}

pub async fn persist_referee_stats(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    state: &MatchState,
    aggregators: &AggregatorRegistry,
) -> PersistenceResult<()> {
    if let Some(agg) = aggregators.get::<RefereeStatsAggregator>() {
        let stats = agg.stats();
        let head_row = MatchRefereePerformanceRow::for_head_referee(
            Uuid::new_v4(),
            match_id,
            state.head_referee().id(),
            &stats,
        );
        let peace_row = MatchRefereePerformanceRow::for_peace_referee(
            Uuid::new_v4(),
            match_id,
            state.peace_referee().id(),
            &stats,
        );
        repositories::match_referee_performance::insert_batch(tx, &[head_row, peace_row]).await?;
    }

    Ok(())
}
