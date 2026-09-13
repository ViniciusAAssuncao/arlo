use crate::domain::calendar::SaveCalendarState;
use crate::error::ControllerResult;
use crate::repositories::calendar::models::SaveCalendarStateRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_save_uuid(
    pool: &SqlitePool,
    save_uuid: Uuid,
) -> ControllerResult<Option<SaveCalendarState>> {
    let row = sqlx::query_as::<_, SaveCalendarStateRow>(
        "SELECT save_uuid, calendar_system_id, current_year, current_day_of_year, assigned_at_unix_seconds FROM save_calendar_states WHERE save_uuid = ?",
    )
    .bind(save_uuid.to_string())
    .fetch_optional(pool)
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn upsert(pool: &SqlitePool, state: &SaveCalendarState) -> ControllerResult<()> {
    sqlx::query(
        "INSERT INTO save_calendar_states (save_uuid, calendar_system_id, current_year, current_day_of_year, assigned_at_unix_seconds) VALUES (?, ?, ?, ?, ?) ON CONFLICT(save_uuid) DO UPDATE SET calendar_system_id = excluded.calendar_system_id, current_year = excluded.current_year, current_day_of_year = excluded.current_day_of_year, assigned_at_unix_seconds = excluded.assigned_at_unix_seconds",
    )
    .bind(state.save_uuid().to_string())
    .bind(state.calendar_system_id().to_string())
    .bind(state.current_date().year())
    .bind(state.current_date().day_of_year() as i32)
    .bind(state.assigned_at_unix_seconds())
    .execute(pool)
    .await?;

    Ok(())
}