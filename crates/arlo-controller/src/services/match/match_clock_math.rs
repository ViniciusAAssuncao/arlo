use arlo_persistence::models::MatchRow;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MatchClockDurationConfig {
    pub regulation_periods: u32,
    pub regulation_period_duration_seconds: f64,
    pub overtime_period_duration_seconds: f64,
}

impl MatchClockDurationConfig {
    pub fn new(
        regulation_periods: u32,
        regulation_period_duration_seconds: f64,
        overtime_period_duration_seconds: f64,
    ) -> Self {
        Self {
            regulation_periods,
            regulation_period_duration_seconds,
            overtime_period_duration_seconds,
        }
    }

    pub fn from_match_row(row: &MatchRow) -> Self {
        Self {
            regulation_periods: row.format_regulation_periods.max(0) as u32,
            regulation_period_duration_seconds: row
                .format_regulation_period_duration_seconds
                .max(0) as f64,
            overtime_period_duration_seconds: row
                .format_overtime_period_duration_seconds
                .max(0) as f64,
        }
    }
}

pub fn calculate_total_elapsed_seconds(
    config: &MatchClockDurationConfig,
    period: u32,
    seconds_in_period: f64,
) -> f64 {
    let bounded_seconds = seconds_in_period.max(0.0);
    if period <= 1 {
        bounded_seconds
    } else if period <= config.regulation_periods {
        let prev_periods = (period - 1) as f64;
        prev_periods * config.regulation_period_duration_seconds + bounded_seconds
    } else {
        let reg_total =
            config.regulation_periods as f64 * config.regulation_period_duration_seconds;
        let ot_periods = (period - config.regulation_periods - 1) as f64;
        reg_total + ot_periods * config.overtime_period_duration_seconds + bounded_seconds
    }
}

pub fn calculate_total_match_duration_seconds(
    config: &MatchClockDurationConfig,
    final_period: u32,
) -> f64 {
    if final_period <= config.regulation_periods {
        config.regulation_periods as f64 * config.regulation_period_duration_seconds
    } else {
        let reg_total =
            config.regulation_periods as f64 * config.regulation_period_duration_seconds;
        let ot_periods = (final_period - config.regulation_periods) as f64;
        reg_total + ot_periods * config.overtime_period_duration_seconds
    }
}

pub fn format_elapsed_time(seconds: f64) -> String {
    let total_secs = seconds.max(0.0).round() as u64;
    let mins = total_secs / 60;
    let secs = total_secs % 60;
    format!("{:02}:{:02}", mins, secs)
}