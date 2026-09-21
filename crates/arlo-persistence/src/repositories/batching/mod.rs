use crate::error::PersistenceResult;
use sqlx::{QueryBuilder, Sqlite, Transaction};

const MAX_SQLITE_VARIABLES: usize = 999;

pub async fn execute_batch_insert<'a, T, F>(
    tx: &mut Transaction<'_, Sqlite>,
    table: &str,
    columns: &[&str],
    items: &'a [T],
    mut bind_fn: F,
) -> PersistenceResult<()>
where
    T: 'a,
    F: FnMut(&mut sqlx::query_builder::Separated<'_, 'a, Sqlite, &'static str>, &'a T),
{
    if items.is_empty() {
        return Ok(());
    }

    let col_count = columns.len().max(1);
    let chunk_size = (MAX_SQLITE_VARIABLES / col_count).max(1);
    let prefix = format!("INSERT INTO {} ({}) ", table, columns.join(", "));

    for chunk in items.chunks(chunk_size) {
        let mut builder: QueryBuilder<'a, Sqlite> = QueryBuilder::new(&prefix);
        builder.push_values(chunk, |mut b, item| {
            bind_fn(&mut b, item);
        });
        builder.build().execute(&mut **tx).await?;
    }

    Ok(())
}
