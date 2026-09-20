use crate::error::{ControllerError, ControllerResult};
use arlo_engine::attributes::AttributeKeyIndex;
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::RwLock;

static ATTRIBUTE_KEY_INDEX: LazyLock<RwLock<Option<Arc<AttributeKeyIndex>>>> =
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

pub async fn refresh(pool: &SqlitePool) -> ControllerResult<Arc<AttributeKeyIndex>> {
    let attr_defs = arlo_db::repositories::attribute_definition::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let mut keys_by_id = HashMap::with_capacity(attr_defs.len());
    for def in attr_defs {
        keys_by_id.insert(def.id(), def.key());
    }

    let index = Arc::new(AttributeKeyIndex::from_map(&keys_by_id));

    let mut write_guard = ATTRIBUTE_KEY_INDEX.write().await;
    *write_guard = Some(index.clone());
    Ok(index)
}

pub async fn invalidate() {
    let mut write_guard = ATTRIBUTE_KEY_INDEX.write().await;
    *write_guard = None;
}