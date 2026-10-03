use crate::error::PersistenceResult;
use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ForecastCalibrationRow {
    pub id: String,
    pub season_instance_id: String,
    pub model_version: i64,
    pub generated_year: i64,
    pub generated_day_of_year: i64,
    pub cutoff_year: i64,
    pub cutoff_day_of_year: i64,
    pub scope: String,
    pub sample_count: i64,
    pub home_advantage: f64,
    pub draw_propensity: f64,
    pub temperature: f64,
    pub rating_scale: f64,
}

pub async fn get(
    pool: &SqlitePool,
    season_id: Uuid,
    model_version: u32,
) -> PersistenceResult<Option<ForecastCalibrationRow>> {
    Ok(sqlx::query_as::<_, ForecastCalibrationRow>(
        "SELECT * FROM forecast_calibrations WHERE season_instance_id = ? AND model_version = ?",
    )
    .bind(season_id.to_string())
    .bind(model_version as i64)
    .fetch_optional(pool)
    .await?)
}

pub async fn insert(pool: &SqlitePool, row: &ForecastCalibrationRow) -> PersistenceResult<()> {
    sqlx::query(
        "INSERT OR IGNORE INTO forecast_calibrations (id, season_instance_id, model_version, generated_year, generated_day_of_year, cutoff_year, cutoff_day_of_year, scope, sample_count, home_advantage, draw_propensity, temperature, rating_scale) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&row.id)
    .bind(&row.season_instance_id)
    .bind(row.model_version)
    .bind(row.generated_year)
    .bind(row.generated_day_of_year)
    .bind(row.cutoff_year)
    .bind(row.cutoff_day_of_year)
    .bind(&row.scope)
    .bind(row.sample_count)
    .bind(row.home_advantage)
    .bind(row.draw_propensity)
    .bind(row.temperature)
    .bind(row.rating_scale)
    .execute(pool)
    .await?;
    Ok(())
}
