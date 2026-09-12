use crate::error::PersistenceResult;
use crate::models::{
    MatchPlayerAvailabilityRow, MatchPlayerFoulByOriginRow, MatchPlayerFoulRow,
    MatchPlayerImpulseRow, MatchPlayerImpulseRunRow, MatchPlayerImpulseShiftByKindRow,
    MatchPlayerInjuryByBodyRegionRow, MatchPlayerInjuryRow, MatchPlayerKickFoulByDecisionRow,
    MatchPlayerKickFoulRow, MatchPlayerPhysicalRow, MatchPlayerPunishmentRow,
};
use crate::repositories;
use arlo_stats::{
    AggregatorRegistry, PlayerAvailabilityAggregator, PlayerFoulAggregator,
    PlayerImpulseAggregator, PlayerInjuryAggregator, PlayerKickFoulAggregator,
    PlayerPhysicalAggregator, PlayerPunishmentAggregator,
};
use sqlx::{Sqlite, Transaction};
use uuid::Uuid;

pub async fn persist_player_condition_stats(
    tx: &mut Transaction<'_, Sqlite>,
    match_id: Uuid,
    aggregators: &AggregatorRegistry,
) -> PersistenceResult<()> {
    if let Some(agg) = aggregators.get::<PlayerPhysicalAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchPlayerPhysicalRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_player_physical::insert_batch(tx, &rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerFoulAggregator>() {
        let mut foul_rows = Vec::new();
        let mut foul_by_origin_rows = Vec::new();
        for (pid, s) in agg.all_stats() {
            foul_rows.push(MatchPlayerFoulRow::from_stats(Uuid::new_v4(), match_id, s));
            for (origin, &count) in s.by_origin() {
                foul_by_origin_rows.push(MatchPlayerFoulByOriginRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *pid,
                    *origin,
                    count,
                ));
            }
        }
        repositories::match_player_fouls::insert_batch(tx, &foul_rows).await?;
        repositories::match_player_fouls::insert_by_origin_batch(tx, &foul_by_origin_rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerPunishmentAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchPlayerPunishmentRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_player_punishments::insert_batch(tx, &rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerAvailabilityAggregator>() {
        let rows: Vec<_> = agg
            .all_stats()
            .values()
            .map(|s| MatchPlayerAvailabilityRow::from_stats(Uuid::new_v4(), match_id, s))
            .collect();
        repositories::match_player_availability::insert_batch(tx, &rows).await?;
    }

    if let Some(agg) = aggregators.get::<PlayerKickFoulAggregator>() {
        let mut kf_rows = Vec::new();
        let mut kf_by_decision_rows = Vec::new();
        for (pid, s) in agg.all_stats() {
            kf_rows.push(MatchPlayerKickFoulRow::from_stats(Uuid::new_v4(), match_id, s));
            for (kind, &count) in s.decisions_by_kind() {
                kf_by_decision_rows.push(MatchPlayerKickFoulByDecisionRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *pid,
                    *kind,
                    count,
                ));
            }
        }
        repositories::match_player_kick_fouls::insert_batch(tx, &kf_rows).await?;
        repositories::match_player_kick_fouls::insert_by_decision_batch(
            tx,
            &kf_by_decision_rows,
        )
        .await?;
    }

    if let Some(agg) = aggregators.get::<PlayerInjuryAggregator>() {
        let mut inj_rows = Vec::new();
        let mut inj_by_body_region_rows = Vec::new();
        for (pid, s) in agg.all_stats() {
            inj_rows.push(MatchPlayerInjuryRow::from_stats(Uuid::new_v4(), match_id, s));
            for (region, &count) in s.by_body_region() {
                inj_by_body_region_rows.push(MatchPlayerInjuryByBodyRegionRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *pid,
                    *region,
                    count,
                ));
            }
        }
        repositories::match_player_injuries::insert_batch(tx, &inj_rows).await?;
        repositories::match_player_injuries::insert_by_body_region_batch(
            tx,
            &inj_by_body_region_rows,
        )
        .await?;
    }

    if let Some(agg) = aggregators.get::<PlayerImpulseAggregator>() {
        let mut impulse_rows = Vec::new();
        let mut impulse_shift_by_kind_rows = Vec::new();
        let mut impulse_run_rows = Vec::new();
        for (pid, s) in agg.all_player_stats() {
            impulse_rows.push(MatchPlayerImpulseRow::from_stats(Uuid::new_v4(), match_id, s));
            for (kind, &count) in s.shifts_by_kind() {
                impulse_shift_by_kind_rows.push(MatchPlayerImpulseShiftByKindRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *pid,
                    *kind,
                    count,
                ));
            }
            for (idx, run) in s.all_completed_and_active_runs().iter().enumerate() {
                impulse_run_rows.push(MatchPlayerImpulseRunRow::from_stats(
                    Uuid::new_v4(),
                    match_id,
                    *pid,
                    idx,
                    run,
                ));
            }
        }
        repositories::match_player_impulse::insert_batch(tx, &impulse_rows).await?;
        repositories::match_player_impulse::insert_shifts_by_kind_batch(
            tx,
            &impulse_shift_by_kind_rows,
        )
        .await?;
        repositories::match_player_impulse::insert_runs_batch(tx, &impulse_run_rows).await?;
    }

    Ok(())
}
