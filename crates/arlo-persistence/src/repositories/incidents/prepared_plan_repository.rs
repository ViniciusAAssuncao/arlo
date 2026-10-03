use crate::error::PersistenceResult;
use crate::models::{MatchPreparedPlanRow, MatchTacticalPlanActivationRow};
use sqlx::{Sqlite, SqlitePool, Transaction};
use uuid::Uuid;

pub async fn insert_plan(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchPreparedPlanRow,
) -> PersistenceResult<()> {
    sqlx::query("INSERT INTO match_prepared_plans (match_id, team_id, plan_id, plan_json, profile_json) VALUES (?, ?, ?, ?, ?)")
        .bind(&row.match_id).bind(&row.team_id).bind(&row.plan_id)
        .bind(sqlx::types::Json(&row.plan)).bind(sqlx::types::Json(&row.profile))
        .execute(&mut **tx).await?;
    Ok(())
}

pub async fn insert_activation(
    tx: &mut Transaction<'_, Sqlite>,
    row: &MatchTacticalPlanActivationRow,
) -> PersistenceResult<()> {
    sqlx::query("INSERT INTO match_tactical_plan_activations (match_id, sequence_number, team_id, plan_id, plan_name, formation_id, profile_id, period, seconds_in_period, total_elapsed_seconds, assignments_json) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(&row.match_id).bind(row.sequence_number).bind(&row.team_id).bind(&row.plan_id)
        .bind(&row.plan_name).bind(&row.formation_id).bind(&row.profile_id).bind(row.period)
        .bind(row.seconds_in_period).bind(row.total_elapsed_seconds)
        .bind(sqlx::types::Json(&row.assignments)).execute(&mut **tx).await?;
    Ok(())
}

pub async fn list_plans_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchPreparedPlanRow>> {
    Ok(sqlx::query_as::<_, MatchPreparedPlanRow>("SELECT match_id, team_id, plan_id, plan_json AS plan, profile_json AS profile FROM match_prepared_plans WHERE match_id = ? ORDER BY team_id, plan_id")
        .bind(match_id.to_string()).fetch_all(pool).await?)
}

pub async fn list_activations_by_match_id(
    pool: &SqlitePool,
    match_id: Uuid,
) -> PersistenceResult<Vec<MatchTacticalPlanActivationRow>> {
    Ok(sqlx::query_as::<_, MatchTacticalPlanActivationRow>("SELECT match_id, sequence_number, team_id, plan_id, plan_name, formation_id, profile_id, period, seconds_in_period, total_elapsed_seconds, assignments_json AS assignments FROM match_tactical_plan_activations WHERE match_id = ? ORDER BY sequence_number")
        .bind(match_id.to_string()).fetch_all(pool).await?)
}
