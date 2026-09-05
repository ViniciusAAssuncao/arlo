use arlo_domain::sport_constants::{
    ATTRIBUTE_SATURATION_MULTIPLIER, ATTRIBUTE_SATURATION_THRESHOLD, CA_FORMULA_MULTIPLIER,
    CA_FORMULA_OFFSET, MAX_CURRENT_ABILITY, MIN_CURRENT_ABILITY,
};

pub fn calculate_current_ability(attributes_and_weights: &[(f64, f64)]) -> i32 {
    let total_weight: f64 = attributes_and_weights.iter().map(|(_, w)| *w).sum();
    if total_weight <= 0.0 {
        return MIN_CURRENT_ABILITY;
    }

    let mut accumulated_total = 0.0;
    for &(value, weight) in attributes_and_weights {
        let saturated_value = if value > ATTRIBUTE_SATURATION_THRESHOLD {
            ATTRIBUTE_SATURATION_THRESHOLD
                + (value - ATTRIBUTE_SATURATION_THRESHOLD) * ATTRIBUTE_SATURATION_MULTIPLIER
        } else {
            value
        };
        accumulated_total += saturated_value * weight;
    }

    let waa = accumulated_total / total_weight;
    let raw_ca = (waa * CA_FORMULA_MULTIPLIER) - CA_FORMULA_OFFSET;
    let rounded_ca = raw_ca.round() as i32;

    rounded_ca.clamp(MIN_CURRENT_ABILITY, MAX_CURRENT_ABILITY)
}