use crate::domain::season::KnockoutTie;
use crate::error::ControllerResult;
use crate::services::season::persistence::knockout_tie_row_mapper::map_row_to_knockout_tie;
use arlo_persistence::models::season::KnockoutTieRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_stage_knockout_ties(
    pool: &SqlitePool,
    stage_id: Uuid,
) -> ControllerResult<Vec<KnockoutTie>> {
    let rows = sqlx::query_as::<_, KnockoutTieRow>(
        "SELECT id, season_stage_id, round_index, tie_index, high_seed_team_id, high_seed_number, low_seed_team_id, low_seed_number, leg_one_fixture_id, leg_two_fixture_id, aggregate_winner_team_id FROM knockout_ties WHERE season_stage_id = ?",
    )
    .bind(stage_id.to_string())
    .fetch_all(pool)
    .await?;

    let mut ties = Vec::with_capacity(rows.len());
    for row in &rows {
        ties.push(map_row_to_knockout_tie(row)?);
    }
    Ok(ties)
}

pub async fn activate_stage(pool: &SqlitePool, stage_id: Uuid) -> ControllerResult<()> {
    sqlx::query("UPDATE season_stages SET status = 'Active' WHERE id = ?")
        .bind(stage_id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn activate_season_instance(
    pool: &SqlitePool,
    season_instance_id: Uuid,
) -> ControllerResult<()> {
    sqlx::query("UPDATE season_instances SET status = 'Active' WHERE id = ?")
        .bind(season_instance_id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn mark_stage_completed(pool: &SqlitePool, stage_id: Uuid) -> ControllerResult<()> {
    sqlx::query("UPDATE season_stages SET status = 'Completed' WHERE id = ?")
        .bind(stage_id.to_string())
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn advance_season_stage(
    pool: &SqlitePool,
    season_instance_id: Uuid,
    next_stage_order_index: u32,
) -> ControllerResult<()> {
    sqlx::query(
        "UPDATE season_instances SET current_stage_order_index = ?, status = 'Active' WHERE id = ?",
    )
    .bind(next_stage_order_index as i32)
    .bind(season_instance_id.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn finalize_season_instance(
    pool: &SqlitePool,
    season_instance_id: Uuid,
) -> ControllerResult<()> {
    let mut tx = pool.begin().await?;
    sqlx::query("UPDATE season_instances SET status = 'Completed' WHERE id = ?")
        .bind(season_instance_id.to_string())
        .execute(&mut *tx)
        .await?;

    sqlx::query(
        "UPDATE season_stages SET status = 'Completed' WHERE season_instance_id = ? AND status != 'Completed'",
    )
    .bind(season_instance_id.to_string())
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO award_season_jobs (award_definition_id, season_instance_id) SELECT d.id, si.id FROM award_definitions d JOIN season_instances si ON si.id = ? WHERE d.active = 1 AND d.trigger_policy = '\"SeasonCompleted\"' AND d.evaluation_window = '\"EntireSeason\"' AND d.scope_kind = 'Competition' AND (NOT EXISTS (SELECT 1 FROM award_eligibility_competitions ec WHERE ec.award_definition_id = d.id) OR EXISTS (SELECT 1 FROM award_eligibility_competitions ec WHERE ec.award_definition_id = d.id AND ec.competition_id = si.competition_id)) ON CONFLICT DO NOTHING"
    )
    .bind(season_instance_id.to_string())
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO award_global_cycle_jobs (award_definition_id, reference_year) SELECT d.id, si.reference_year FROM award_definitions d JOIN season_instances si ON si.id = ? WHERE d.active = 1 AND d.trigger_policy = '\"SeasonCompleted\"' AND d.evaluation_window = '\"PreviousSeasonCycle\"' AND d.scope_kind = 'Global' AND (NOT EXISTS (SELECT 1 FROM award_eligibility_competitions ec WHERE ec.award_definition_id = d.id) OR EXISTS (SELECT 1 FROM award_eligibility_competitions ec WHERE ec.award_definition_id = d.id AND ec.competition_id = si.competition_id)) ON CONFLICT DO NOTHING",
    )
    .bind(season_instance_id.to_string())
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}
