use crate::error::{AnalyticsError, AnalyticsResult};
use crate::power_ranking::config::PowerRankingConfig;
use crate::power_ranking::rating::PowerRating;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MatchForecast {
    home_win: f64,
    draw: f64,
    away_win: f64,
}

impl MatchForecast {
    pub fn home_win(self) -> f64 {
        self.home_win
    }

    pub fn draw(self) -> f64 {
        self.draw
    }

    pub fn away_win(self) -> f64 {
        self.away_win
    }

    pub fn expected_home_score(self) -> f64 {
        self.home_win + 0.5 * self.draw
    }
}

pub fn forecast_match(
    home_rating: PowerRating,
    away_rating: PowerRating,
    neutral_venue: bool,
    config: &PowerRankingConfig,
) -> AnalyticsResult<MatchForecast> {
    config.validate()?;
    let advantage = if neutral_venue {
        0.0
    } else {
        config.home_advantage
    };
    let difference = (home_rating.value() + advantage - away_rating.value()) / config.rating_scale;
    if !difference.is_finite() {
        return Err(AnalyticsError::NonFinite {
            field: "rating_difference".into(),
        });
    }
    let home_log_weight = difference * std::f64::consts::LN_10 / 2.0;
    if !home_log_weight.is_finite() {
        return Err(AnalyticsError::NonFinite {
            field: "home_log_weight".into(),
        });
    }
    let away_log_weight = -home_log_weight;
    let draw_log_weight = if config.draw_propensity == 0.0 {
        f64::NEG_INFINITY
    } else {
        config.draw_propensity.ln()
    };
    let maximum = home_log_weight.max(away_log_weight).max(draw_log_weight);
    let home_weight = (home_log_weight - maximum).exp();
    let draw_weight = (draw_log_weight - maximum).exp();
    let away_weight = (away_log_weight - maximum).exp();
    let total = home_weight + draw_weight + away_weight;
    Ok(MatchForecast {
        home_win: home_weight / total,
        draw: draw_weight / total,
        away_win: away_weight / total,
    })
}
