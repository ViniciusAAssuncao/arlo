use super::get_or_create_calibration;
use crate::domain::calendar::CalendarDate;
use crate::dto::season::MatchPredictionDto;
use crate::error::{ControllerError, ControllerResult};
use crate::services::season::power_ranking::load_preseason_seeds;
use arlo_analytics::{
    predict_match, PowerRankingConfig, PowerRating, MATCH_PREDICTION_MODEL_VERSION,
    POWER_RANKING_MODEL_VERSION,
};
use arlo_persistence::repositories::season::{
    fixtures, forecast_calibrations, season_instances, season_stages,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_match_prediction(
    pool: &SqlitePool,
    fixture_id: Uuid,
    as_of: CalendarDate,
) -> ControllerResult<MatchPredictionDto> {
    let fixture = fixtures::get_by_id(pool, fixture_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("fixture {fixture_id} not found")))?;
    if fixture.status == "Completed" || fixture.status == "Walkover" {
        return Err(ControllerError::Validation(
            "fixture is already decided".into(),
        ));
    }
    let stage = season_stages::get_by_id(pool, Uuid::parse_str(&fixture.season_stage_id)?)
        .await?
        .ok_or_else(|| ControllerError::NotFound("fixture stage not found".into()))?;
    let season_id = Uuid::parse_str(&stage.season_instance_id)?;
    let season = season_instances::get_by_id(pool, season_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound("fixture season not found".into()))?;
    let (calibration_id, calibration) = get_or_create_calibration(
        pool,
        season_id,
        Uuid::parse_str(&season.competition_id)?,
        as_of,
    )
    .await?;
    let home_id = Uuid::parse_str(&fixture.home_team_id)?;
    let away_id = Uuid::parse_str(&fixture.away_team_id)?;
    let snapshot = forecast_calibrations::latest_pair_ratings(
        pool,
        season_id,
        home_id,
        away_id,
        as_of.year(),
        as_of.day_of_year(),
        POWER_RANKING_MODEL_VERSION,
    )
    .await?;
    let (snapshot_id, home_rating, away_rating) = if let Some((id, home, away)) = snapshot {
        (Some(Uuid::parse_str(&id)?), home, away)
    } else {
        let seeds =
            load_preseason_seeds(pool, season_id, as_of, &PowerRankingConfig::default()).await?;
        let home = seeds
            .iter()
            .find(|seed| seed.team_id() == home_id)
            .ok_or_else(|| ControllerError::InvalidData("home rating missing".into()))?
            .initial_rating()
            .value();
        let away = seeds
            .iter()
            .find(|seed| seed.team_id() == away_id)
            .ok_or_else(|| ControllerError::InvalidData("away rating missing".into()))?
            .initial_rating()
            .value();
        (None, home, away)
    };
    let probabilities = predict_match(
        PowerRating::new(home_rating)?,
        PowerRating::new(away_rating)?,
        fixture.is_neutral_venue,
        calibration.parameters,
    )?;
    Ok(MatchPredictionDto {
        fixture_id,
        home_win: probabilities.home_win,
        draw: probabilities.draw,
        away_win: probabilities.away_win,
        power_snapshot_id: snapshot_id,
        calibration_id,
        model_version: MATCH_PREDICTION_MODEL_VERSION,
        generated_as_of: as_of,
    })
}
