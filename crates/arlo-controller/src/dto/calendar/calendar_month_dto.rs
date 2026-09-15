use crate::domain::calendar::CalendarMonthDefinition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarMonthDto {
    pub order_index: u32,
    pub name: String,
    pub day_count: u32,
}

impl CalendarMonthDto {
    pub fn new(order_index: u32, name: impl Into<String>, day_count: u32) -> Self {
        Self {
            order_index,
            name: name.into(),
            day_count,
        }
    }
}

impl From<&CalendarMonthDefinition> for CalendarMonthDto {
    fn from(def: &CalendarMonthDefinition) -> Self {
        Self {
            order_index: def.order_index(),
            name: def.name().to_string(),
            day_count: def.day_count(),
        }
    }
}

impl From<CalendarMonthDefinition> for CalendarMonthDto {
    fn from(def: CalendarMonthDefinition) -> Self {
        Self::from(&def)
    }
}
