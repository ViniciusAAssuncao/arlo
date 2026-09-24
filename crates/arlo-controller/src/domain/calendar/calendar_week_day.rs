use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarWeekDayDefinition {
    order_index: u32,
    name: String,
}

impl CalendarWeekDayDefinition {
    pub fn new(order_index: u32, name: impl Into<String>) -> Self {
        Self {
            order_index,
            name: name.into(),
        }
    }

    pub fn order_index(&self) -> u32 {
        self.order_index
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
