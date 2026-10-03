use crate::domain::calendar::CalendarDate;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::calendar::date_advancer;
use crate::services::day_simulation::day_progress;
use crate::services::event_scheduling::event_dispatcher::{
    dispatch_due_events, DispatchedEventResult,
};
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::season::matchday::matchday_orchestrator;
use crate::services::season::power_ranking::maybe_publish_power_rankings;
use arlo_persistence::repositories::calendar::save_calendar_state;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct DayAdvancementResult {
    pub save_uuid: Uuid,
    pub previous_date: CalendarDate,
    pub current_date: CalendarDate,
    pub dispatched_events: Vec<DispatchedEventResult>,
    pub matches_played_count: u32,
}

impl DayAdvancementResult {
    pub fn new(
        save_uuid: Uuid,
        previous_date: CalendarDate,
        current_date: CalendarDate,
        dispatched_events: Vec<DispatchedEventResult>,
        matches_played_count: u32,
    ) -> Self {
        Self {
            save_uuid,
            previous_date,
            current_date,
            dispatched_events,
            matches_played_count,
        }
    }
}

pub async fn run_day_advancement(
    pool: &SqlitePool,
    save_uuid: Uuid,
    trigger_store: &Arc<PendingTriggerStore>,
) -> ControllerResult<DayAdvancementResult> {
    let row = save_calendar_state::get_by_save_uuid(pool, save_uuid)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Save calendar state for save {} not found",
                save_uuid
            ))
        })?;

    let calendar_system_id = Uuid::parse_str(&row.calendar_system_id)?;
    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!("Calendar system {} not found", calendar_system_id))
    })?;

    let previous_date = CalendarDate::new(row.current_year, row.current_day_of_year as u32);
    let current_date = date_advancer::advance(calendar, &previous_date, 1);

    let progress = day_progress::load_or_start(pool, save_uuid, &previous_date, &current_date).await?;
    let mut phase = progress.phase;
    let mut matches_played_count = progress.matches_played_count as u32;
    if phase == "Started" {
        let plan = arlo_recovery::orchestration::prepare_all_players_one_day(
            pool, current_date.year(), current_date.day_of_year(),
        ).await.map_err(|error| ControllerError::InvalidData(error.to_string()))?;
        let mut tx = pool.begin().await?;
        arlo_recovery::orchestration::persist_daily_condition_plan(&mut tx, plan)
            .await.map_err(|error| ControllerError::InvalidData(error.to_string()))?;
        day_progress::advance_phase_with_tx(&mut tx, save_uuid, "Started", "RecoveryDone").await?;
        tx.commit().await?;
        phase = "RecoveryDone".into();
    }

    let mut dispatched_events = Vec::new();
    if phase == "RecoveryDone" {
        if trigger_store.has_due(&current_date).await {
            day_progress::advance_phase(pool, save_uuid, "RecoveryDone", "EventsRunning").await?;
            dispatched_events = dispatch_due_events(
                pool, Arc::clone(trigger_store), calendar_system_id, &current_date,
            ).await?;
            day_progress::advance_phase(pool, save_uuid, "EventsRunning", "EventsDone").await?;
        } else {
            day_progress::advance_phase(pool, save_uuid, "RecoveryDone", "EventsDone").await?;
        }
        phase = "EventsDone".into();
    }
    if phase == "EventsRunning" {
        return Err(ControllerError::InvalidData(
            "A calendar event was interrupted; automatic replay could duplicate its effects".into(),
        ));
    }

    if phase == "EventsDone" {
        matches_played_count = matchday_orchestrator::run_due_matches(
            pool, save_uuid, current_date.year(), current_date.day_of_year(),
        ).await?;
        phase = "MatchesDone".into();
    }
    if phase != "MatchesDone" {
        return Err(ControllerError::InvalidData("Unknown day progress phase".into()));
    }
    maybe_publish_power_rankings(pool, calendar, current_date).await?;
    day_progress::finish(pool, save_uuid, &current_date).await?;

    Ok(DayAdvancementResult::new(
        save_uuid,
        previous_date,
        current_date,
        dispatched_events,
        matches_played_count,
    ))
}
