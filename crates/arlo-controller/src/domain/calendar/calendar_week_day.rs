use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CalendarDaySocialRole {
    Workday,
    RestDay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarWeekDayDefinition {
    order_index: u32,
    name: String,
    social_role: Option<CalendarDaySocialRole>,
}

impl CalendarWeekDayDefinition {
    pub fn new(order_index: u32, name: impl Into<String>) -> Self {
        Self {
            order_index,
            name: name.into(),
            social_role: None,
        }
    }

    pub fn with_social_role(mut self, social_role: Option<CalendarDaySocialRole>) -> Self {
        self.social_role = social_role;
        self
    }

    pub fn order_index(&self) -> u32 {
        self.order_index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn social_role(&self) -> Option<CalendarDaySocialRole> {
        self.social_role
    }
}
