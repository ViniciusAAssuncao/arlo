use arlo_domain::PitchZone;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PasserContactResolved {
    passer_id: Uuid,
    defender_id: Uuid,
    late: bool,
    rough: bool,
    violent: bool,
}

impl PasserContactResolved {
    pub fn new(passer_id: Uuid, defender_id: Uuid, late: bool, rough: bool, violent: bool) -> Self {
        Self { passer_id, defender_id, late, rough, violent }
    }

    pub fn passer_id(&self) -> Uuid { self.passer_id }
    pub fn defender_id(&self) -> Uuid { self.defender_id }
    pub fn late(&self) -> bool { self.late }
    pub fn rough(&self) -> bool { self.rough }
    pub fn violent(&self) -> bool { self.violent }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoalguardRecoveryResolved {
    goalguard_id: Uuid,
    team_id: Uuid,
    position_mirim: f64,
    zone: PitchZone,
    used_hands: bool,
}

impl GoalguardRecoveryResolved {
    pub fn new(goalguard_id: Uuid, team_id: Uuid, position_mirim: f64, zone: PitchZone, used_hands: bool) -> Self {
        Self { goalguard_id, team_id, position_mirim, zone, used_hands }
    }

    pub fn goalguard_id(&self) -> Uuid { self.goalguard_id }
    pub fn team_id(&self) -> Uuid { self.team_id }
    pub fn position_mirim(&self) -> f64 { self.position_mirim }
    pub fn zone(&self) -> PitchZone { self.zone }
    pub fn used_hands(&self) -> bool { self.used_hands }
}
