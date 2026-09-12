use crate::error::PersistenceResult;
use crate::models::{
    MatchPlayerArtrineDecisionByKindRow, MatchPlayerArtrineDecisionRow, MatchPlayerAssistRow,
    MatchPlayerDrivesRow, MatchPlayerDuelByKindRow, MatchPlayerDuelRow, MatchPlayerReceivingRow,
    MatchPlayerScoringAttemptByPostRow, MatchPlayerScoringAttemptRow, MatchPlayerTouchesRow,
};
use crate::repositories;
use arlo_stats::{
    AggregatorRegistry, PlayerArtrineDecisionAggregator, PlayerAssistsAggregator,
    PlayerDrivesAggregator, PlayerDuelAggregator, PlayerReceivingAggregator,
    PlayerScoringAttemptsAggregator, PlayerTouchesAggregator,
};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_player_action_stats(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    aggregators: &AggregatorRegistry,
) -> PersistenceResult<()> {
    if let Some(agg) = aggregators.get::<PlayerTouchesAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchPlayerTouchesRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_player_touches::insert_batch(tx, &rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerDrivesAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchPlayerDrivesRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_player_drives::insert_batch(tx, &rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerDuelAggregator>() {
        let mut duel_rows = Vec::new();
        let mut duel_by_kind_rows = Vec::new();
        for (pid, s) in agg.all_stats() {
            duel_rows.push(MatchPlayerDuelRow::from_stats(Uuid::new_v4(), match_id, s));
            for (kind, ks) in s.by_kind() {
                duel_by_kind_rows.push(MatchPlayerDuelByKindRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *pid,
                    *kind,
                    ks,
                ));
            }
        }
        repositories::match_player_duels::insert_batch(tx, &duel_rows).await?;
        repositories::match_player_duels::insert_by_kind_batch(tx, &duel_by_kind_rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerReceivingAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchPlayerReceivingRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_player_receiving::insert_batch(tx, &rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerScoringAttemptsAggregator>() {
        let mut score_attempt_rows = Vec::new();
        let mut score_attempt_by_post_rows = Vec::new();
        for (pid, s) in agg.all_stats() {
            score_attempt_rows.push(MatchPlayerScoringAttemptRow::from_stats(
                Uuid::new_v4(),
                match_id,
                s,
            ));
            for (post, &(att, conv)) in s.by_post() {
                score_attempt_by_post_rows.push(MatchPlayerScoringAttemptByPostRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *pid,
                    *post,
                    att,
                    conv,
                ));
            }
        }
        repositories::match_player_scoring_attempts::insert_batch(tx, &score_attempt_rows).await?;
        repositories::match_player_scoring_attempts::insert_by_post_batch(
            tx,
            &score_attempt_by_post_rows,
        )
        .await?;
    }

    if let Some(agg) = aggregators.get::<PlayerAssistsAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchPlayerAssistRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_player_assists::insert_batch(tx, &rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerArtrineDecisionAggregator>() {
        let mut decision_rows = Vec::new();
        let mut decision_by_kind_rows = Vec::new();
        for (pid, s) in agg.all_stats() {
            decision_rows.push(MatchPlayerArtrineDecisionRow::from_stats(
                Uuid::new_v4(),
                match_id,
                s,
            ));
            for (kind, ks) in s.by_kind() {
                decision_by_kind_rows.push(MatchPlayerArtrineDecisionByKindRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *pid,
                    *kind,
                    ks,
                ));
            }
        }
        repositories::match_player_artrine_decisions::insert_batch(tx, &decision_rows).await?;
        repositories::match_player_artrine_decisions::insert_by_kind_batch(
            tx,
            &decision_by_kind_rows,
        )
        .await?;
    }

    Ok(())
}
