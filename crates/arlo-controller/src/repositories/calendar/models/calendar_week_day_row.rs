use crate::domain::calendar::CalendarWeekDayDefinition;
use crate::error::ControllerResult;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CalendarWeekDayRow {
    pub id: String,
    pub calendar_system_id: String,
    pub order_index: i32,
    pub name: String,
}

impl CalendarWeekDayRow {
    pub fn to_domain(&self) -> ControllerResult<CalendarWeekDayDefinition> {
        Ok(CalendarWeekDayDefinition::new(
            self.order_index as u32,
            &self.name,
        ))
    }
}
