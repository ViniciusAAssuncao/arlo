use crate::world_state::core::constants::*;

pub fn calculate_urgency_index(score_deficit: i32, time_urgency: f64) -> f64 {
    if score_deficit > 0 {
        ((score_deficit as f64)
            * TRAILING_URGENCY_DEFICIT_FACTOR
            * (TRAILING_URGENCY_BASE_WEIGHT + TRAILING_URGENCY_TIME_WEIGHT * time_urgency))
            .clamp(0.0, MAX_TRAILING_URGENCY_INDEX)
    } else if score_deficit < 0 {
        ((-score_deficit as f64) * LEADING_URGENCY_DEFICIT_FACTOR * time_urgency)
            .clamp(0.0, MAX_LEADING_URGENCY_INDEX)
    } else {
        (TIED_URGENCY_BASE_FACTOR * time_urgency).clamp(0.0, MAX_TIED_URGENCY_INDEX)
    }
}