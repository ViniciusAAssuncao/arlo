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
    sqlx::query("UPDATE season_instances SET status = 'Completed' WHERE id = ?")
        .bind(season_instance_id.to_string())
        .execute(pool)
        .await?;

    sqlx::query(
        "UPDATE season_stages SET status = 'Completed' WHERE season_instance_id = ? AND status != 'Completed'",
    )
    .bind(season_instance_id.to_string())
    .execute(pool)
    .await?;

    Ok(())
}