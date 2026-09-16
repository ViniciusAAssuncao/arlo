use crate::error::DbResult;
use crate::models::CalendarSystemRow;
use crate::repositories::fetch::{fetch_all, fetch_optional_by_param};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<CalendarSystemRow>> {
    let row = fetch_optional_by_param::<CalendarSystemRow>(
        pool,
        "SELECT id, name, description, leap_units_per_cycle, cycle_length_years, cycle_reference_year, days_per_occurrence, intercalation_placement_kind FROM calendar_systems WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    Ok(row)
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<CalendarSystemRow>> {
    let rows = fetch_all::<CalendarSystemRow>(
        pool,
        "SELECT id, name, description, leap_units_per_cycle, cycle_length_years, cycle_reference_year, days_per_occurrence, intercalation_placement_kind FROM calendar_systems ORDER BY name ASC",
    )
    .await?;

    Ok(rows)
}