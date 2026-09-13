use crate::domain::calendar::CalendarCatalog;
use crate::error::ControllerResult;
use crate::repositories::calendar::calendar_system_repository;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::OnceCell;

static CALENDAR_CATALOG: OnceCell<Arc<CalendarCatalog>> = OnceCell::const_new();

pub async fn get_or_load_calendar_catalog(
    pool: &SqlitePool,
) -> ControllerResult<Arc<CalendarCatalog>> {
    CALENDAR_CATALOG
        .get_or_try_init(|| async {
            let systems = calendar_system_repository::list_all(pool).await?;
            Ok(Arc::new(CalendarCatalog::new(systems)))
        })
        .await
        .cloned()
}