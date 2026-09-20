use crate::domain::calendar::{BlackoutWindow, CalendarDate, CalendarSystem};
use crate::services::calendar::date_advancer;

pub fn skip_forward_past_blackout(
    calendar: &CalendarSystem,
    date: &CalendarDate,
    windows: &[BlackoutWindow],
) -> CalendarDate {
    let mut current = *date;
    loop {
        let mut advanced = false;
        for window in windows {
            if window.contains(&current) {
                current = date_advancer::advance(calendar, &window.end(), 1);
                advanced = true;
                break;
            }
        }
        if !advanced {
            break;
        }
    }
    current
}