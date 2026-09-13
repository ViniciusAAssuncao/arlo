use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarMonthDefinition {
    order_index: u32,
    name: String,
    day_count: u32,
}

impl CalendarMonthDefinition {
    pub fn new(order_index: u32, name: impl Into<String>, day_count: u32) -> Self {
        Self {
            order_index,
            name: name.into(),
            day_count,
        }
    }

    pub fn order_index(&self) -> u32 {
        self.order_index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn day_count(&self) -> u32 {
        self.day_count
    }
}