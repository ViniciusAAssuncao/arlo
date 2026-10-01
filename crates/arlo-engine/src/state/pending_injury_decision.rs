use arlo_domain::InjurySeverityGrade;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PendingInjuryDecision {
    team_id: Uuid,
    player_id: Uuid,
    injury_definition_id: Uuid,
    severity_grade: InjurySeverityGrade,
}

impl PendingInjuryDecision {
    pub(crate) fn new(
        team_id: Uuid,
        player_id: Uuid,
        injury_definition_id: Uuid,
        severity_grade: InjurySeverityGrade,
    ) -> Self {
        Self { team_id, player_id, injury_definition_id, severity_grade }
    }

    pub fn team_id(self) -> Uuid { self.team_id }
    pub fn player_id(self) -> Uuid { self.player_id }
    pub fn injury_definition_id(self) -> Uuid { self.injury_definition_id }
    pub fn severity_grade(self) -> InjurySeverityGrade { self.severity_grade }
}
