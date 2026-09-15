use crate::domain::event_scheduling::{PendingTrigger, TriggerKind};
use crate::domain::season::Fixture;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::event_scheduling::stage_completion_date_calculator::calculate_stage_completion_date;
use crate::services::season::persistence::persist_generated_stage_schedule;
use crate::services::season::stage::external_qualification_resolver::resolve_external_winners;
use crate::services::season::stage::stage_schedule_generator::{
    generate_stage_schedule_from_seeds, GeneratedStageSchedule,
};
use crate::services::season::stage::stage_transition_evaluator::evaluate_stage_transition;
use crate::services::season::standings::random_tiebreak_resolver::seed_from_uuid;
use crate::services::season::standings::standings_pipeline;
use crate::services::season::venue::assign_neutral_venues_to_fixtures;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn handle_stage_transition(
    pool: &SqlitePool,
    competition_id: Uuid,
    calendar_system_id: Uuid,
    season_instance_id: Uuid,
    stage_instance_id: Uuid,
    target_stage_order_index: u32,
    completed_fixtures: &[Fixture],
    participating_team_ids: &[Uuid],
    reference_year: i64,
    start_round_index: u32,
    trigger_store: Arc<PendingTriggerStore>,
) -> ControllerResult<GeneratedStageSchedule> {
    let config_arc = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!(
            "Calendar system {} not found",
            calendar_system_id
        ))
    })?;

    let target_stage_def = config_arc
        .stages()
        .iter()
        .find(|s| s.stage_order_index() == target_stage_order_index)
        .ok_or_else(|| {
            ControllerError::Validation(format!(
                "Stage definition with order_index {} not found",
                target_stage_order_index
            ))
        })?;

    let external_winners =
        resolve_external_winners(pool, target_stage_def.entry_rule()).await?;

    let seed = seed_from_uuid(stage_instance_id);
    let sorted_standings = standings_pipeline::calculate_and_rank_standings(
        participating_team_ids,
        completed_fixtures,
        config_arc.spa_scoring_policy(),
        config_arc.qta_weighting_policy(),
        config_arc.tie_break_criteria(),
        seed,
    );

    let seeds = evaluate_stage_transition(
        &sorted_standings,
        target_stage_def.entry_rule(),
        config_arc.groups(),
        &external_winners,
    )?;

    let mut schedule = generate_stage_schedule_from_seeds(
        calendar,
        &config_arc,
        target_stage_def,
        season_instance_id,
        stage_instance_id,
        &seeds,
        reference_year,
        start_round_index,
    )?;

    assign_neutral_venues_to_fixtures(pool, competition_id, &mut schedule.fixtures).await?;

    persist_generated_stage_schedule(pool, &schedule).await?;

    if let Some(completion_date) = calculate_stage_completion_date(calendar, &schedule.fixtures) {
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

    Ok(schedule)
}