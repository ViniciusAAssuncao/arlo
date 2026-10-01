use crate::error::{ControllerError, ControllerResult};
use arlo_domain::Venue;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use uuid::Uuid;

static COUNTRY_VENUES_CACHE: LazyLock<RwLock<HashMap<Uuid, Arc<Vec<Venue>>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
static ALL_VENUES_CACHE: LazyLock<RwLock<Option<Arc<Vec<Venue>>>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn get_or_load_venues_for_country(
    pool: &SqlitePool,
    country_id: Uuid,
) -> ControllerResult<Arc<Vec<Venue>>> {
    {
        let read_guard = COUNTRY_VENUES_CACHE.read().await;
        if let Some(venues) = read_guard.get(&country_id) {
            return Ok(venues.clone());
        }
    }

    let venues = arlo_db::repositories::venue::list_by_country_id(pool, country_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let arc_venues = Arc::new(venues);
    let mut write_guard = COUNTRY_VENUES_CACHE.write().await;
    write_guard.insert(country_id, arc_venues.clone());
    Ok(arc_venues)
}

pub async fn get_or_load_all_venues(pool: &SqlitePool) -> ControllerResult<Arc<Vec<Venue>>> {
    {
        let read_guard = ALL_VENUES_CACHE.read().await;
        if let Some(venues) = read_guard.as_ref() {
            return Ok(venues.clone());
        }
    }

    let venues = arlo_db::repositories::venue::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let arc_venues = Arc::new(venues);
    let mut write_guard = ALL_VENUES_CACHE.write().await;
    *write_guard = Some(arc_venues.clone());
    Ok(arc_venues)
}

pub async fn refresh_all(pool: &SqlitePool) -> ControllerResult<()> {
    let all_venues = arlo_db::repositories::venue::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut fresh_map: HashMap<Uuid, Vec<Venue>> = HashMap::new();
    for venue in &all_venues {
        fresh_map
            .entry(venue.country_id())
            .or_default()
            .push(venue.clone());
    }

    let mut country_guard = COUNTRY_VENUES_CACHE.write().await;
    *country_guard = fresh_map
        .into_iter()
        .map(|(k, v)| (k, Arc::new(v)))
        .collect();

    let mut all_guard = ALL_VENUES_CACHE.write().await;
    *all_guard = Some(Arc::new(all_venues));

    Ok(())
}
