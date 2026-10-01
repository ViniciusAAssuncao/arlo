use crate::error::DbResult;
use sqlx::{FromRow, SqlitePool};

pub async fn fetch_optional_by_param<T>(
    pool: &SqlitePool,
    query: &str,
    param: &str,
) -> DbResult<Option<T>>
where
    T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let result = sqlx::query_as::<_, T>(query)
        .bind(param)
        .fetch_optional(pool)
        .await?;
    Ok(result)
}

pub async fn fetch_all<T>(pool: &SqlitePool, query: &str) -> DbResult<Vec<T>>
where
    T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let result = sqlx::query_as::<_, T>(query).fetch_all(pool).await?;
    Ok(result)
}

pub async fn fetch_all_by_param<T>(pool: &SqlitePool, query: &str, param: &str) -> DbResult<Vec<T>>
where
    T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let result = sqlx::query_as::<_, T>(query)
        .bind(param)
        .fetch_all(pool)
        .await?;
    Ok(result)
}

pub async fn fetch_all_by_two_params<T>(
    pool: &SqlitePool,
    query: &str,
    param1: &str,
    param2: &str,
) -> DbResult<Vec<T>>
where
    T: for<'r> FromRow<'r, sqlx::sqlite::SqliteRow> + Send + Unpin,
{
    let result = sqlx::query_as::<_, T>(query)
        .bind(param1)
        .bind(param2)
        .fetch_all(pool)
        .await?;
    Ok(result)
}
