use crate::domain::calendar::{CalendarDaySocialRole, CalendarWeekDayDefinition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarWeekDayDto {
    pub order_index: u32,
    pub name: String,
    pub social_role: Option<CalendarDaySocialRole>,
}

impl CalendarWeekDayDto {
    pub fn new(order_index: u32, name: impl Into<String>) -> Self {
        Self {
            order_index,
            name: name.into(),
            social_role: None,
        }
    }
}

impl From<&CalendarWeekDayDefinition> for CalendarWeekDayDto {
    fn from(def: &CalendarWeekDayDefinition) -> Self {
        Self {
            order_index: def.order_index(),
            name: def.name().to_string(),
            social_role: def.social_role(),
        }
    }
}

impl From<CalendarWeekDayDefinition> for CalendarWeekDayDto {
    fn from(def: CalendarWeekDayDefinition) -> Self {
        Self::from(&def)
    }
}
