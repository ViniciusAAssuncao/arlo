use crate::domain::calendar::{ResolvedCalendarDate, SaveCalendarState};
use crate::dto::calendar::ResolvedCalendarDateDto;
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::calendar::save_calendar_state_repository;
use crate::services::calendar::{date_advancer, date_encoder, date_resolver};
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
        ControllerError::NotFound(format!(
            "Calendar system {} not found",
            calendar_system_id
        ))
    })?;

    let encoded_date = date_encoder::encode(calendar, &starting_resolved_date)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let assigned_at_unix_seconds = now.as_secs() as i64;

    let state = SaveCalendarState::new(
        save_uuid,
        calendar_system_id,
        encoded_date,
        assigned_at_unix_seconds,
    );

    save_calendar_state_repository::upsert(pool, &state).await?;

    let resolved = date_resolver::resolve(calendar, &encoded_date);
    Ok(ResolvedCalendarDateDto::from_domain(&resolved, calendar))
}

pub async fn get_current_date(
    pool: &SqlitePool,
    save_uuid: Uuid,
) -> ControllerResult<ResolvedCalendarDateDto> {
    let state = save_calendar_state_repository::get_by_save_uuid(pool, save_uuid)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Save calendar state for save {} not found",
                save_uuid
            ))
        })?;

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog
        .get(&state.calendar_system_id())
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Calendar system {} not found",
                state.calendar_system_id()
            ))
        })?;

    let resolved = date_resolver::resolve(calendar, &state.current_date());
    Ok(ResolvedCalendarDateDto::from_domain(&resolved, calendar))
}

pub async fn advance_current_date(
    pool: &SqlitePool,
    save_uuid: Uuid,
    delta_days: i64,
) -> ControllerResult<ResolvedCalendarDateDto> {
    let state = save_calendar_state_repository::get_by_save_uuid(pool, save_uuid)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Save calendar state for save {} not found",
                save_uuid
            ))
        })?;

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog
        .get(&state.calendar_system_id())
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Calendar system {} not found",
                state.calendar_system_id()
            ))
        })?;

    let new_date = date_advancer::advance(calendar, &state.current_date(), delta_days);
    let updated_state = SaveCalendarState::new(
        state.save_uuid(),
        state.calendar_system_id(),
        new_date,
        state.assigned_at_unix_seconds(),
    );

    save_calendar_state_repository::upsert(pool, &updated_state).await?;

    let resolved = date_resolver::resolve(calendar, &new_date);
    Ok(ResolvedCalendarDateDto::from_domain(&resolved, calendar))
}
