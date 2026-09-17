use crate::domain::calendar::{CalendarSystem, ResolvedCalendarDate};
use crate::dto::calendar::{CalendarMonthDayDto, CalendarMonthViewDto};
use crate::error::{ControllerError, ControllerResult};
use crate::services::calendar::{date_encoder, date_resolver};

pub fn build_month_view(
    calendar: &CalendarSystem,
    year: i64,
    month_order_index: u32,
) -> ControllerResult<CalendarMonthViewDto> {
    let month = calendar
        .months()
        .iter()
        .find(|m| m.order_index() == month_order_index)
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Mês com índice de ordem {} não encontrado no calendário",
                month_order_index
            ))
        })?;

    let month_name = month.name().to_string();
    let week_day_names = calendar
        .week_days()
        .iter()
        .map(|w| w.name().to_string())
        .collect();

    let mut days = Vec::with_capacity(month.day_count() as usize);

    for day_of_month in 1..=month.day_count() {
        let requested_date = ResolvedCalendarDate::RegularDay {
            year,
            month_order_index,
            day_of_month,
            week_day_index: 0,
        };

        let encoded = date_encoder::encode(calendar, &requested_date)?;
        let resolved = date_resolver::resolve(calendar, &encoded);

        let week_day_order_index = match resolved {
            ResolvedCalendarDate::RegularDay {
                week_day_index, ..
            } => week_day_index,
            ResolvedCalendarDate::IntercalaryDay {
                week_day_index, ..
            } => week_day_index.unwrap_or(0),
        };

        days.push(CalendarMonthDayDto {
            day_of_month,
            week_day_order_index,
        });
    }

    Ok(CalendarMonthViewDto {
        month_name,
        year,
        week_day_names,
        days,
    })
}