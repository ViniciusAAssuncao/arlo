use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchLineupUsageRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub tactical_lineup_id: String,
    pub formation_id: String,
    pub team_tactical_profile_id: Option<String>,
    pub manager_id: String,
}

impl MatchLineupUsageRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        tactical_lineup_id: Uuid,
        formation_id: Uuid,
        team_tactical_profile_id: Option<Uuid>,
        manager_id: Uuid,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            tactical_lineup_id: tactical_lineup_id.to_string(),
            formation_id: formation_id.to_string(),
            team_tactical_profile_id: team_tactical_profile_id.map(|id| id.to_string()),
            manager_id: manager_id.to_string(),
        }
    }
}
