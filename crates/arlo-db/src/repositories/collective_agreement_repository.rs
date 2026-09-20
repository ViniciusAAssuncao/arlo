use crate::error::DbResult;
use crate::models::league_calendar::CollectiveAgreementRow;
use crate::repositories::fetch::{fetch_all, fetch_optional_by_param};
use arlo_domain::CollectiveAgreement;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<CollectiveAgreement>> {
    let row = fetch_optional_by_param::<CollectiveAgreementRow>(
        pool,
        "SELECT id, name, rule_kind, start_month_order_index, start_day_of_month, end_month_order_index, end_day_of_month, end_year_offset, created_at_unix_seconds FROM collective_agreements WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<CollectiveAgreement>> {
    let rows = fetch_all::<CollectiveAgreementRow>(
        pool,
        "SELECT id, name, rule_kind, start_month_order_index, start_day_of_month, end_month_order_index, end_day_of_month, end_year_offset, created_at_unix_seconds FROM collective_agreements",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_ids(
    pool: &SqlitePool,
    ids: &[Uuid],
) -> DbResult<Vec<CollectiveAgreement>> {
    let mut results = Vec::with_capacity(ids.len());
    for &id in ids {
        if let Some(agreement) = get_by_id(pool, id).await? {
            results.push(agreement);
        }
    }
    Ok(results)
}