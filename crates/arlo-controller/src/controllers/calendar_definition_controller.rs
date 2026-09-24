use crate::dto::calendar::{CalendarMonthViewDto, CalendarSystemDto};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::calendar::month_view_builder::build_month_view;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_calendar_system(
    pool: &SqlitePool,
    id: Uuid,
) -> ControllerResult<Option<CalendarSystemDto>> {
    let catalog = get_or_load_calendar_catalog(pool).await?;
    Ok(catalog.get(&id).map(CalendarSystemDto::from))
}

pub async fn list_calendar_systems(pool: &SqlitePool) -> ControllerResult<Vec<CalendarSystemDto>> {
    let catalog = get_or_load_calendar_catalog(pool).await?;
    let mut systems: Vec<CalendarSystemDto> = catalog.all().map(CalendarSystemDto::from).collect();
    systems.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(systems)
}

pub async fn get_month_view(
    pool: &SqlitePool,
    calendar_system_id: Uuid,
    year: i64,
    month_order_index: u32,
) -> ControllerResult<CalendarMonthViewDto> {
    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = catalog.get(&calendar_system_id).ok_or_else(|| {
        ControllerError::NotFound(format!("Calendar system {} not found", calendar_system_id))
    })?;
    build_month_view(calendar, year, month_order_index)
}
