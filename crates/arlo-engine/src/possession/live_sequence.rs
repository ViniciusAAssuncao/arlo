use arlo_domain::{ArtrineDecisionKind, PitchZone};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TouchActionType {
    InitialHandoff,
    Carry,
    ShortPass,
    LongLaunch,
    Cross,
    Reception,
    FinishingAttempt,
}

impl From<ArtrineDecisionKind> for TouchActionType {
    fn from(kind: ArtrineDecisionKind) -> Self {
        match kind {
            ArtrineDecisionKind::SelfCarry => Self::Carry,
            ArtrineDecisionKind::ShortPass => Self::ShortPass,
            ArtrineDecisionKind::LongLaunch => Self::LongLaunch,
            ArtrineDecisionKind::Cross => Self::Cross,
            ArtrineDecisionKind::SelfFinish => Self::FinishingAttempt,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TouchRecord {
    pub player_id: Uuid,
    pub action_type: TouchActionType,
    pub zone: PitchZone,
    pub timestamp_seconds: f64,
}

impl TouchRecord {
    pub fn new(
        player_id: Uuid,
        action_type: TouchActionType,
        zone: PitchZone,
        timestamp_seconds: f64,
    ) -> Self {
        Self {
            player_id,
            action_type,
            zone,
            timestamp_seconds,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn action_type(&self) -> TouchActionType {
        self.action_type
    }

    pub fn zone(&self) -> PitchZone {
        self.zone
    }

    pub fn timestamp_seconds(&self) -> f64 {
        self.timestamp_seconds
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct LiveSequenceTracker {
    records: Vec<TouchRecord>,
}

impl LiveSequenceTracker {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn record_touch(
        &mut self,
        player_id: Uuid,
        action_type: TouchActionType,
        zone: PitchZone,
        timestamp_seconds: f64,
    ) {
        self.records.push(TouchRecord::new(
            player_id,
            action_type,
            zone,
            timestamp_seconds,
        ));
    }

    pub fn add_record(&mut self, record: TouchRecord) {
        self.records.push(record);
    }

    pub fn records(&self) -> &[TouchRecord] {
        &self.records
    }

    pub fn clear(&mut self) {
        self.records.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    pub fn last_touch(&self) -> Option<&TouchRecord> {
        self.records.last()
    }

    pub fn last_touch_player(&self) -> Option<Uuid> {
        self.records.last().map(|r| r.player_id)
    }

    pub fn primary_assister(&self, scorer_id: Uuid) -> Option<Uuid> {
        for record in self.records.iter().rev() {
            if record.player_id != scorer_id
                && matches!(
                    record.action_type,
                    TouchActionType::ShortPass
                        | TouchActionType::LongLaunch
                        | TouchActionType::Cross
                        | TouchActionType::InitialHandoff
                )
            {
                return Some(record.player_id);
            }
        }
        None
    }

    pub fn secondary_assister(&self, scorer_id: Uuid) -> Option<Uuid> {
        let mut found_primary = false;
        let mut primary_id = None;
        for record in self.records.iter().rev() {
            if record.player_id != scorer_id
                && matches!(
                    record.action_type,
                    TouchActionType::ShortPass
                        | TouchActionType::LongLaunch
                        | TouchActionType::Cross
                        | TouchActionType::InitialHandoff
                )
            {
                if !found_primary {
                    found_primary = true;
                    primary_id = Some(record.player_id);
                } else if Some(record.player_id) != primary_id {
                    return Some(record.player_id);
                }
            }
        }
        None
    }

    pub fn assist_chain(&self, scorer_id: Uuid) -> (Option<Uuid>, Option<Uuid>) {
        (
            self.primary_assister(scorer_id),
            self.secondary_assister(scorer_id),
        )
    }
}
