use crate::domain::calendar::CalendarWeekDayDefinition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarWeekDayDto {
    pub order_index: u32,
    pub name: String,
}

impl CalendarWeekDayDto {
    pub fn new(order_index: u32, name: impl Into<String>) -> Self {
        Self {
            order_index,
            name: name.into(),
        }
    }
}

impl From<&CalendarWeekDayDefinition> for CalendarWeekDayDto {
    fn from(def: &CalendarWeekDayDefinition) -> Self {
        Self {
            order_index: def.order_index(),
            name: def.name().to_string(),
        }
    }
}

impl From<CalendarWeekDayDefinition> for CalendarWeekDayDto {
    fn from(def: CalendarWeekDayDefinition) -> Self {
        Self::from(&def)
    }
}