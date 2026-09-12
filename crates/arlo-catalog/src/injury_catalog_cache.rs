use arlo_db::repositories::injury_definition;
use arlo_db::{DbResult, SqlitePool};
use arlo_domain::InjuryCatalog;
use std::sync::Arc;
use tokio::sync::OnceCell;

static INJURY_CATALOG: OnceCell<Arc<InjuryCatalog>> = OnceCell::const_new();

pub async fn get_or_load_injury_catalog(pool: &SqlitePool) -> DbResult<Arc<InjuryCatalog>> {
    let catalog = INJURY_CATALOG
        .get_or_try_init(|| async {
            let definitions = injury_definition::list_all(pool).await?;
            let catalog = InjuryCatalog::new(definitions);
            Ok::<Arc<InjuryCatalog>, arlo_db::DbError>(Arc::new(catalog))
        })
        .await?;

    Ok(Arc::clone(catalog))
}