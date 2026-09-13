use crate::domain::calendar::{
    CalendarDate, CalendarSystem, IntercalationPlacement, ResolvedCalendarDate,
};
use crate::error::{ControllerError, ControllerResult};
use crate::services::calendar::leap_year_calculator::is_leap_year;

pub fn encode(
    calendar: &CalendarSystem,
    resolved: &ResolvedCalendarDate,
) -> ControllerResult<CalendarDate> {
    match resolved {
        ResolvedCalendarDate::IntercalaryDay {
            year,
            intercalary_index,
            ..
        } => {
            let leap = is_leap_year(calendar.intercalation_rule(), *year);
            let intercalary_days = if leap {
                calendar.intercalation_rule().days_per_occurrence()
            } else {
                0
            };

            if !leap || intercalary_days == 0 {
                return Err(ControllerError::Validation(format!(
                    "Year {} is not a leap year or has no intercalary days",
                    year
                )));
            }

            if *intercalary_index >= intercalary_days {
                return Err(ControllerError::Validation(format!(
                    "Intercalary index {} exceeds maximum {} for year {}",
                    intercalary_index,
                    intercalary_days - 1,
                    year
                )));
            }

            match calendar.intercalation_rule().placement() {
                IntercalationPlacement::BeforeFirstMonth => {
                    Ok(CalendarDate::new(*year, *intercalary_index))
                }
                IntercalationPlacement::AfterLastMonth => {
                    let regular_days: u32 =
                        calendar.months().iter().map(|m| m.day_count()).sum();
                    Ok(CalendarDate::new(*year, regular_days + *intercalary_index))
                }
                IntercalationPlacement::AppendToMonth { .. } => {
                    Err(ControllerError::Validation(
                        "Calendar system uses AppendToMonth and does not have standalone intercalary days"
                            .to_string(),
                    ))
                }
            }
        }
        ResolvedCalendarDate::RegularDay {
            year,
            month_order_index,
            day_of_month,
            ..
        } => {
            if *day_of_month == 0 {
                return Err(ControllerError::Validation(
                    "day_of_month must be at least 1".to_string(),
                ));
            }

            let target_month = calendar
                .months()
                .iter()
                .find(|m| m.order_index() == *month_order_index)
                .ok_or_else(|| {
                    ControllerError::Validation(format!(
                        "Month order index {} not found",
                        month_order_index
                    ))
                })?;

            let leap = is_leap_year(calendar.intercalation_rule(), *year);
            let intercalary_days = if leap {
                calendar.intercalation_rule().days_per_occurrence()
            } else {
                0
            };

            let max_days = match calendar.intercalation_rule().placement() {
                IntercalationPlacement::AppendToMonth {
                    month_order_index: target_idx,
                } if target_idx == *month_order_index && leap => {
                    target_month.day_count() + intercalary_days
                }
                _ => target_month.day_count(),
            };

            if *day_of_month > max_days {
                return Err(ControllerError::Validation(format!(
                    "day_of_month {} exceeds maximum {} for month {}",
                    day_of_month, max_days, month_order_index
                )));
            }

            let mut offset = match calendar.intercalation_rule().placement() {
                IntercalationPlacement::BeforeFirstMonth => intercalary_days,
                _ => 0,
            };

            for month in calendar.months() {
                if month.order_index() == *month_order_index {
                    break;
                }

                let month_len = match calendar.intercalation_rule().placement() {
                    IntercalationPlacement::AppendToMonth {
                        month_order_index: target_idx,
                    } if target_idx == month.order_index() && leap => {
                        month.day_count() + intercalary_days
                    }
                    _ => month.day_count(),
                };

                offset += month_len;
            }

            offset += *day_of_month - 1;
            Ok(CalendarDate::new(*year, offset))
        }
    }
}