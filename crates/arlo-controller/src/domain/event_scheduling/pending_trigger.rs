use crate::domain::calendar::CalendarDate;
use crate::domain::event_scheduling::trigger_kind::TriggerKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PendingTrigger {
    trigger_date: CalendarDate,
    competition_id: Uuid,
    kind: TriggerKind,
}

impl PendingTrigger {
    pub fn new(trigger_date: CalendarDate, competition_id: Uuid, kind: TriggerKind) -> Self {
        Self {
            trigger_date,
            competition_id,
            kind,
        }
    }

    pub fn trigger_date(&self) -> CalendarDate {
        self.trigger_date
    }

    pub fn competition_id(&self) -> Uuid {
        self.competition_id
    }

    pub fn kind(&self) -> TriggerKind {
        self.kind
    }
}