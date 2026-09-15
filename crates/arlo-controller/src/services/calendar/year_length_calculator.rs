use crate::domain::calendar::CalendarSystem;
use crate::services::calendar::leap_year_calculator::is_leap_year;

pub fn total_days_in_year(calendar: &CalendarSystem, year: i64) -> u32 {
    let regular_days: u32 = calendar.months().iter().map(|m| m.day_count()).sum();

    if is_leap_year(calendar.intercalation_rule(), year) {
        regular_days + calendar.intercalation_rule().days_per_occurrence()
    } else {
        regular_days
    }
}