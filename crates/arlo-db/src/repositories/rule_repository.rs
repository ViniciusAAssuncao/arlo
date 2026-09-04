use crate::error::DbResult;
use crate::models::RuleRow;
use crate::repositories::fetch::{
    fetch_all, fetch_all_by_param, fetch_all_by_two_params, fetch_optional_by_param,
};
use arlo_domain::{Rule, RuleCategory};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> DbResult<Option<Rule>> {
    let row = fetch_optional_by_param::<RuleRow>(
        pool,
        "SELECT id, competition_id, category, rule_key, value FROM rules WHERE id = ?",
        &id.to_string(),
    )
    .await?;

    match row {
        Some(r) => Ok(Some(r.to_domain()?)),
        None => Ok(None),
    }
}

pub async fn list_all(pool: &SqlitePool) -> DbResult<Vec<Rule>> {
    let rows = fetch_all::<RuleRow>(
        pool,
        "SELECT id, competition_id, category, rule_key, value FROM rules",
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_competition_id(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> DbResult<Vec<Rule>> {
    let rows = fetch_all_by_param::<RuleRow>(
        pool,
        "SELECT id, competition_id, category, rule_key, value FROM rules WHERE competition_id = ?",
        &competition_id.to_string(),
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}

pub async fn list_by_competition_id_and_category(
    pool: &SqlitePool,
    competition_id: Uuid,
    category: RuleCategory,
) -> DbResult<Vec<Rule>> {
    let category_str = match category {
        RuleCategory::Calendar => "Calendar",
        RuleCategory::Teams => "Teams",
        RuleCategory::Scoring => "Scoring",
        RuleCategory::Phases => "Phases",
        RuleCategory::TieBreaker => "TieBreaker",
        RuleCategory::PromotionRelegation => "PromotionRelegation",
    };
    let rows = fetch_all_by_two_params::<RuleRow>(
        pool,
        "SELECT id, competition_id, category, rule_key, value FROM rules WHERE competition_id = ? AND category = ?",
        &competition_id.to_string(),
        category_str,
    )
    .await?;
    let mut results = Vec::with_capacity(rows.len());
    for row in rows {
        results.push(row.to_domain()?);
    }
    Ok(results)
}