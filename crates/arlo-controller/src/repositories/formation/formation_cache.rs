use crate::error::{ControllerError, ControllerResult};
use arlo_domain::Formation;
use sqlx::SqlitePool;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;

static FORMATIONS_CACHE: LazyLock<RwLock<Option<Arc<Vec<Formation>>>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn get_or_load_formations(
    pool: &SqlitePool,
) -> ControllerResult<Arc<Vec<Formation>>> {
    {
        let read_guard = FORMATIONS_CACHE.read().await;
        if let Some(formations) = read_guard.as_ref() {
            return Ok(formations.clone());
        }
    }

    let formations = arlo_db::repositories::formation::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let arc_formations = Arc::new(formations);

    let mut write_guard = FORMATIONS_CACHE.write().await;
    if let Some(existing) = write_guard.as_ref() {
        return Ok(existing.clone());
    }

    *write_guard = Some(arc_formations.clone());
    Ok(arc_formations)
}

pub async fn refresh(pool: &SqlitePool) -> ControllerResult<Arc<Vec<Formation>>> {
    let formations = arlo_db::repositories::formation::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let arc_formations = Arc::new(formations);

    let mut write_guard = FORMATIONS_CACHE.write().await;
    *write_guard = Some(arc_formations.clone());
    Ok(arc_formations)
}

pub async fn invalidate() {
    let mut write_guard = FORMATIONS_CACHE.write().await;
    *write_guard = None;
}