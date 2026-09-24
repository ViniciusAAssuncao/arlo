use crate::domain::calendar::IntercalationRule;

fn floor_div(a: i64, b: i64) -> i64 {
    let d = a / b;
    let r = a % b;
    if (r != 0) && ((a < 0) ^ (b < 0)) {
        d - 1
    } else {
        d
    }
}

pub fn is_leap_year(rule: &IntercalationRule, year: i64) -> bool {
    if rule.leap_units_per_cycle() == 0 || rule.cycle_length_years() <= 0 {
        return false;
    }

    let y = year - rule.cycle_reference_year();
    let current_bucket = floor_div(y * rule.leap_units_per_cycle(), rule.cycle_length_years());
    let prev_bucket = floor_div(
        (y - 1) * rule.leap_units_per_cycle(),
        rule.cycle_length_years(),
    );

    current_bucket > prev_bucket
}
