use arlo_domain::LeagueCalendarConfig;
use crate::error::ControllerResult;
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use sqlx::SqlitePool;
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_league_calendar_config(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> ControllerResult<Option<Arc<LeagueCalendarConfig>>> {
    get_or_load_league_calendar_config(pool, competition_id).await
}

pub async fn list_league_calendar_configs(
    pool: &SqlitePool,
) -> ControllerResult<Vec<Arc<LeagueCalendarConfig>>> {
    let leagues = arlo_db::repositories::league::list_all(pool)
        .await
        .map_err(|e| crate::error::ControllerError::InvalidData(e.to_string()))?;

    let mut configs = Vec::with_capacity(leagues.len());
    for league in leagues {
        if let Some(config) = get_or_load_league_calendar_config(pool, league.id()).await? {
            configs.push(config);
        }
    }
    Ok(configs)
}
