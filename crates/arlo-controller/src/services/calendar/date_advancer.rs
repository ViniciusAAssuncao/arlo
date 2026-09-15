use crate::domain::calendar::{CalendarDate, CalendarSystem};
use crate::services::calendar::year_length_calculator::total_days_in_year;

pub fn advance(
    calendar: &CalendarSystem,
    date: &CalendarDate,
    delta_days: i64,
) -> CalendarDate {
    let mut year = date.year();
    let mut day = date.day_of_year() as i64 + delta_days;

    while day < 0 {
        year -= 1;
        let days_in_year = total_days_in_year(calendar, year) as i64;
        day += days_in_year;
    }

    loop {
        let days_in_year = total_days_in_year(calendar, year) as i64;
        if day < days_in_year {
            break;
        }
        day -= days_in_year;
        year += 1;
    }

    CalendarDate::new(year, day as u32)
}