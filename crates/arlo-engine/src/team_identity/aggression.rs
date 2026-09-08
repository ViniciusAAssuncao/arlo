use arlo_domain::sport_constants::HOME_FIELD_ADVANTAGE_LOGIT;
use arlo_tactics::Aggression;

pub fn duel_logit_offset(aggression: Aggression) -> f64 {
    -aggression.value() * HOME_FIELD_ADVANTAGE_LOGIT
}

pub fn intensity_multiplier_scale(aggression: Aggression) -> f64 {
    1.0 + aggression.value()
}
