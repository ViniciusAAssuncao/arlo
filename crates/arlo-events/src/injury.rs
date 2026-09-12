use arlo_domain::{BodyRegion, InjuryMechanism, InjurySeverityGrade};
use arlo_math::Probability;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjuryIncidentRecorded {
    player_id: Uuid,
    team_id: Uuid,
    mechanism: InjuryMechanism,
    body_region: BodyRegion,
    severity_grade: InjurySeverityGrade,
    injury_definition_id: Uuid,
    trigger_probability: Probability,
}

impl InjuryIncidentRecorded {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        mechanism: InjuryMechanism,
        body_region: BodyRegion,
        severity_grade: InjurySeverityGrade,
        injury_definition_id: Uuid,
        trigger_probability: Probability,
    ) -> Self {
        Self {
            player_id,
            team_id,
            mechanism,
            body_region,
            severity_grade,
            injury_definition_id,
            trigger_probability,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn mechanism(&self) -> InjuryMechanism {
        self.mechanism
    }

    pub fn body_region(&self) -> BodyRegion {
        self.body_region
    }

    pub fn severity_grade(&self) -> InjurySeverityGrade {
        self.severity_grade
    }

    pub fn injury_definition_id(&self) -> Uuid {
        self.injury_definition_id
    }

    pub fn trigger_probability(&self) -> Probability {
        self.trigger_probability
    }
}
