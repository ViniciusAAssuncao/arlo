use crate::physical::models::aerobic::calculate_player_age;
use arlo_domain::sport_constants::{
    AGE_INFLECTION_POINT_YEARS, AGE_MAX_RISK_MULTIPLIER, AGE_RISK_LOGISTIC_SLOPE,
};
use arlo_domain::Player;
use arlo_math::stats::contrast::logistic;

pub fn calculate_age_risk_multiplier(age_years: f64) -> f64 {
    let logit = AGE_RISK_LOGISTIC_SLOPE * (age_years - AGE_INFLECTION_POINT_YEARS);
    1.0 + (AGE_MAX_RISK_MULTIPLIER - 1.0) * logistic(logit)
}

pub fn derive_player_age_risk_multiplier(player: &Player, current_unix_seconds: i64) -> f64 {
    let age_years = calculate_player_age(player, current_unix_seconds);
    calculate_age_risk_multiplier(age_years)
}
