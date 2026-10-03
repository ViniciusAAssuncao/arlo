use crate::error::{PersistenceError, PersistenceResult};
use arlo_analytics::prediction::preseason::{
    EncounterProjection, GoalPointModel, PreseasonConfig, PreseasonProjection, RoundTeamProjection,
    StageTeamProjection, TeamProjection,
};
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct StoredPreseasonProjection {
    pub id: Uuid,
    pub calibration_id: Uuid,
    pub generated_year: i64,
    pub generated_day_of_year: u32,
    pub power_seed_model_version: u32,
    pub projection: PreseasonProjection,
}

#[derive(FromRow)]
struct HeaderRow {
    id: String,
    season_instance_id: String,
    model_version: i64,
    calibration_id: String,
    generated_year: i64,
    generated_day_of_year: i64,
    simulation_count: i64,
    random_seed: String,
    power_seed_model_version: i64,
    historical_rating_sigma: f64,
    new_team_rating_sigma: f64,
    knockout_margin_mean: f64,
    goal_point_mean: Option<f64>,
    goal_point_rating_slope: Option<f64>,
}

#[derive(FromRow)]
struct TeamRow {
    team_id: String,
    projected_table_position: Option<i64>,
    top_four_probability: Option<f64>,
    champion_probability: f64,
    runner_up_probability: f64,
}

#[derive(FromRow)]
struct StageRow {
    stage_index: i64,
    team_id: String,
    reach_probability: f64,
    mean_position: Option<f64>,
    median_position: Option<i64>,
    modal_position: Option<i64>,
    position_probabilities_json: String,
}

#[derive(FromRow)]
struct RoundRow {
    stage_index: i64,
    round_index: i64,
    team_id: String,
    reach_probability: f64,
}

#[derive(FromRow)]
struct EncounterRow {
    stage_index: i64,
    round_index: i64,
    first_team_id: String,
    second_team_id: String,
    probability: f64,
}

pub async fn get(
    pool: &SqlitePool,
    season_id: Uuid,
) -> PersistenceResult<Option<StoredPreseasonProjection>> {
    let Some(header) = sqlx::query_as::<_, HeaderRow>(
        "SELECT * FROM preseason_projections WHERE season_instance_id = ?",
    )
    .bind(season_id.to_string())
    .fetch_optional(pool)
    .await?
    else {
        return Ok(None);
    };
    let teams = sqlx::query_as::<_, TeamRow>(
        "SELECT team_id, projected_table_position, top_four_probability, champion_probability, runner_up_probability FROM preseason_team_projections WHERE projection_id = ? ORDER BY team_id",
    ).bind(&header.id).fetch_all(pool).await?
        .into_iter().map(|row| Ok(TeamProjection {
            team_id: Uuid::parse_str(&row.team_id)?,
            projected_table_position: optional_u32(row.projected_table_position)?,
            top_four_probability: row.top_four_probability,
            champion_probability: row.champion_probability,
            runner_up_probability: row.runner_up_probability,
        })).collect::<PersistenceResult<Vec<_>>>()?;
    let stages = sqlx::query_as::<_, StageRow>(
        "SELECT stage_index, team_id, reach_probability, mean_position, median_position, modal_position, position_probabilities_json FROM preseason_stage_team_projections WHERE projection_id = ? ORDER BY stage_index, team_id",
    ).bind(&header.id).fetch_all(pool).await?
        .into_iter().map(|row| Ok(StageTeamProjection {
            stage_index: required_u32(row.stage_index)?,
            team_id: Uuid::parse_str(&row.team_id)?,
            reach_probability: row.reach_probability,
            mean_position: row.mean_position,
            median_position: optional_u32(row.median_position)?,
            modal_position: optional_u32(row.modal_position)?,
            position_probabilities: serde_json::from_str(&row.position_probabilities_json)
                .map_err(|error| PersistenceError::InvalidData(error.to_string()))?,
        })).collect::<PersistenceResult<Vec<_>>>()?;
    let rounds = sqlx::query_as::<_, RoundRow>(
        "SELECT stage_index, round_index, team_id, reach_probability FROM preseason_round_team_projections WHERE projection_id = ? ORDER BY stage_index, round_index, team_id",
    ).bind(&header.id).fetch_all(pool).await?
        .into_iter().map(|row| Ok(RoundTeamProjection {
            stage_index: required_u32(row.stage_index)?,
            round_index: required_u32(row.round_index)?,
            team_id: Uuid::parse_str(&row.team_id)?,
            reach_probability: row.reach_probability,
        })).collect::<PersistenceResult<Vec<_>>>()?;
    let encounters = sqlx::query_as::<_, EncounterRow>(
        "SELECT stage_index, round_index, first_team_id, second_team_id, probability FROM preseason_encounter_projections WHERE projection_id = ? ORDER BY stage_index, round_index, first_team_id, second_team_id",
    ).bind(&header.id).fetch_all(pool).await?
        .into_iter().map(|row| Ok(EncounterProjection {
            stage_index: required_u32(row.stage_index)?,
            round_index: required_u32(row.round_index)?,
            first_team_id: Uuid::parse_str(&row.first_team_id)?,
            second_team_id: Uuid::parse_str(&row.second_team_id)?,
            probability: row.probability,
        })).collect::<PersistenceResult<Vec<_>>>()?;
    Ok(Some(StoredPreseasonProjection {
        id: Uuid::parse_str(&header.id)?,
        calibration_id: Uuid::parse_str(&header.calibration_id)?,
        generated_year: header.generated_year,
        generated_day_of_year: required_u32(header.generated_day_of_year)?,
        power_seed_model_version: required_u32(header.power_seed_model_version)?,
        projection: PreseasonProjection {
            season_instance_id: Uuid::parse_str(&header.season_instance_id)?,
            model_version: required_u32(header.model_version)?,
            simulation_count: required_u32(header.simulation_count)?,
            random_seed: header
                .random_seed
                .parse()
                .map_err(|_| PersistenceError::InvalidData("invalid projection seed".into()))?,
            config: PreseasonConfig {
                simulation_count: required_u32(header.simulation_count)?,
                historical_rating_sigma: header.historical_rating_sigma,
                new_team_rating_sigma: header.new_team_rating_sigma,
                knockout_margin_mean: header.knockout_margin_mean,
            },
            goal_point_model: header
                .goal_point_mean
                .zip(header.goal_point_rating_slope)
                .map(|(mean_per_team, rating_slope)| GoalPointModel {
                    mean_per_team,
                    rating_slope,
                }),
            teams,
            stages,
            rounds,
            encounters,
        },
    }))
}

fn required_u32(value: i64) -> PersistenceResult<u32> {
    u32::try_from(value)
        .map_err(|_| PersistenceError::InvalidData("invalid projection count".into()))
}

fn optional_u32(value: Option<i64>) -> PersistenceResult<Option<u32>> {
    value.map(required_u32).transpose()
}
