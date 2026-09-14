use crate::domain::calendar::{CalendarDate, CalendarSystem, ResolvedCalendarDate};
use crate::error::{ControllerError, ControllerResult};
use crate::services::calendar::{date_advancer, date_encoder, date_resolver};

pub fn subtract_days(calendar: &CalendarSystem, date: &CalendarDate, days: i64) -> CalendarDate {
    date_advancer::advance(calendar, date, -days)
}

pub fn subtract_months(
    calendar: &CalendarSystem,
    date: &CalendarDate,
    months_to_subtract: u32,
) -> ControllerResult<CalendarDate> {
    if months_to_subtract == 0 {
        return Ok(*date);
    }

    if calendar.months().is_empty() {
        return Err(ControllerError::Validation(
            "Calendar system has no defined months".to_string(),
        ));
    }

    let resolved = date_resolver::resolve(calendar, date);
    let (year, month_order_index, day_of_month) = match resolved {
        ResolvedCalendarDate::RegularDay {
            year,
            month_order_index,
            day_of_month,
            ..
        } => (year, month_order_index, day_of_month),
        ResolvedCalendarDate::IntercalaryDay { year, .. } => (year, 0, 1),
    };

    let total_months = calendar.months().len() as i64;
    let net_month_offset = month_order_index as i64 - months_to_subtract as i64;
    let year_delta = net_month_offset.div_euclid(total_months);
    let target_month_idx = net_month_offset.rem_euclid(total_months) as u32;
    let target_year = year + year_delta;

    let target_month_def = calendar
        .months()
        .iter()
        .find(|m| m.order_index() == target_month_idx)
        .ok_or_else(|| {
            ControllerError::Validation(format!(
                "Target month index {} not found in calendar definition",
                target_month_idx
            ))
        })?;

    let clamped_day = day_of_month.min(target_month_def.day_count());

    let target_resolved = ResolvedCalendarDate::RegularDay {
        year: target_year,
        month_order_index: target_month_idx,
        day_of_month: clamped_day,
        week_day_index: 0,
    };

    date_encoder::encode(calendar, &target_resolved)
}