use crate::domain::calendar::calendar_month::CalendarMonthDefinition;
use crate::domain::calendar::calendar_validation::validate_calendar_system;
use crate::domain::calendar::calendar_week_day::CalendarWeekDayDefinition;
use crate::domain::calendar::intercalation_rule::IntercalationRule;
use crate::error::ControllerResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CalendarSystem {
    id: Uuid,
    name: String,
    description: Option<String>,
    months: Vec<CalendarMonthDefinition>,
    week_days: Vec<CalendarWeekDayDefinition>,
    intercalation_rule: IntercalationRule,
}

impl CalendarSystem {
    pub fn new(
        id: Uuid,
        name: impl Into<String>,
        description: Option<String>,
        months: Vec<CalendarMonthDefinition>,
        week_days: Vec<CalendarWeekDayDefinition>,
        intercalation_rule: IntercalationRule,
    ) -> ControllerResult<Self> {
        validate_calendar_system(&months, &week_days, &intercalation_rule)?;

        Ok(Self {
            id,
            name: name.into(),
            description,
            months,
            week_days,
            intercalation_rule,
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn months(&self) -> &[CalendarMonthDefinition] {
        &self.months
    }

    pub fn week_days(&self) -> &[CalendarWeekDayDefinition] {
        &self.week_days
    }

    pub fn intercalation_rule(&self) -> &IntercalationRule {
        &self.intercalation_rule
    }
}