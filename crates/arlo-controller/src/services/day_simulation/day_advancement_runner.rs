use crate::domain::calendar::{ CalendarDate, SaveCalendarState };
use crate::error::{ ControllerError, ControllerResult };
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::calendar::save_calendar_state_repository;
use crate::services::calendar::date_advancer;
use crate::services::event_scheduling::event_dispatcher::{
    dispatch_due_events,
    DispatchedEventResult,
};
use crate::services::event_scheduling::pending_trigger_store::PendingTriggerStore;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug)]
pub struct DayAdvancementResult {
    pub save_uuid: Uuid,
    pub previous_date: CalendarDate,
    pub current_date: CalendarDate,
    pub dispatched_events: Vec<DispatchedEventResult>,
}

impl DayAdvancementResult {
    pub fn new(
        save_uuid: Uuid,
        previous_date: CalendarDate,
        current_date: CalendarDate,
        dispatched_events: Vec<DispatchedEventResult>
    ) -> Self {
        Self {
            save_uuid,
            previous_date,
            current_date,
            dispatched_events,
        }
    }
}

pub async fn run_day_advancement(
    pool: &SqlitePool,
    save_uuid: Uuid,
    trigger_store: &PendingTriggerStore
) -> ControllerResult<DayAdvancementResult> {
    let state = save_calendar_state_repository
        ::get_by_save_uuid(pool, save_uuid).await?
        .ok_or_else(|| {
            ControllerError::NotFound(
                format!("Save calendar state for save {} not found", save_uuid)
            )
        })?;

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog
        .get(&state.calendar_system_id())
        .ok_or_else(|| {
            ControllerError::NotFound(
                format!("Calendar system {} not found", state.calendar_system_id())
            )
        })?;

    let previous_date = state.current_date();
    let current_date = date_advancer::advance(calendar, &previous_date, 1);

    let updated_state = SaveCalendarState::new(
        state.save_uuid(),
        state.calendar_system_id(),
        current_date,
        state.assigned_at_unix_seconds()
    );

    save_calendar_state_repository::upsert(pool, &updated_state).await?;

    let dispatched_events = dispatch_due_events(
        pool,
        trigger_store,
        state.calendar_system_id(),
        &current_date
    ).await?;

    Ok(DayAdvancementResult::new(save_uuid, previous_date, current_date, dispatched_events))
}
