use arlo_db::repositories::{fault_definition, fault_punishment_option};
use arlo_db::{DbResult, SqlitePool};
use arlo_domain::FaultCatalog;
use std::sync::Arc;
use tokio::sync::OnceCell;

static FAULT_CATALOG: OnceCell<Arc<FaultCatalog>> = OnceCell::const_new();

pub async fn get_or_load_fault_catalog(pool: &SqlitePool) -> DbResult<Arc<FaultCatalog>> {
    let catalog = FAULT_CATALOG
        .get_or_try_init(|| async {
            let definitions = fault_definition::list_all(pool).await?;
            let mut all_options = Vec::new();
            for def in &definitions {
                let options: Vec<arlo_domain::FaultPunishmentOption> =
                    fault_punishment_option::list_by_fault_definition_id(pool, def.id()).await?;
                all_options.extend(options);
            }
            let catalog = FaultCatalog::new(definitions, all_options);
            Ok::<Arc<FaultCatalog>, arlo_db::DbError>(Arc::new(catalog))
        })
        .await?;

    Ok(Arc::clone(catalog))
}