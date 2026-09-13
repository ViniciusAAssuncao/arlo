use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResolvedCalendarDate {
    RegularDay {
        year: i64,
        month_order_index: u32,
        day_of_month: u32,
    },
    IntercalaryDay {
        year: i64,
        intercalary_index: u32,
    },
}