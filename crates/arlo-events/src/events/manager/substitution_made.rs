use crate::envelope::MatchClockInstant;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubstitutionReason {
    Fatigue,
    Tactical,
    Disciplinary,
    Injury,
}

impl SubstitutionReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Fatigue => "Fatigue",
            Self::Tactical => "Tactical",
            Self::Disciplinary => "Disciplinary",
            Self::Injury => "Injury",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubstitutionMade {
    team_id: Uuid,
    player_out: Uuid,
    player_in: Uuid,
    match_clock: MatchClockInstant,
    reason: SubstitutionReason,
}

impl SubstitutionMade {
    pub fn new(
        team_id: Uuid,
        player_out: Uuid,
        player_in: Uuid,
        match_clock: MatchClockInstant,
        reason: SubstitutionReason,
    ) -> Self {
        Self {
            team_id,
            player_out,
            player_in,
            match_clock,
            reason,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn player_out(&self) -> Uuid {
        self.player_out
    }

    pub fn player_in(&self) -> Uuid {
        self.player_in
    }

    pub fn match_clock(&self) -> MatchClockInstant {
        self.match_clock
    }

    pub fn reason(&self) -> SubstitutionReason {
        self.reason
    }
}
