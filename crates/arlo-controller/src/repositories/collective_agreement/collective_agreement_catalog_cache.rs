use crate::error::ControllerResult;
use crate::repositories::collective_agreement::collective_agreement_repository;
use arlo_domain::CollectiveAgreement;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use uuid::Uuid;

static COLLECTIVE_AGREEMENT_CATALOG: LazyLock<
    RwLock<Option<Arc<HashMap<Uuid, CollectiveAgreement>>>>,
> = LazyLock::new(|| RwLock::new(None));

pub async fn get_or_load_collective_agreement_catalog(
    pool: &SqlitePool,
) -> ControllerResult<Arc<HashMap<Uuid, CollectiveAgreement>>> {
    {
        let read_guard = COLLECTIVE_AGREEMENT_CATALOG.read().await;
        if let Some(catalog) = read_guard.as_ref() {
            return Ok(catalog.clone());
        }
    }

    let agreements = collective_agreement_repository::list_all(pool).await?;
    let mut map = HashMap::with_capacity(agreements.len());
    for agreement in agreements {
        map.insert(agreement.id(), agreement);
    }
    let catalog = Arc::new(map);

    let mut write_guard = COLLECTIVE_AGREEMENT_CATALOG.write().await;
    if let Some(existing) = write_guard.as_ref() {
        return Ok(existing.clone());
    }

    *write_guard = Some(catalog.clone());
    Ok(catalog)
}

pub async fn refresh(
    pool: &SqlitePool,
) -> ControllerResult<Arc<HashMap<Uuid, CollectiveAgreement>>> {
    let agreements = collective_agreement_repository::list_all(pool).await?;
    let mut map = HashMap::with_capacity(agreements.len());
    for agreement in agreements {
        map.insert(agreement.id(), agreement);
    }
    let catalog = Arc::new(map);

    let mut write_guard = COLLECTIVE_AGREEMENT_CATALOG.write().await;
    *write_guard = Some(catalog.clone());
    Ok(catalog)
}

pub async fn invalidate() {
    let mut write_guard = COLLECTIVE_AGREEMENT_CATALOG.write().await;
    *write_guard = None;
}