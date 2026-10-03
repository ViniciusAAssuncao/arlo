use super::calibration::get_or_create_calibration;
use super::projection_plan::build_projection_plan;
use crate::domain::calendar::CalendarDate;
use crate::dto::season::PreseasonForecastDto;
use crate::error::{ControllerError, ControllerResult};
use arlo_analytics::prediction::preseason::{project_preseason, PreseasonConfig};
use arlo_analytics::POWER_RANKING_MODEL_VERSION;
use arlo_persistence::repositories::season::{
    forecast_calibrations, preseason_projections, season_instances,
};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;
use uuid::Uuid;

static PROJECTION_LOCKS: LazyLock<Mutex<HashMap<Uuid, Arc<Mutex<()>>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub async fn get_or_create_preseason_forecast(
    pool: &SqlitePool,
    season_id: Uuid,
    as_of: CalendarDate,
) -> ControllerResult<Option<PreseasonForecastDto>> {
    if let Some(stored) = preseason_projections::get(pool, season_id).await? {
        return Ok(Some(stored.into()));
    }
    let lock = {
        let mut locks = PROJECTION_LOCKS.lock().await;
        locks
            .entry(season_id)
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone()
    };
    let _guard = lock.lock().await;
    if let Some(stored) = preseason_projections::get(pool, season_id).await? {
        return Ok(Some(stored.into()));
    }
    let season = season_instances::get_by_id(pool, season_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("season {season_id} not found")))?;
    let Some((year, day)) = forecast_calibrations::first_fixture_date(pool, season_id).await?
    else {
        return Ok(None);
    };
    let cutoff = CalendarDate::new(
        year,
        u32::try_from(day)
            .map_err(|_| ControllerError::InvalidData("invalid first fixture day".into()))?,
    );
    if as_of > cutoff {
        return Ok(None);
    }
    let completed: Option<i64> = sqlx::query_scalar(
        "SELECT 1 FROM fixtures f JOIN season_stages s ON s.id = f.season_stage_id WHERE s.season_instance_id = ? AND (f.status = 'Completed' OR f.status = 'Walkover') LIMIT 1",
    )
    .bind(season_id.to_string()).fetch_optional(pool).await?;
    if completed.is_some() {
        return Ok(None);
    }
    let competition_id = Uuid::parse_str(&season.competition_id)?;
    let config = PreseasonConfig::default();
    let (calibration_id, calibration) =
        get_or_create_calibration(pool, season_id, competition_id, as_of).await?;
    let Some(plan) = build_projection_plan(
        pool,
        season_id,
        competition_id,
        season.reference_year,
        as_of,
        cutoff,
        calibration.parameters,
        calibration.scope,
        config,
    )
    .await?
    else {
        return Ok(None);
    };
    let projection = tokio::task::spawn_blocking(move || project_preseason(plan))
        .await
        .map_err(|error| ControllerError::InvalidData(error.to_string()))??;
    preseason_projections::publish(
        pool,
        &projection,
        calibration_id,
        as_of.year(),
        as_of.day_of_year(),
        POWER_RANKING_MODEL_VERSION,
    )
    .await?;
    Ok(preseason_projections::get(pool, season_id)
        .await?
        .map(Into::into))
}
