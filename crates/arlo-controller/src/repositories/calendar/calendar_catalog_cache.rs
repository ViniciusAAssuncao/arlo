use crate::domain::calendar::CalendarCatalog;
use crate::error::ControllerResult;
use crate::repositories::calendar::calendar_system_repository;
use sqlx::SqlitePool;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;

static CALENDAR_CATALOG: LazyLock<RwLock<Option<Arc<CalendarCatalog>>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn get_or_load_calendar_catalog(
    pool: &SqlitePool,
) -> ControllerResult<Arc<CalendarCatalog>> {
    {
        let read_guard = CALENDAR_CATALOG.read().await;
        if let Some(catalog) = read_guard.as_ref() {
            return Ok(catalog.clone());
        }
    }

    let systems = calendar_system_repository::list_all(pool).await?;
    let catalog = Arc::new(CalendarCatalog::new(systems));

    let mut write_guard = CALENDAR_CATALOG.write().await;
    if let Some(existing) = write_guard.as_ref() {
        return Ok(existing.clone());
    }

    *write_guard = Some(catalog.clone());
    Ok(catalog)
}

pub async fn refresh(pool: &SqlitePool) -> ControllerResult<Arc<CalendarCatalog>> {
    let systems = calendar_system_repository::list_all(pool).await?;
    let catalog = Arc::new(CalendarCatalog::new(systems));

    let mut write_guard = CALENDAR_CATALOG.write().await;
    *write_guard = Some(catalog.clone());
    Ok(catalog)
}

pub async fn invalidate() {
    let mut write_guard = CALENDAR_CATALOG.write().await;
    *write_guard = None;
}
