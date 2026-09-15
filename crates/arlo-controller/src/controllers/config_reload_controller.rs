use crate::error::ControllerResult;
use crate::repositories::calendar::calendar_catalog_cache;
use crate::repositories::league_calendar::league_calendar_config_cache;
use crate::repositories::season::standings_cache;
use crate::repositories::venue::eligible_venue_cache;
use sqlx::SqlitePool;

pub async fn reload_all_caches(pool: &SqlitePool) -> ControllerResult<()> {
    calendar_catalog_cache::refresh(pool).await?;
    league_calendar_config_cache::refresh_all(pool).await?;
    eligible_venue_cache::refresh_all(pool).await?;
    standings_cache::invalidate_all().await;
    Ok(())
}
