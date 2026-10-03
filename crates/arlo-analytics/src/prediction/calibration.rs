use super::match_forecast::{predict_match, ForecastParameters, MATCH_PREDICTION_MODEL_VERSION};
use crate::error::{AnalyticsError, AnalyticsResult};
use crate::power_ranking::PowerRating;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PredictionOutcome {
    HomeWin,
    Draw,
    AwayWin,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ForecastObservation {
    pub home_rating: PowerRating,
    pub away_rating: PowerRating,
    pub neutral_venue: bool,
    pub outcome: PredictionOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalibrationScope {
    Competition,
    Federation,
    Global,
    Prior,
}

#[derive(Debug, Clone, Default)]
pub struct CalibrationHistory<'a> {
    pub competition: &'a [ForecastObservation],
    pub federation: &'a [ForecastObservation],
    pub global: &'a [ForecastObservation],
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ForecastCalibration {
    pub model_version: u32,
    pub scope: CalibrationScope,
    pub sample_count: u32,
    pub parameters: ForecastParameters,
}

pub fn calibrate_forecast(
    history: CalibrationHistory<'_>,
    minimum_local_samples: usize,
    shrinkage_samples: f64,
    draw_shrinkage_samples: f64,
    rating_scale: f64,
) -> AnalyticsResult<ForecastCalibration> {
    if !shrinkage_samples.is_finite()
        || shrinkage_samples <= 0.0
        || !draw_shrinkage_samples.is_finite()
        || draw_shrinkage_samples <= 0.0
    {
        return Err(AnalyticsError::InvalidData(
            "shrinkage parameters must be positive".into(),
        ));
    }
    let mut prior = ForecastParameters::default();
    prior.rating_scale = rating_scale;
    prior.validate()?;
    let (observations, scope) = if history.competition.len() >= minimum_local_samples {
        (history.competition, CalibrationScope::Competition)
    } else if history.federation.len() >= minimum_local_samples {
        (history.federation, CalibrationScope::Federation)
    } else if !history.global.is_empty() {
        (history.global, CalibrationScope::Global)
    } else {
        (&[][..], CalibrationScope::Prior)
    };
    let sample_count = u32::try_from(observations.len())
        .map_err(|_| AnalyticsError::InvalidData("too many calibration observations".into()))?;
    if observations.is_empty() {
        return Ok(ForecastCalibration {
            model_version: MATCH_PREDICTION_MODEL_VERSION,
            scope,
            sample_count,
            parameters: prior,
        });
    }
    let mut fitted = prior;
    let mut best = loss(observations, fitted)?;
    for step in [100.0, 40.0, 15.0, 5.0] {
        for candidate in (-4..=4).map(|n| n as f64 * step) {
            let mut trial = fitted;
            trial.home_advantage = candidate;
            consider(observations, trial, &mut fitted, &mut best)?;
        }
    }
    for step in [0.2, 0.08, 0.025] {
        for offset in -5..=5 {
            let mut trial = fitted;
            trial.draw_propensity =
                (fitted.draw_propensity + offset as f64 * step).clamp(0.05, 3.0);
            consider(observations, trial, &mut fitted, &mut best)?;
        }
    }
    for step in [0.25, 0.1, 0.03] {
        for offset in -5..=5 {
            let mut trial = fitted;
            trial.temperature = (fitted.temperature + offset as f64 * step).clamp(0.4, 4.0);
            consider(observations, trial, &mut fitted, &mut best)?;
        }
    }
    let weight = observations.len() as f64 / (observations.len() as f64 + shrinkage_samples);
    let draw_weight =
        observations.len() as f64 / (observations.len() as f64 + draw_shrinkage_samples);
    let parameters = ForecastParameters {
        home_advantage: weight * fitted.home_advantage,
        draw_propensity: 0.5 + draw_weight * (fitted.draw_propensity - 0.5),
        temperature: 1.0 + weight * (fitted.temperature - 1.0),
        rating_scale,
    };
    Ok(ForecastCalibration {
        model_version: MATCH_PREDICTION_MODEL_VERSION,
        scope,
        sample_count,
        parameters,
    })
}

fn consider(
    observations: &[ForecastObservation],
    trial: ForecastParameters,
    fitted: &mut ForecastParameters,
    best: &mut f64,
) -> AnalyticsResult<()> {
    let candidate_loss = loss(observations, trial)?;
    if candidate_loss < *best {
        *fitted = trial;
        *best = candidate_loss;
    }
    Ok(())
}

fn loss(
    observations: &[ForecastObservation],
    parameters: ForecastParameters,
) -> AnalyticsResult<f64> {
    let mut total = 0.0;
    for observation in observations {
        let probabilities = predict_match(
            observation.home_rating,
            observation.away_rating,
            observation.neutral_venue,
            parameters,
        )?;
        let probability = match observation.outcome {
            PredictionOutcome::HomeWin => probabilities.home_win,
            PredictionOutcome::Draw => probabilities.draw,
            PredictionOutcome::AwayWin => probabilities.away_win,
        };
        total -= probability.max(f64::MIN_POSITIVE).ln();
    }
    Ok(total)
}
