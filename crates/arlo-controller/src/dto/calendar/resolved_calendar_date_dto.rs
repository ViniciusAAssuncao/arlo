use crate::domain::calendar::{CalendarSystem, ResolvedCalendarDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum ResolvedCalendarDateDto {
    RegularDay {
        year: i64,
        month_order_index: u32,
        month_name: String,
        day_of_month: u32,
    },
    IntercalaryDay {
        year: i64,
        intercalary_index: u32,
    },
}

impl ResolvedCalendarDateDto {
    pub fn from_domain(resolved: &ResolvedCalendarDate, calendar: &CalendarSystem) -> Self {
        match resolved {
            ResolvedCalendarDate::RegularDay {
                year,
                month_order_index,
                day_of_month,
            } => {
                let month_name = calendar
                    .months()
                    .iter()
                    .find(|m| m.order_index() == *month_order_index)
                    .map(|m| m.name().to_string())
                    .unwrap_or_default();

                Self::RegularDay {
                    year: *year,
                    month_order_index: *month_order_index,
                    month_name,
                    day_of_month: *day_of_month,
                }
            }
            ResolvedCalendarDate::IntercalaryDay {
                year,
                intercalary_index,
            } => Self::IntercalaryDay {
                year: *year,
                intercalary_index: *intercalary_index,
            },
        }
    }
}
