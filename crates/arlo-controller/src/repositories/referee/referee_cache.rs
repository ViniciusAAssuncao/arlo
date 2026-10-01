use crate::error::{ControllerError, ControllerResult};
use crate::repositories::attribute::attribute_definition_cache::get_or_load_referee_attribute_definitions;
use arlo_domain::Referee;
use sqlx::SqlitePool;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;

static REFEREES_CACHE: LazyLock<RwLock<Option<Arc<Vec<Referee>>>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn get_or_load_referees(pool: &SqlitePool) -> ControllerResult<Arc<Vec<Referee>>> {
    {
        let read_guard = REFEREES_CACHE.read().await;
        if let Some(referees) = read_guard.as_ref() {
            return Ok(referees.clone());
        }
    }

    let def_map = get_or_load_referee_attribute_definitions(pool).await?;
    let referees = arlo_db::repositories::referee::list_all(pool, &def_map)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let arc_referees = Arc::new(referees);

    let mut write_guard = REFEREES_CACHE.write().await;
    if let Some(existing) = write_guard.as_ref() {
        return Ok(existing.clone());
    }

    *write_guard = Some(arc_referees.clone());
    Ok(arc_referees)
}

pub async fn refresh(pool: &SqlitePool) -> ControllerResult<Arc<Vec<Referee>>> {
    let def_map = get_or_load_referee_attribute_definitions(pool).await?;
    let referees = arlo_db::repositories::referee::list_all(pool, &def_map)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let arc_referees = Arc::new(referees);

    let mut write_guard = REFEREES_CACHE.write().await;
    *write_guard = Some(arc_referees.clone());
    Ok(arc_referees)
}

pub async fn invalidate() {
    let mut write_guard = REFEREES_CACHE.write().await;
    *write_guard = None;
}
