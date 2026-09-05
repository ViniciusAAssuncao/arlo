use crate::weighting::calculate_weighted_saturated_average;
use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_MULTIPLIER, ATTRIBUTE_SATURATION_THRESHOLD, CA_FORMULA_MULTIPLIER,
    CA_FORMULA_OFFSET, MAX_CURRENT_ABILITY, MIN_CURRENT_ABILITY,
};

pub fn calculate_current_ability(attributes_and_weights: &[(f64, f64)]) -> i32 {
    let waa = match calculate_weighted_saturated_average(
        attributes_and_weights,
        ATTRIBUTE_SATURATION_THRESHOLD,
        ATTRIBUTE_SATURATION_MULTIPLIER,
    ) {
        Some(avg) => avg,
        None => return MIN_CURRENT_ABILITY,
    };

    let raw_ca = (waa * CA_FORMULA_MULTIPLIER) - CA_FORMULA_OFFSET;
    let rounded_ca = raw_ca.round() as i32;

    rounded_ca.clamp(MIN_CURRENT_ABILITY, MAX_CURRENT_ABILITY)
}