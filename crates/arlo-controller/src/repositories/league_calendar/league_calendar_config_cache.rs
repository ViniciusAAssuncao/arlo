use crate::error::{ControllerError, ControllerResult};
use arlo_domain::LeagueCalendarConfig;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use uuid::Uuid;

static LEAGUE_CONFIG_CACHE: LazyLock<RwLock<HashMap<Uuid, Arc<LeagueCalendarConfig>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub async fn get_or_load_league_calendar_config(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> ControllerResult<Option<Arc<LeagueCalendarConfig>>> {
    {
        let read_guard = LEAGUE_CONFIG_CACHE.read().await;
        if let Some(config) = read_guard.get(&competition_id) {
            return Ok(Some(config.clone()));
        }
    }

    let config_opt =
        arlo_db::repositories::league_calendar_config::get_by_competition_id(pool, competition_id)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut write_guard = LEAGUE_CONFIG_CACHE.write().await;
    if let Some(config) = write_guard.get(&competition_id) {
        return Ok(Some(config.clone()));
    }

    if let Some(config) = config_opt {
        let arc_config = Arc::new(config);
        write_guard.insert(competition_id, arc_config.clone());
        Ok(Some(arc_config))
    } else {
        Ok(None)
    }
}

pub async fn refresh_all(pool: &SqlitePool) -> ControllerResult<()> {
    let leagues = arlo_db::repositories::league::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut fresh_map = HashMap::with_capacity(leagues.len());
    for league in leagues {
        let comp_id = league.id();
        if let Some(config) =
            arlo_db::repositories::league_calendar_config::get_by_competition_id(pool, comp_id)
                .await
                .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        {
            fresh_map.insert(comp_id, Arc::new(config));
        }
    }

    let mut write_guard = LEAGUE_CONFIG_CACHE.write().await;
    *write_guard = fresh_map;
    Ok(())
}
