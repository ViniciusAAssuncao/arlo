use crate::domain::calendar::CalendarDate;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::calendar::date_advancer;
use crate::services::event_scheduling::event_dispatcher::{
    dispatch_due_events, DispatchedEventResult,
};
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use crate::services::season::matchday::matchday_orchestrator;
use arlo_persistence::models::calendar::SaveCalendarStateRow;
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
        ControllerError::NotFound(format!(
            "Calendar system {} not found",
            calendar_system_id
        ))
    })?;

    let previous_date = CalendarDate::new(row.current_year, row.current_day_of_year as u32);
    let current_date = date_advancer::advance(calendar, &previous_date, 1);

    let updated_row = SaveCalendarStateRow::new(
        save_uuid,
        calendar_system_id,
        current_date.year(),
        current_date.day_of_year(),
        row.assigned_at_unix_seconds,
    );

    save_calendar_state::upsert(pool, &updated_row).await?;

    let dispatched_events = dispatch_due_events(
        pool,
        Arc::clone(trigger_store),
        calendar_system_id,
        &current_date,
    )
    .await?;

    let matches_played_count = matchday_orchestrator::run_due_matches(
        pool,
        current_date.year(),
        current_date.day_of_year(),
    )
    .await?;

    Ok(DayAdvancementResult::new(
        save_uuid,
        previous_date,
        current_date,
        dispatched_events,
        matches_played_count,
    ))
}