use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CalendarDate {
    year: i64,
    day_of_year: u32,
}

impl CalendarDate {
    pub(crate) fn new(year: i64, day_of_year: u32) -> Self {
        Self { year, day_of_year }
    }

    pub fn year(&self) -> i64 {
        self.year
    }

    pub(crate) fn day_of_year(&self) -> u32 {
        self.day_of_year
    }
}