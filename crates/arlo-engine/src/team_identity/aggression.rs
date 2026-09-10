use arlo_domain::sport_constants::TACTICAL_STYLE_LOGIT_SCALE;
use arlo_tactics::Aggression;

pub fn duel_logit_offset(aggression: Aggression) -> f64 {
    -aggression.value() * TACTICAL_STYLE_LOGIT_SCALE
}

pub fn intensity_multiplier_scale(aggression: Aggression) -> f64 {
    1.0 + aggression.value()
}
