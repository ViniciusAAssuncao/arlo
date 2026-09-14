use crate::domain::season::{Fixture, StandingsEntry};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::season::persistence::persist_generated_stage_schedule;
use crate::services::season::stage::stage_schedule_generator::{
    generate_stage_schedule_from_seeds, GeneratedStageSchedule,
};
use crate::services::season::stage::stage_transition_evaluator::evaluate_stage_transition;
use crate::services::season::standings::standings_calculator::calculate_standings;
use crate::services::season::standings::tie_break_resolver::sort_standings;
use arlo_domain::StandingsPointsPolicy;
use sqlx::SqlitePool;
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

    let points_policy = StandingsPointsPolicy::default_policy();
    let unsorted_standings: Vec<StandingsEntry> =
        calculate_standings(participating_team_ids, completed_fixtures, &points_policy);
    let sorted_standings = sort_standings(unsorted_standings, &[]);

    let seeds = evaluate_stage_transition(&sorted_standings, &target_stage_def.entry_rule())?;

    let schedule = generate_stage_schedule_from_seeds(
        calendar,
        &config_arc,
        target_stage_def,
        season_instance_id,
        stage_instance_id,
        &seeds,
        reference_year,
        start_round_index,
    )?;

    persist_generated_stage_schedule(pool, &schedule).await?;

    Ok(schedule)
}