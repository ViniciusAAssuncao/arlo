use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarMonthDayDto {
    pub day_of_month: u32,
    pub week_day_order_index: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalendarMonthViewDto {
    pub month_name: String,
    pub year: i64,
    pub week_day_names: Vec<String>,
    pub days: Vec<CalendarMonthDayDto>,
}
