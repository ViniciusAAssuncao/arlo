use crate::domain::calendar::{CalendarDaySocialRole, CalendarWeekDayDefinition};
use crate::error::{ControllerError, ControllerResult};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CalendarWeekDayRow {
    pub id: String,
    pub calendar_system_id: String,
    pub order_index: i32,
    pub name: String,
    pub social_role: Option<String>,
}

impl CalendarWeekDayRow {
    pub fn to_domain(&self) -> ControllerResult<CalendarWeekDayDefinition> {
        let social_role = match self.social_role.as_deref() {
            Some("workday") => Some(CalendarDaySocialRole::Workday),
            Some("rest_day") => Some(CalendarDaySocialRole::RestDay),
            None => None,
            Some(value) => {
                return Err(ControllerError::InvalidData(format!(
                    "invalid calendar day social role: {value}"
                )))
            }
        };
        Ok(
            CalendarWeekDayDefinition::new(self.order_index as u32, &self.name)
                .with_social_role(social_role),
        )
    }
}
