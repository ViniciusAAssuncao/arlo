use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClockStopReason {
    OutOfBounds,
    ArbitralStoppage,
    Foul,
    Review,
    ArtroMarking,
    CountdownToSize,
    TimeCall,
    ChallengeCall,
    PointScored,
    PeriodEnd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClockState {
    Running,
    Stopped(ClockStopReason),
}

impl ClockState {
    pub fn is_running(&self) -> bool {
        matches!(self, Self::Running)
    }

    pub fn is_stopped(&self) -> bool {
        matches!(self, Self::Stopped(_))
    }

    pub fn stop_reason(&self) -> Option<ClockStopReason> {
        match self {
            Self::Running => None,
            Self::Stopped(reason) => Some(*reason),
        }
    }
}

impl Default for ClockState {
    fn default() -> Self {
        Self::Stopped(ClockStopReason::PeriodEnd)
    }
}
