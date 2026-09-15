use crate::dto::calendar::CalendarSystemDto;
use crate::error::ControllerResult;
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
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
    let mut systems: Vec<CalendarSystemDto> =
        catalog.all().map(CalendarSystemDto::from).collect();
    systems.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(systems)
}
