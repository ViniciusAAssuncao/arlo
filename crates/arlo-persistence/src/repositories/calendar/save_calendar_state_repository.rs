use crate::error::PersistenceResult;
use crate::models::calendar::SaveCalendarStateRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_save_uuid(
    pool: &SqlitePool,
    save_uuid: Uuid,
) -> PersistenceResult<Option<SaveCalendarStateRow>> {
    let row = sqlx::query_as::<_, SaveCalendarStateRow>(
        "SELECT save_uuid, calendar_system_id, current_year, current_day_of_year, assigned_at_unix_seconds FROM save_calendar_states WHERE save_uuid = ?",
    )
    .bind(save_uuid.to_string())
    .fetch_optional(pool)
    .await?;

    Ok(row)
}

pub async fn upsert(pool: &SqlitePool, row: &SaveCalendarStateRow) -> PersistenceResult<()> {
    sqlx::query(
        "INSERT INTO save_calendar_states (save_uuid, calendar_system_id, current_year, current_day_of_year, assigned_at_unix_seconds) VALUES (?, ?, ?, ?, ?) ON CONFLICT(save_uuid) DO UPDATE SET calendar_system_id = excluded.calendar_system_id, current_year = excluded.current_year, current_day_of_year = excluded.current_day_of_year, assigned_at_unix_seconds = excluded.assigned_at_unix_seconds",
    )
    .bind(&row.save_uuid)
    .bind(&row.calendar_system_id)
    .bind(row.current_year)
    .bind(row.current_day_of_year)
    .bind(row.assigned_at_unix_seconds)
    .execute(pool)
    .await?;

    Ok(())
}
