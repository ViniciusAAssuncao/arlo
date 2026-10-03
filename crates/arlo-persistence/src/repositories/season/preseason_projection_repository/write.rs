use crate::error::PersistenceResult;
use arlo_analytics::prediction::preseason::PreseasonProjection;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn publish(
    pool: &SqlitePool,
    projection: &PreseasonProjection,
    calibration_id: Uuid,
    generated_year: i64,
    generated_day: u32,
    power_seed_model_version: u32,
) -> PersistenceResult<Uuid> {
    let id = Uuid::new_v4();
    let mut tx = pool.begin().await?;
    let inserted = sqlx::query(
        "INSERT OR IGNORE INTO preseason_projections (id, season_instance_id, model_version, calibration_id, generated_year, generated_day_of_year, simulation_count, random_seed, power_seed_model_version, historical_rating_sigma, new_team_rating_sigma, knockout_margin_mean, goal_point_mean, goal_point_rating_slope) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(id.to_string())
    .bind(projection.season_instance_id.to_string())
    .bind(projection.model_version as i64)
    .bind(calibration_id.to_string())
    .bind(generated_year)
    .bind(generated_day as i64)
    .bind(projection.simulation_count as i64)
    .bind(projection.random_seed.to_string())
    .bind(power_seed_model_version as i64)
    .bind(projection.config.historical_rating_sigma)
    .bind(projection.config.new_team_rating_sigma)
    .bind(projection.config.knockout_margin_mean)
    .bind(projection.goal_point_model.map(|model| model.mean_per_team))
    .bind(projection.goal_point_model.map(|model| model.rating_slope))
    .execute(&mut *tx)
    .await?;
    let stored_id: String =
        sqlx::query_scalar("SELECT id FROM preseason_projections WHERE season_instance_id = ?")
            .bind(projection.season_instance_id.to_string())
            .fetch_one(&mut *tx)
            .await?;
    if inserted.rows_affected() == 0 {
        tx.commit().await?;
        return Ok(Uuid::parse_str(&stored_id)?);
    }
    for team in &projection.teams {
        sqlx::query(
            "INSERT INTO preseason_team_projections (projection_id, team_id, projected_table_position, top_four_probability, champion_probability, runner_up_probability) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&stored_id).bind(team.team_id.to_string())
        .bind(team.projected_table_position.map(i64::from))
        .bind(team.top_four_probability)
        .bind(team.champion_probability).bind(team.runner_up_probability)
        .execute(&mut *tx).await?;
    }
    for stage in &projection.stages {
        let probabilities = serde_json::to_string(&stage.position_probabilities)
            .map_err(|error| crate::error::PersistenceError::InvalidData(error.to_string()))?;
        sqlx::query(
            "INSERT INTO preseason_stage_team_projections (projection_id, stage_index, team_id, reach_probability, mean_position, median_position, modal_position, position_probabilities_json) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&stored_id).bind(stage.stage_index as i64).bind(stage.team_id.to_string())
        .bind(stage.reach_probability).bind(stage.mean_position)
        .bind(stage.median_position.map(i64::from)).bind(stage.modal_position.map(i64::from))
        .bind(probabilities).execute(&mut *tx).await?;
    }
    for round in &projection.rounds {
        sqlx::query(
            "INSERT INTO preseason_round_team_projections (projection_id, stage_index, round_index, team_id, reach_probability) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&stored_id).bind(round.stage_index as i64).bind(round.round_index as i64)
        .bind(round.team_id.to_string()).bind(round.reach_probability)
        .execute(&mut *tx).await?;
    }
    for encounter in &projection.encounters {
        sqlx::query(
            "INSERT INTO preseason_encounter_projections (projection_id, stage_index, round_index, first_team_id, second_team_id, probability) VALUES (?, ?, ?, ?, ?, ?)",
        )
        .bind(&stored_id).bind(encounter.stage_index as i64).bind(encounter.round_index as i64)
        .bind(encounter.first_team_id.to_string()).bind(encounter.second_team_id.to_string())
        .bind(encounter.probability).execute(&mut *tx).await?;
    }
    tx.commit().await?;
    Ok(Uuid::parse_str(&stored_id)?)
}
