use crate::domain::calendar::CalendarMonthDefinition;
use crate::error::ControllerResult;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct CalendarMonthRow {
    pub id: String,
    pub calendar_system_id: String,
    pub order_index: i32,
    pub name: String,
    pub day_count: i32,
}

impl CalendarMonthRow {
    pub fn to_domain(&self) -> ControllerResult<CalendarMonthDefinition> {
        Ok(CalendarMonthDefinition::new(
            self.order_index as u32,
            &self.name,
            self.day_count as u32,
        ))
    }
}