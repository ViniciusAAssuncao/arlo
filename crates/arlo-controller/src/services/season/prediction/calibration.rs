use crate::domain::calendar::CalendarDate;
use crate::error::{ControllerError, ControllerResult};
use arlo_analytics::{
    calibrate_forecast, CalibrationHistory, CalibrationScope, ForecastCalibration,
    ForecastObservation, ForecastParameters, PowerRating, PredictionOutcome,
    MATCH_PREDICTION_MODEL_VERSION, POWER_RANKING_MODEL_VERSION,
};
use arlo_persistence::repositories::season::forecast_calibrations::{
    self, ForecastCalibrationRow, ForecastHistoryRow,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_or_create_calibration(
    pool: &SqlitePool,
    season_id: Uuid,
    competition_id: Uuid,
    as_of: CalendarDate,
) -> ControllerResult<(Uuid, ForecastCalibration)> {
    if let Some(row) =
        forecast_calibrations::get(pool, season_id, MATCH_PREDICTION_MODEL_VERSION).await?
    {
        return map_row(row);
    }
    let (cutoff_year, cutoff_day) = forecast_calibrations::first_fixture_date(pool, season_id)
        .await?
        .ok_or_else(|| ControllerError::InvalidData("season has no scheduled fixtures".into()))?;
    let federation_id =
        sqlx::query_scalar::<_, String>("SELECT federation_id FROM competitions WHERE id = ?")
            .bind(competition_id.to_string())
            .fetch_one(pool)
            .await?;
    let cutoff_day = u32::try_from(cutoff_day)
        .map_err(|_| ControllerError::InvalidData("invalid calibration cutoff".into()))?;
    let competition_rows = forecast_calibrations::list_history(
        pool,
        cutoff_year,
        cutoff_day,
        POWER_RANKING_MODEL_VERSION,
        Some(competition_id),
        None,
        2000,
    )
    .await?;
    let federation_rows = if competition_rows.len() < 100 {
        forecast_calibrations::list_history(
            pool,
            cutoff_year,
            cutoff_day,
            POWER_RANKING_MODEL_VERSION,
            None,
            Some(&federation_id),
            2000,
        )
        .await?
    } else {
        Vec::new()
    };
    let global_rows = if competition_rows.len() < 100 && federation_rows.len() < 100 {
        forecast_calibrations::list_history(
            pool,
            cutoff_year,
            cutoff_day,
            POWER_RANKING_MODEL_VERSION,
            None,
            None,
            5000,
        )
        .await?
    } else {
        Vec::new()
    };
    let competition = observations(&competition_rows)?;
    let federation = observations(&federation_rows)?;
    let global = observations(&global_rows)?;
    let calibration = calibrate_forecast(
        CalibrationHistory {
            competition: &competition,
            federation: &federation,
            global: &global,
        },
        100,
        500.0,
        400.0,
    )?;
    let id = Uuid::new_v4();
    let row = ForecastCalibrationRow {
        id: id.to_string(),
        season_instance_id: season_id.to_string(),
        model_version: calibration.model_version as i64,
        generated_year: as_of.year(),
        generated_day_of_year: as_of.day_of_year() as i64,
        cutoff_year,
        cutoff_day_of_year: cutoff_day as i64,
        scope: format!("{:?}", calibration.scope),
        sample_count: calibration.sample_count as i64,
        home_advantage: calibration.parameters.home_advantage,
        draw_propensity: calibration.parameters.draw_propensity,
        temperature: calibration.parameters.temperature,
        rating_scale: calibration.parameters.rating_scale,
    };
    forecast_calibrations::insert(pool, &row).await?;
    map_row(
        forecast_calibrations::get(pool, season_id, MATCH_PREDICTION_MODEL_VERSION)
            .await?
            .ok_or_else(|| ControllerError::InvalidData("calibration publication failed".into()))?,
    )
}

fn observations(rows: &[ForecastHistoryRow]) -> ControllerResult<Vec<ForecastObservation>> {
    rows.iter()
        .map(|row| {
            Ok(ForecastObservation {
                home_rating: PowerRating::new(row.home_rating)?,
                away_rating: PowerRating::new(row.away_rating)?,
                neutral_venue: row.neutral_venue,
                outcome: match row.home_score.cmp(&row.away_score) {
                    std::cmp::Ordering::Greater => PredictionOutcome::HomeWin,
                    std::cmp::Ordering::Equal => PredictionOutcome::Draw,
                    std::cmp::Ordering::Less => PredictionOutcome::AwayWin,
                },
            })
        })
        .collect()
}

fn map_row(row: ForecastCalibrationRow) -> ControllerResult<(Uuid, ForecastCalibration)> {
    let scope = match row.scope.as_str() {
        "Competition" => CalibrationScope::Competition,
        "Federation" => CalibrationScope::Federation,
        "Global" => CalibrationScope::Global,
        "Prior" => CalibrationScope::Prior,
        _ => {
            return Err(ControllerError::InvalidData(
                "invalid calibration scope".into(),
            ))
        }
    };
    let parameters = ForecastParameters {
        home_advantage: row.home_advantage,
        draw_propensity: row.draw_propensity,
        temperature: row.temperature,
        rating_scale: row.rating_scale,
    };
    parameters.validate()?;
    Ok((
        Uuid::parse_str(&row.id)?,
        ForecastCalibration {
            model_version: u32::try_from(row.model_version)
                .map_err(|_| ControllerError::InvalidData("invalid calibration version".into()))?,
            scope,
            sample_count: u32::try_from(row.sample_count).map_err(|_| {
                ControllerError::InvalidData("invalid calibration sample count".into())
            })?,
            parameters,
        },
    ))
}
