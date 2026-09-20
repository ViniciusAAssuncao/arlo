use crate::domain::calendar::{BlackoutWindow, CalendarSystem, ResolvedCalendarDate};
use crate::error::ControllerResult;
use crate::services::calendar::date_encoder;
use arlo_domain::{CollectiveAgreement, CollectiveAgreementRule};
use std::ops::RangeInclusive;

pub fn resolve_collective_agreement_windows(
    calendar: &CalendarSystem,
    agreement: &CollectiveAgreement,
    years: RangeInclusive<i64>,
) -> ControllerResult<Vec<BlackoutWindow>> {
    match agreement.rule() {
        CollectiveAgreementRule::AnnualBlackoutWindow(rule) => {
            let mut windows = Vec::new();
            for year in years {
                let start_resolved = ResolvedCalendarDate::RegularDay {
                    year,
                    month_order_index: rule.start_month_order_index(),
                    day_of_month: rule.start_day_of_month(),
                    week_day_index: 0,
                };
                let start_date = date_encoder::encode(calendar, &start_resolved)?;

                let end_year = year + rule.end_year_offset() as i64;
                let end_resolved = ResolvedCalendarDate::RegularDay {
                    year: end_year,
                    month_order_index: rule.end_month_order_index(),
                    day_of_month: rule.end_day_of_month(),
                    week_day_index: 0,
                };
                let end_date = date_encoder::encode(calendar, &end_resolved)?;

                windows.push(BlackoutWindow::new(start_date, end_date));
            }
            Ok(windows)
        }
    }
}