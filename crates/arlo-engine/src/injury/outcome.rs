use arlo_domain::{BodyRegion, InjuryMechanism, InjurySeverityGrade};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InjuryIncidentResolution {
    pub injured_player_id: Uuid,
    pub team_id: Uuid,
    pub mechanism: InjuryMechanism,
    pub body_region: BodyRegion,
    pub severity_grade: InjurySeverityGrade,
    pub injury_definition_id: Option<Uuid>,
    pub trigger_probability: f64,
}

impl InjuryIncidentResolution {
    pub fn new(
        injured_player_id: Uuid,
        team_id: Uuid,
        mechanism: InjuryMechanism,
        body_region: BodyRegion,
        severity_grade: InjurySeverityGrade,
        injury_definition_id: Option<Uuid>,
        trigger_probability: f64,
    ) -> Self {
        Self {
            injured_player_id,
            team_id,
            mechanism,
            body_region,
            severity_grade,
            injury_definition_id,
            trigger_probability,
        }
    }

    pub fn injured_player_id(&self) -> Uuid {
        self.injured_player_id
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

    pub fn injury_definition_id(&self) -> Option<Uuid> {
        self.injury_definition_id
    }

    pub fn trigger_probability(&self) -> f64 {
        self.trigger_probability
    }
}
