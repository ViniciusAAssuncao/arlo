use crate::error::DbResult;
use crate::models::CalendarMonthRow;
use crate::repositories::fetch::fetch_all_by_param;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn list_by_calendar_system_id(
    pool: &SqlitePool,
    calendar_system_id: Uuid,
) -> DbResult<Vec<CalendarMonthRow>> {
    let rows = fetch_all_by_param::<CalendarMonthRow>(
        pool,
        "SELECT id, calendar_system_id, order_index, name, day_count FROM calendar_months WHERE calendar_system_id = ? ORDER BY order_index ASC",
        &calendar_system_id.to_string(),
    )
    .await?;

    Ok(rows)
}