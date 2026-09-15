use crate::domain::calendar::calendar_date::CalendarDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SaveCalendarState {
    save_uuid: Uuid,
    calendar_system_id: Uuid,
    current_date: CalendarDate,
    assigned_at_unix_seconds: i64,
}

impl SaveCalendarState {
    pub fn new(
        save_uuid: Uuid,
        calendar_system_id: Uuid,
        current_date: CalendarDate,
        assigned_at_unix_seconds: i64,
    ) -> Self {
        Self {
            save_uuid,
            calendar_system_id,
            current_date,
            assigned_at_unix_seconds,
        }
    }

    pub fn save_uuid(&self) -> Uuid {
        self.save_uuid
    }

    pub fn calendar_system_id(&self) -> Uuid {
        self.calendar_system_id
    }

    pub fn current_date(&self) -> CalendarDate {
        self.current_date
    }

    pub fn assigned_at_unix_seconds(&self) -> i64 {
        self.assigned_at_unix_seconds
    }
}