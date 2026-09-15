use crate::attributes::RefereeAttributeTable;
use crate::officiating::added_time::event_stimulus::calculate_raw_stimulus_seconds;
use crate::officiating::added_time::period_weighting::period_added_time_multiplier;
use crate::officiating::added_time::referee_recognition::referee_recognition_scale;
use crate::officiating::added_time::stoppage_log::PeriodStoppageLog;
use arlo_domain::MatchFormatRules;

pub fn estimate_added_time_seconds(
    log: &PeriodStoppageLog,
    referee_table: &RefereeAttributeTable,
    period: u32,
    format_rules: &MatchFormatRules,
) -> f64 {
    let raw_stimulus = calculate_raw_stimulus_seconds(log);
    let period_multiplier = period_added_time_multiplier(period, format_rules);
    let recognition_scale = referee_recognition_scale(referee_table);
    raw_stimulus * period_multiplier * recognition_scale
}
