use crate::domain::calendar::{CalendarDate, CalendarSystem};
use crate::domain::season::Fixture;
use crate::services::calendar::date_advancer;

pub fn calculate_stage_completion_date(
    calendar: &CalendarSystem,
    fixtures: &[Fixture],
) -> Option<CalendarDate> {
    let latest_date = fixtures.iter().map(|f| f.scheduled_date()).max()?;
    Some(date_advancer::advance(calendar, &latest_date, 1))
}
