use crate::domain::calendar::{
    CalendarDate, CalendarSystem, IntercalationPlacement, ResolvedCalendarDate,
};
use crate::services::calendar::leap_year_calculator::is_leap_year;

pub fn resolve(calendar: &CalendarSystem, date: &CalendarDate) -> ResolvedCalendarDate {
    let leap = is_leap_year(calendar.intercalation_rule(), date.year());
    let intercalary_days = if leap {
        calendar.intercalation_rule().days_per_occurrence()
    } else {
        0
    };
    let day = date.day_of_year();

    match calendar.intercalation_rule().placement() {
        IntercalationPlacement::BeforeFirstMonth => {
            if day < intercalary_days {
                return ResolvedCalendarDate::IntercalaryDay {
                    year: date.year(),
                    intercalary_index: day,
                };
            }

            let mut remaining = day - intercalary_days;
            for month in calendar.months() {
                if remaining < month.day_count() {
                    return ResolvedCalendarDate::RegularDay {
                        year: date.year(),
                        month_order_index: month.order_index(),
                        day_of_month: remaining + 1,
                    };
                }
                remaining -= month.day_count();
            }

            let last_month = calendar.months().last().unwrap();
            ResolvedCalendarDate::RegularDay {
                year: date.year(),
                month_order_index: last_month.order_index(),
                day_of_month: last_month.day_count(),
            }
        }
        IntercalationPlacement::AfterLastMonth => {
            let mut remaining = day;
            for month in calendar.months() {
                if remaining < month.day_count() {
                    return ResolvedCalendarDate::RegularDay {
                        year: date.year(),
                        month_order_index: month.order_index(),
                        day_of_month: remaining + 1,
                    };
                }
                remaining -= month.day_count();
            }

            ResolvedCalendarDate::IntercalaryDay {
                year: date.year(),
                intercalary_index: remaining,
            }
        }
        IntercalationPlacement::AppendToMonth { month_order_index } => {
            let mut remaining = day;
            for month in calendar.months() {
                let month_len = month.day_count()
                    + if leap && month.order_index() == month_order_index {
                        intercalary_days
                    } else {
                        0
                    };

                if remaining < month_len {
                    return ResolvedCalendarDate::RegularDay {
                        year: date.year(),
                        month_order_index: month.order_index(),
                        day_of_month: remaining + 1,
                    };
                }
                remaining -= month_len;
            }

            let last_month = calendar.months().last().unwrap();
            let last_len = last_month.day_count()
                + if leap && last_month.order_index() == month_order_index {
                    intercalary_days
                } else {
                    0
                };

            ResolvedCalendarDate::RegularDay {
                year: date.year(),
                month_order_index: last_month.order_index(),
                day_of_month: last_len,
            }
        }
    }
}