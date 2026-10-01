use crate::domain::calendar::{
    CalendarDate, CalendarSystem, IntercalationPlacement, ResolvedCalendarDate,
};
use crate::services::calendar::leap_year_calculator::is_leap_year;

fn floor_div(a: i64, b: i64) -> i64 {
    let d = a / b;
    let r = a % b;
    if (r != 0) && ((a < 0) ^ (b < 0)) {
        d - 1
    } else {
        d
    }
}

pub fn resolve(calendar: &CalendarSystem, date: &CalendarDate) -> ResolvedCalendarDate {
    let rule = calendar.intercalation_rule();
    let leap = is_leap_year(rule, date.year());
    let intercalary_days = if leap { rule.days_per_occurrence() } else { 0 };
    let day = date.day_of_year();
    let week_days_len = calendar.week_days().len() as i64;
    let regular_days_per_year: i64 = calendar.months().iter().map(|m| m.day_count() as i64).sum();

    let calculate_disrupted_week_day = || -> u32 {
        let leap_years = if rule.leap_units_per_cycle() == 0 || rule.cycle_length_years() <= 0 {
            0
        } else {
            floor_div(
                (date.year() - 1 - rule.cycle_reference_year()) * rule.leap_units_per_cycle(),
                rule.cycle_length_years(),
            ) - floor_div(
                (-1 - rule.cycle_reference_year()) * rule.leap_units_per_cycle(),
                rule.cycle_length_years(),
            )
        };
        let total_days = regular_days_per_year * date.year()
            + leap_years * rule.days_per_occurrence() as i64
            + day as i64;
        if week_days_len > 0 {
            total_days.rem_euclid(week_days_len) as u32
        } else {
            0
        }
    };

    let calculate_non_disrupted_regular_week_day =
        |month_order_index: u32, day_of_month: u32| -> u32 {
            let mut regular_days_before_month: i64 = 0;
            for m in calendar.months() {
                if m.order_index() < month_order_index {
                    regular_days_before_month += m.day_count() as i64;
                }
            }
            let total_regular_days = regular_days_per_year * date.year()
                + regular_days_before_month
                + (day_of_month as i64 - 1);
            if week_days_len > 0 {
                total_regular_days.rem_euclid(week_days_len) as u32
            } else {
                0
            }
        };

    let disrupts = rule.disrupts_week_cycle();

    match rule.placement() {
        IntercalationPlacement::BeforeFirstMonth => {
            if day < intercalary_days {
                let week_day_index = if disrupts {
                    Some(calculate_disrupted_week_day())
                } else {
                    None
                };
                return ResolvedCalendarDate::IntercalaryDay {
                    year: date.year(),
                    intercalary_index: day,
                    week_day_index,
                };
            }

            let mut remaining = day - intercalary_days;
            for month in calendar.months() {
                if remaining < month.day_count() {
                    let day_of_month = remaining + 1;
                    let week_day_index = if disrupts {
                        calculate_disrupted_week_day()
                    } else {
                        calculate_non_disrupted_regular_week_day(month.order_index(), day_of_month)
                    };
                    return ResolvedCalendarDate::RegularDay {
                        year: date.year(),
                        month_order_index: month.order_index(),
                        day_of_month,
                        week_day_index,
                    };
                }
                remaining -= month.day_count();
            }

            let last_month = calendar.months().last().unwrap();
            let day_of_month = last_month.day_count();
            let week_day_index = if disrupts {
                calculate_disrupted_week_day()
            } else {
                calculate_non_disrupted_regular_week_day(last_month.order_index(), day_of_month)
            };
            ResolvedCalendarDate::RegularDay {
                year: date.year(),
                month_order_index: last_month.order_index(),
                day_of_month,
                week_day_index,
            }
        }
        IntercalationPlacement::AfterLastMonth => {
            let mut remaining = day;
            for month in calendar.months() {
                if remaining < month.day_count() {
                    let day_of_month = remaining + 1;
                    let week_day_index = if disrupts {
                        calculate_disrupted_week_day()
                    } else {
                        calculate_non_disrupted_regular_week_day(month.order_index(), day_of_month)
                    };
                    return ResolvedCalendarDate::RegularDay {
                        year: date.year(),
                        month_order_index: month.order_index(),
                        day_of_month,
                        week_day_index,
                    };
                }
                remaining -= month.day_count();
            }

            let week_day_index = if disrupts {
                Some(calculate_disrupted_week_day())
            } else {
                None
            };
            ResolvedCalendarDate::IntercalaryDay {
                year: date.year(),
                intercalary_index: remaining,
                week_day_index,
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
                    let day_of_month = remaining + 1;
                    let week_day_index = if disrupts {
                        calculate_disrupted_week_day()
                    } else {
                        calculate_non_disrupted_regular_week_day(month.order_index(), day_of_month)
                    };
                    return ResolvedCalendarDate::RegularDay {
                        year: date.year(),
                        month_order_index: month.order_index(),
                        day_of_month,
                        week_day_index,
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

            let day_of_month = last_len;
            let week_day_index = if disrupts {
                calculate_disrupted_week_day()
            } else {
                calculate_non_disrupted_regular_week_day(last_month.order_index(), day_of_month)
            };
            ResolvedCalendarDate::RegularDay {
                year: date.year(),
                month_order_index: last_month.order_index(),
                day_of_month,
                week_day_index,
            }
        }
    }
}
