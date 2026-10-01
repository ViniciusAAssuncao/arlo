use crate::domain::calendar::{CalendarSystem, ResolvedCalendarDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ResolvedCalendarDateDto {
    RegularDay {
        year: i64,
        month_order_index: u32,
        month_name: String,
        day_of_month: u32,
        week_day_name: String,
        week_day_order_index: u32,
    },
    IntercalaryDay {
        year: i64,
        intercalary_index: u32,
        week_day_name: Option<String>,
        week_day_order_index: Option<u32>,
    },
}

impl ResolvedCalendarDateDto {
    pub fn from_domain(resolved: &ResolvedCalendarDate, calendar: &CalendarSystem) -> Self {
        match resolved {
            ResolvedCalendarDate::RegularDay {
                year,
                month_order_index,
                day_of_month,
                week_day_index,
            } => {
                let month_name = calendar
                    .months()
                    .iter()
                    .find(|m| m.order_index() == *month_order_index)
                    .map(|m| m.name().to_string())
                    .unwrap_or_default();

                let week_day_name = calendar
                    .week_days()
                    .iter()
                    .find(|w| w.order_index() == *week_day_index)
                    .map(|w| w.name().to_string())
                    .unwrap_or_default();

                Self::RegularDay {
                    year: *year,
                    month_order_index: *month_order_index,
                    month_name,
                    day_of_month: *day_of_month,
                    week_day_name,
                    week_day_order_index: *week_day_index,
                }
            }
            ResolvedCalendarDate::IntercalaryDay {
                year,
                intercalary_index,
                week_day_index,
            } => {
                let week_day_name = week_day_index.and_then(|idx| {
                    calendar
                        .week_days()
                        .iter()
                        .find(|w| w.order_index() == idx)
                        .map(|w| w.name().to_string())
                });

                Self::IntercalaryDay {
                    year: *year,
                    intercalary_index: *intercalary_index,
                    week_day_name,
                    week_day_order_index: *week_day_index,
                }
            }
        }
    }
}
