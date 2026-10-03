use crate::error::PersistenceResult;
use sqlx::{Sqlite, Transaction};

pub async fn execute_batch_insert<'a, T: 'a>(
    tx: &mut Transaction<'_, Sqlite>,
    table: &str,
    columns: &[&str],
    items: &'a [T],
    bind: impl FnMut(&mut sqlx::query_builder::Separated<'_, 'a, Sqlite, &'static str>, &'a T),
) -> PersistenceResult<()> {
    arlo_db::repositories::batching::execute_batch_insert(tx, table, columns, items, bind).await?;
    Ok(())
}
