use crate::error::ControllerResult;
use crate::repositories::attribute::attribute_definition_cache;
use crate::repositories::calendar::calendar_catalog_cache;
use crate::repositories::collective_agreement::collective_agreement_catalog_cache;
use crate::repositories::formation::formation_cache;
use crate::repositories::league_calendar::league_calendar_config_cache;
use crate::repositories::referee::referee_cache;
use crate::repositories::season::standings_cache;
use crate::repositories::venue::eligible_venue_cache;
use crate::services::season::matchday::matchday_catalog_cache;
use sqlx::SqlitePool;

pub async fn reload_all_caches(pool: &SqlitePool) -> ControllerResult<()> {
    attribute_definition_cache::refresh(pool).await?;
    calendar_catalog_cache::refresh(pool).await?;
    collective_agreement_catalog_cache::refresh(pool).await?;
    formation_cache::refresh(pool).await?;
    referee_cache::refresh(pool).await?;
    league_calendar_config_cache::refresh_all(pool).await?;
    eligible_venue_cache::refresh_all(pool).await?;
    matchday_catalog_cache::refresh(pool).await?;
    standings_cache::invalidate_all().await;
    Ok(())
}
