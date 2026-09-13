use crate::domain::calendar::CalendarSystem;
use crate::dto::calendar::calendar_month_dto::CalendarMonthDto;
use crate::dto::calendar::intercalation_rule_dto::IntercalationRuleDto;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarSystemDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub months: Vec<CalendarMonthDto>,
    pub intercalation_rule: IntercalationRuleDto,
}

impl CalendarSystemDto {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        description: Option<String>,
        months: Vec<CalendarMonthDto>,
        intercalation_rule: IntercalationRuleDto,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            description,
            months,
            intercalation_rule,
        }
    }
}

impl From<&CalendarSystem> for CalendarSystemDto {
    fn from(system: &CalendarSystem) -> Self {
        Self {
            id: system.id(),
            name: system.name().to_string(),
            description: system.description().map(str::to_string),
            months: system.months().iter().map(CalendarMonthDto::from).collect(),
            intercalation_rule: IntercalationRuleDto::from(system.intercalation_rule()),
        }
    }
}

impl From<CalendarSystem> for CalendarSystemDto {
    fn from(system: CalendarSystem) -> Self {
        Self::from(&system)
    }
}
