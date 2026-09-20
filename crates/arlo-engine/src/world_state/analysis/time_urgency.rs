use crate::world_state::core::constants::*;

pub fn calculate_time_urgency(total_remaining_seconds: f64) -> f64 {
    1.0 / (1.0 + (total_remaining_seconds / URGENCY_TIME_HALF_LIFE_SECONDS).powf(URGENCY_POWER_CURVE))
}

pub fn calculate_total_remaining_seconds(
    period: u32,
    regulation_periods: u32,
    seconds_in_period: f64,
    period_duration_seconds: f64,
) -> f64 {
    let rem_in_period = (period_duration_seconds - seconds_in_period).max(0.0);
    if period <= regulation_periods {
        ((regulation_periods - period) as f64) * period_duration_seconds + rem_in_period
    } else {
        rem_in_period
    }
}