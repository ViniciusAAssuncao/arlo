use crate::models::season::FixtureRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct AttendanceSnapshot {
    pub total: i32,
    pub home_supporters: i32,
    pub away_supporters: i32,
    pub unaffiliated_spectators: i32,
    pub match_appeal: f64,
    pub home_popularity: f64,
    pub away_popularity: f64,
    pub model_version: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPersistenceContext {
    pub home_tactical_lineup_id: Uuid,
    pub away_tactical_lineup_id: Uuid,
    pub home_formation_id: Uuid,
    pub away_formation_id: Uuid,
    pub home_team_tactical_profile_id: Option<Uuid>,
    pub away_team_tactical_profile_id: Option<Uuid>,
    pub venue_id: Option<Uuid>,
    pub completed_at_unix_seconds: Option<i64>,
    pub created_at_unix_seconds: Option<i64>,
    pub completed_fixture: Option<FixtureRow>,
    pub attendance: Option<AttendanceSnapshot>,
}

impl MatchPersistenceContext {
    pub fn new(
        home_tactical_lineup_id: Uuid,
        away_tactical_lineup_id: Uuid,
        home_formation_id: Uuid,
        away_formation_id: Uuid,
        home_team_tactical_profile_id: Option<Uuid>,
        away_team_tactical_profile_id: Option<Uuid>,
        venue_id: Option<Uuid>,
    ) -> Self {
        Self {
            home_tactical_lineup_id,
            away_tactical_lineup_id,
            home_formation_id,
            away_formation_id,
            home_team_tactical_profile_id,
            away_team_tactical_profile_id,
            venue_id,
            completed_at_unix_seconds: None,
            created_at_unix_seconds: None,
            completed_fixture: None,
            attendance: None,
        }
    }

    pub fn with_timestamps(
        mut self,
        completed_at_unix_seconds: i64,
        created_at_unix_seconds: i64,
    ) -> Self {
        self.completed_at_unix_seconds = Some(completed_at_unix_seconds);
        self.created_at_unix_seconds = Some(created_at_unix_seconds);
        self
    }

    pub fn with_completed_fixture(mut self, completed_fixture: FixtureRow) -> Self {
        self.completed_fixture = Some(completed_fixture);
        self
    }

    pub fn with_attendance(mut self, attendance: Option<AttendanceSnapshot>) -> Self {
        self.attendance = attendance;
        self
    }
}
