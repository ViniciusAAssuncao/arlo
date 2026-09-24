use crate::error::{ControllerError, ControllerResult};
use arlo_domain::{
    AttributeDefinition, AttributeKey, FaultCatalog, FaultPunishmentOption, InjuryCatalog,
    InjuryDefinition,
};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MatchdayCatalogs {
    pub fault_catalog: Arc<FaultCatalog>,
    pub injury_catalog: Arc<InjuryCatalog>,
    pub attribute_keys_by_id: Arc<HashMap<Uuid, AttributeKey>>,
    pub attribute_definitions_by_id: Arc<HashMap<Uuid, AttributeDefinition>>,
}

static MATCHDAY_CATALOGS: LazyLock<RwLock<Option<Arc<MatchdayCatalogs>>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn get_or_load_matchday_catalogs(
    pool: &SqlitePool,
) -> ControllerResult<Arc<MatchdayCatalogs>> {
    {
        let read_guard = MATCHDAY_CATALOGS.read().await;
        if let Some(catalogs) = read_guard.as_ref() {
            return Ok(catalogs.clone());
        }
    }

    let fault_defs = arlo_db::repositories::fault_definition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut all_options: Vec<FaultPunishmentOption> = Vec::new();
    for def in &fault_defs {
        let options = arlo_db::repositories::fault_punishment_option::list_by_fault_definition_id(
            pool,
            def.id(),
        )
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
        all_options.extend(options);
    }

    let fault_catalog = Arc::new(FaultCatalog::new(fault_defs, all_options));

    let injury_defs: Vec<InjuryDefinition> =
        arlo_db::repositories::injury_definition::list_all(pool)
            .await
            .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let injury_catalog = Arc::new(InjuryCatalog::new(injury_defs));

    let attr_defs = arlo_db::repositories::attribute_definition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut keys_by_id = HashMap::with_capacity(attr_defs.len());
    let mut defs_by_id = HashMap::with_capacity(attr_defs.len());
    for def in attr_defs {
        keys_by_id.insert(def.id(), def.key());
        defs_by_id.insert(def.id(), def);
    }

    let catalogs = Arc::new(MatchdayCatalogs {
        fault_catalog,
        injury_catalog,
        attribute_keys_by_id: Arc::new(keys_by_id),
        attribute_definitions_by_id: Arc::new(defs_by_id),
    });

    let mut write_guard = MATCHDAY_CATALOGS.write().await;
    if let Some(existing) = write_guard.as_ref() {
        return Ok(existing.clone());
    }

    *write_guard = Some(catalogs.clone());
    Ok(catalogs)
}

pub async fn refresh(pool: &SqlitePool) -> ControllerResult<Arc<MatchdayCatalogs>> {
    let mut write_guard = MATCHDAY_CATALOGS.write().await;
    *write_guard = None;
    drop(write_guard);
    get_or_load_matchday_catalogs(pool).await
}

pub async fn invalidate() {
    let mut write_guard = MATCHDAY_CATALOGS.write().await;
    *write_guard = None;
}
