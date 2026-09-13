use crate::domain::calendar::{CalendarDate, SaveCalendarState};
use crate::error::ControllerResult;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct SaveCalendarStateRow {
    pub save_uuid: String,
    pub calendar_system_id: String,
    pub current_year: i64,
    pub current_day_of_year: i32,
    pub assigned_at_unix_seconds: i64,
}

impl SaveCalendarStateRow {
    pub fn to_domain(&self) -> ControllerResult<SaveCalendarState> {
        let save_uuid = Uuid::parse_str(&self.save_uuid)?;
        let calendar_system_id = Uuid::parse_str(&self.calendar_system_id)?;
        let current_date =
            CalendarDate::new(self.current_year, self.current_day_of_year as u32);

        Ok(SaveCalendarState::new(
            save_uuid,
            calendar_system_id,
            current_date,
            self.assigned_at_unix_seconds,
        ))
    }
}