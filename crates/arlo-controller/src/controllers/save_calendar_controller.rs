use crate::domain::calendar::{CalendarDate, ResolvedCalendarDate};
use crate::dto::calendar::ResolvedCalendarDateDto;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::calendar::{date_advancer, date_encoder, date_resolver};
use arlo_persistence::models::calendar::SaveCalendarStateRow;
use arlo_persistence::repositories::calendar::save_calendar_state;
use sqlx::SqlitePool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn assign_calendar_to_save(
    pool: &SqlitePool,
    save_uuid: Uuid,
    calendar_system_id: Uuid,
    starting_resolved_date: ResolvedCalendarDate,
) -> ControllerResult<ResolvedCalendarDateDto> {
    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!("Calendar system {} not found", calendar_system_id))
    })?;

    let encoded_date = date_encoder::encode(calendar, &starting_resolved_date)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let assigned_at_unix_seconds = now.as_secs() as i64;

    let row = SaveCalendarStateRow::new(
        save_uuid,
        calendar_system_id,
        encoded_date.year(),
        encoded_date.day_of_year(),
        assigned_at_unix_seconds,
    );

    save_calendar_state::upsert(pool, &row).await?;

    let resolved = date_resolver::resolve(calendar, &encoded_date);
    Ok(ResolvedCalendarDateDto::from_domain(&resolved, calendar))
}

pub async fn get_current_date(
    pool: &SqlitePool,
    save_uuid: Uuid,
) -> ControllerResult<ResolvedCalendarDateDto> {
    let row = save_calendar_state::get_by_save_uuid(pool, save_uuid)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Save calendar state for save {} not found",
                save_uuid
            ))
        })?;

    let calendar_system_id = Uuid::parse_str(&row.calendar_system_id)?;
    let current_date = CalendarDate::new(row.current_year, row.current_day_of_year as u32);

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!("Calendar system {} not found", calendar_system_id))
    })?;

    let resolved = date_resolver::resolve(calendar, &current_date);
    Ok(ResolvedCalendarDateDto::from_domain(&resolved, calendar))
}

pub async fn advance_current_date(
    pool: &SqlitePool,
    save_uuid: Uuid,
    delta_days: i64,
) -> ControllerResult<ResolvedCalendarDateDto> {
    let row = save_calendar_state::get_by_save_uuid(pool, save_uuid)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Save calendar state for save {} not found",
                save_uuid
            ))
        })?;

    let calendar_system_id = Uuid::parse_str(&row.calendar_system_id)?;
    let current_date = CalendarDate::new(row.current_year, row.current_day_of_year as u32);

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!("Calendar system {} not found", calendar_system_id))
    })?;

    let new_date = date_advancer::advance(calendar, &current_date, delta_days);
    let updated_row = SaveCalendarStateRow::new(
        save_uuid,
        calendar_system_id,
        new_date.year(),
        new_date.day_of_year(),
        row.assigned_at_unix_seconds,
    );

    save_calendar_state::upsert(pool, &updated_row).await?;

    let resolved = date_resolver::resolve(calendar, &new_date);
    Ok(ResolvedCalendarDateDto::from_domain(&resolved, calendar))
}
