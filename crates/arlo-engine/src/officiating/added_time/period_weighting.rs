use crate::officiating::added_time::eligibility::{
    is_first_half_end, is_overtime_period, is_second_half_end,
};
use arlo_domain::sport_constants::{
    ADDED_TIME_FIRST_HALF_MULTIPLIER, ADDED_TIME_OVERTIME_MULTIPLIER,
    ADDED_TIME_SECOND_HALF_MULTIPLIER,
};
use arlo_domain::MatchFormatRules;

pub fn period_added_time_multiplier(period: u32, format_rules: &MatchFormatRules) -> f64 {
    if is_overtime_period(period, format_rules) {
        ADDED_TIME_OVERTIME_MULTIPLIER
    } else if is_second_half_end(period, format_rules) {
        ADDED_TIME_SECOND_HALF_MULTIPLIER
    } else if is_first_half_end(period, format_rules) {
        ADDED_TIME_FIRST_HALF_MULTIPLIER
    } else {
        0.0
    }
}
