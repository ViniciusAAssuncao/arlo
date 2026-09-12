use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchSquadSelectionRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub player_id: String,
    pub was_starter: bool,
    pub formation_slot_index: Option<i32>,
    pub slot_role: Option<String>,
    pub was_used: bool,
    pub final_availability_status: String,
    pub final_suspended_remaining_seconds: Option<f64>,
}

impl MatchSquadSelectionRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        player_id: Uuid,
        was_starter: bool,
        formation_slot_index: Option<i32>,
        slot_role: Option<String>,
        was_used: bool,
        final_availability_status: impl Into<String>,
        final_suspended_remaining_seconds: Option<f64>,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            player_id: player_id.to_string(),
            was_starter,
            formation_slot_index,
            slot_role,
            was_used,
            final_availability_status: final_availability_status.into(),
            final_suspended_remaining_seconds,
        }
    }
}
