use crate::domain::calendar::CalendarDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BlackoutWindow {
    start: CalendarDate,
    end: CalendarDate,
}

impl BlackoutWindow {
    pub fn new(start: CalendarDate, end: CalendarDate) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> CalendarDate {
        self.start
    }

    pub fn end(&self) -> CalendarDate {
        self.end
    }

    pub fn contains(&self, date: &CalendarDate) -> bool {
        *date >= self.start && *date <= self.end
    }
}
