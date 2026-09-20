use crate::error::{ControllerError, ControllerResult};
use arlo_domain::AttributeDefinition;
use arlo_engine::attributes::AttributeKeyIndex;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;
use uuid::Uuid;

static ATTRIBUTE_KEY_INDEX: LazyLock<RwLock<Option<Arc<AttributeKeyIndex>>>> =
    LazyLock::new(|| RwLock::new(None));

static ATTRIBUTE_DEFINITIONS: LazyLock<RwLock<Option<Arc<HashMap<Uuid, AttributeDefinition>>>>> =
    LazyLock::new(|| RwLock::new(None));

pub async fn get_or_load_attribute_key_index(
    pool: &SqlitePool,
) -> ControllerResult<Arc<AttributeKeyIndex>> {
    {
        let read_guard = ATTRIBUTE_KEY_INDEX.read().await;
        if let Some(index) = read_guard.as_ref() {
            return Ok(index.clone());
        }
    }

    let attr_defs = arlo_db::repositories::attribute_definition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut keys_by_id = HashMap::with_capacity(attr_defs.len());
    for def in attr_defs {
        keys_by_id.insert(def.id(), def.key());
    }

    let index = Arc::new(AttributeKeyIndex::from_map(&keys_by_id));

    let mut write_guard = ATTRIBUTE_KEY_INDEX.write().await;
    if let Some(existing) = write_guard.as_ref() {
        return Ok(existing.clone());
    }

    *write_guard = Some(index.clone());
    Ok(index)
}

pub async fn get_or_load_attribute_definitions(
    pool: &SqlitePool,
) -> ControllerResult<Arc<HashMap<Uuid, AttributeDefinition>>> {
    {
        let read_guard = ATTRIBUTE_DEFINITIONS.read().await;
        if let Some(defs) = read_guard.as_ref() {
            return Ok(defs.clone());
        }
    }

    let attr_defs = arlo_db::repositories::attribute_definition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut defs_by_id = HashMap::with_capacity(attr_defs.len());
    for def in attr_defs {
        defs_by_id.insert(def.id(), def);
    }

    let map = Arc::new(defs_by_id);

    let mut write_guard = ATTRIBUTE_DEFINITIONS.write().await;
    if let Some(existing) = write_guard.as_ref() {
        return Ok(existing.clone());
    }

    *write_guard = Some(map.clone());
    Ok(map)
}

pub async fn refresh(pool: &SqlitePool) -> ControllerResult<Arc<AttributeKeyIndex>> {
    let attr_defs = arlo_db::repositories::attribute_definition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut keys_by_id = HashMap::with_capacity(attr_defs.len());
    let mut defs_by_id = HashMap::with_capacity(attr_defs.len());
    for def in attr_defs {
        keys_by_id.insert(def.id(), def.key());
        defs_by_id.insert(def.id(), def);
    }

    let index = Arc::new(AttributeKeyIndex::from_map(&keys_by_id));
    let defs_map = Arc::new(defs_by_id);

    let mut write_guard = ATTRIBUTE_KEY_INDEX.write().await;
    *write_guard = Some(index.clone());

    let mut defs_guard = ATTRIBUTE_DEFINITIONS.write().await;
    *defs_guard = Some(defs_map);

    Ok(index)
}

pub async fn invalidate() {
    let mut write_guard = ATTRIBUTE_KEY_INDEX.write().await;
    *write_guard = None;

    let mut defs_guard = ATTRIBUTE_DEFINITIONS.write().await;
    *defs_guard = None;
}
