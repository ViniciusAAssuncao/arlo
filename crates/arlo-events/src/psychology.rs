use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImpulseEventKind {
    DuelWon,
    DuelLost,
    ScoreFor,
    ScoreAgainst,
    TurnoverCommitted,
    TurnoverWon,
    SeriesSuccess,
    SeriesFailure,
    MilestoneStreak,
    BigPlayCompleted,
    BigPlayAllowed,
}

impl ImpulseEventKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DuelWon => "DuelWon",
            Self::DuelLost => "DuelLost",
            Self::ScoreFor => "ScoreFor",
            Self::ScoreAgainst => "ScoreAgainst",
            Self::TurnoverCommitted => "TurnoverCommitted",
            Self::TurnoverWon => "TurnoverWon",
            Self::SeriesSuccess => "SeriesSuccess",
            Self::SeriesFailure => "SeriesFailure",
            Self::MilestoneStreak => "MilestoneStreak",
            Self::BigPlayCompleted => "BigPlayCompleted",
            Self::BigPlayAllowed => "BigPlayAllowed",
        }
    }

    pub fn is_positive(&self) -> bool {
        matches!(
            self,
            Self::DuelWon
                | Self::ScoreFor
                | Self::TurnoverWon
                | Self::SeriesSuccess
                | Self::MilestoneStreak
                | Self::BigPlayCompleted
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpulseShiftRecorded {
    player_id: Uuid,
    previous_value: u8,
    new_value: u8,
    event_kind: ImpulseEventKind,
    surprisal: f64,
}

impl ImpulseShiftRecorded {
    pub fn new(
        player_id: Uuid,
        previous_value: u8,
        new_value: u8,
        event_kind: ImpulseEventKind,
        surprisal: f64,
    ) -> Self {
        Self {
            player_id,
            previous_value,
            new_value,
            event_kind,
            surprisal,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn previous_value(&self) -> u8 {
        self.previous_value
    }

    pub fn new_value(&self) -> u8 {
        self.new_value
    }

    pub fn event_kind(&self) -> ImpulseEventKind {
        self.event_kind
    }

    pub fn surprisal(&self) -> f64 {
        self.surprisal
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImpulseCriticalReached {
    player_id: Uuid,
    value: u8,
    duration_seconds: f64,
}

impl ImpulseCriticalReached {
    pub fn new(player_id: Uuid, value: u8, duration_seconds: f64) -> Self {
        Self {
            player_id,
            value,
            duration_seconds,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn duration_seconds(&self) -> f64 {
        self.duration_seconds
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PsychologyEvent {
    ImpulseShiftRecorded(ImpulseShiftRecorded),
    ImpulseCriticalReached(ImpulseCriticalReached),
}

impl From<ImpulseShiftRecorded> for PsychologyEvent {
    fn from(ev: ImpulseShiftRecorded) -> Self {
        Self::ImpulseShiftRecorded(ev)
    }
}

impl From<ImpulseCriticalReached> for PsychologyEvent {
    fn from(ev: ImpulseCriticalReached) -> Self {
        Self::ImpulseCriticalReached(ev)
    }
}
