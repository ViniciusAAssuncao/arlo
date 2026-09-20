use crate::domain::calendar::CalendarSystem;
use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::domain::season::{KnockoutTie, SeasonStageInstance, StageStatus};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::collective_agreement::collective_agreement_catalog_cache::get_or_load_collective_agreement_catalog;
use crate::services::calendar::{resolve_collective_agreement_windows, skip_forward_past_blackout};
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::event_scheduling::stage_completion_date_calculator::calculate_stage_completion_date;
use crate::services::season::persistence::{
    load_stage_knockout_ties, map_row_to_fixture, persist_generated_stage_schedule,
};
use crate::services::season::stage::knockout_bracket_progress_detector::{
    detect_knockout_bracket_progress, KnockoutBracketProgress,
};
use crate::services::season::stage::knockout_round_resolver::resolve_tie_winner;
use crate::services::season::stage::next_knockout_round_generator::generate_next_knockout_round;
use crate::services::season::stage::stage_schedule_generator::GeneratedStageSchedule;
use arlo_domain::{LeagueCalendarConfig, StageDefinition, StageType};
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn advance_knockout_round(
    pool: &SqlitePool,
    competition_id: Uuid,
    _calendar_system_id: Uuid,
    season_instance_id: Uuid,
    stage_instance_id: Uuid,
    stage_def: &StageDefinition,
    config: &LeagueCalendarConfig,
    calendar: &CalendarSystem,
    trigger_store: Arc<PendingTriggerStore>,
    _reference_year: i64,
) -> ControllerResult<Option<GeneratedStageSchedule>> {
    let leg_format = match stage_def.knockout_leg_format() {
        Some(fmt) => fmt,
        None => return Ok(None),
    };

    let ties = load_stage_knockout_ties(pool, stage_instance_id).await?;
    if ties.is_empty() {
        return Ok(None);
    }

    let fixture_rows = arlo_persistence::repositories::season::fixtures::list_by_stage_id(
        pool,
        stage_instance_id,
    )
    .await?;

    let mut fixtures = Vec::with_capacity(fixture_rows.len());
    for row in &fixture_rows {
        fixtures.push(map_row_to_fixture(row)?);
    }

    let mut updated_ties = Vec::with_capacity(ties.len());
    let mut tx = pool.begin().await?;

    for tie in ties {
        if tie.aggregate_winner_team_id().is_none() {
            if let Some(winner_id) =
                resolve_tie_winner(&tie, &fixtures, config.tie_break_criteria())
            {
                arlo_persistence::repositories::season::knockout_ties::update_winner(
                    &mut tx,
                    tie.id(),
                    Some(winner_id),
                    tie.leg_two_fixture_id(),
                )
                .await?;

                updated_ties.push(KnockoutTie::new(
                    tie.id(),
                    tie.season_stage_id(),
                    tie.round_index(),
                    tie.tie_index(),
                    tie.high_seed(),
                    tie.low_seed(),
                    tie.leg_one_fixture_id(),
                    tie.leg_two_fixture_id(),
                    Some(winner_id),
                ));
            } else {
                updated_ties.push(tie);
            }
        } else {
            updated_ties.push(tie);
        }
    }

    tx.commit().await?;

    let progress = detect_knockout_bracket_progress(
        &updated_ties,
        &fixtures,
        config.tie_break_criteria(),
    );

    match progress {
        KnockoutBracketProgress::RoundCompleteNeedsNextRound {
            winner_seeds, ..
        } => {
            let round_completion_date = calculate_stage_completion_date(calendar, &fixtures)
                .ok_or_else(|| ControllerError::Validation("No fixtures found to calculate completion date".to_string()))?;

            let ca_catalog = get_or_load_collective_agreement_catalog(pool).await?;
            let mut blackout_windows = Vec::new();
            let years = (round_completion_date.year() - 1)..=(round_completion_date.year() + 1);

            for ca_id in config.collective_agreement_ids() {
                if let Some(agreement) = ca_catalog.get(ca_id) {
                    let windows = resolve_collective_agreement_windows(
                        calendar,
                        agreement,
                        years.clone(),
                    )?;
                    blackout_windows.extend(windows);
                }
            }

            let anchor_date = skip_forward_past_blackout(calendar, &round_completion_date, &blackout_windows);

            let generated = generate_next_knockout_round(
                calendar,
                config.timing(),
                anchor_date,
                stage_instance_id,
                &fixtures,
                &winner_seeds,
                leg_format,
            )?;

            let stage_instance = SeasonStageInstance::new(
                stage_instance_id,
                season_instance_id,
                stage_def.stage_order_index(),
                StageType::KnockoutBracket,
                StageStatus::Active,
            );

            let schedule = GeneratedStageSchedule {
                stage_instance,
                fixtures: generated.fixtures,
                knockout_ties: generated.ties,
            };

            persist_generated_stage_schedule(pool, &schedule).await?;

            if let Some(completion_date) =
                calculate_stage_completion_date(calendar, &schedule.fixtures)
            {
                trigger_store
                    .insert(PendingTrigger::new(
                        completion_date,
                        competition_id,
                        TriggerKind::StageTransitionCheckDue,
                    ))
                    .await;
            }

            if let Some(first_date) = schedule.fixtures.iter().map(|f| f.scheduled_date()).min() {
                trigger_store
                    .insert(PendingTrigger::new(
                        first_date,
                        competition_id,
                        TriggerKind::ConflictScanDue,
                    ))
                    .await;
            }

            Ok(Some(schedule))
        }
        KnockoutBracketProgress::RoundIncomplete { .. }
        | KnockoutBracketProgress::BracketComplete { .. } => Ok(None),
    }
}