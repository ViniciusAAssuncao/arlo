use arlo_domain::MatchFormatRules;

pub fn is_first_half_end(period: u32, format_rules: &MatchFormatRules) -> bool {
    period == format_rules.regulation_periods() / 2
}

pub fn is_second_half_end(period: u32, format_rules: &MatchFormatRules) -> bool {
    period == format_rules.regulation_periods()
}

pub fn is_overtime_period(period: u32, format_rules: &MatchFormatRules) -> bool {
    period > format_rules.regulation_periods()
}

pub fn is_added_time_eligible_period(period: u32, format_rules: &MatchFormatRules) -> bool {
    is_first_half_end(period, format_rules)
        || is_second_half_end(period, format_rules)
        || is_overtime_period(period, format_rules)
}
