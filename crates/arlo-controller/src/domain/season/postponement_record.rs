use crate::domain::calendar::CalendarDate;
use crate::domain::season::postponement_reason::PostponementReason;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PostponementRecord {
    id: Uuid,
    fixture_id: Uuid,
    original_date: CalendarDate,
    new_date: CalendarDate,
    reason: PostponementReason,
}

impl PostponementRecord {
    pub fn new(
        id: Uuid,
        fixture_id: Uuid,
        original_date: CalendarDate,
        new_date: CalendarDate,
        reason: PostponementReason,
    ) -> Self {
        Self {
            id,
            fixture_id,
            original_date,
            new_date,
            reason,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn fixture_id(&self) -> Uuid {
        self.fixture_id
    }

    pub fn original_date(&self) -> CalendarDate {
        self.original_date
    }

    pub fn new_date(&self) -> CalendarDate {
        self.new_date
    }

    pub fn reason(&self) -> PostponementReason {
        self.reason
    }
}
