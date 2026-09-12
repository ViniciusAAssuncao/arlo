use arlo_events::SubstitutionReason;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchManagerSubstitutionByReasonRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub reason: String,
    pub substitutions_count: i32,
}

impl MatchManagerSubstitutionByReasonRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        reason: impl Into<String>,
        substitutions_count: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            reason: reason.into(),
            substitutions_count: substitutions_count as i32,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        reason: SubstitutionReason,
        count: u32,
    ) -> Self {
        Self::new(id, match_id, team_id, reason.as_str(), count)
    }
}