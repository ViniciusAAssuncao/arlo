use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchTeamScoreRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub is_home: bool,
    pub goal_points: i32,
    pub field_goals: i32,
    pub field_points: i32,
    pub total_points: i32,
}

impl MatchTeamScoreRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        is_home: bool,
        goal_points: u32,
        field_goals: u32,
        field_points: u32,
        total_points: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            is_home,
            goal_points: goal_points as i32,
            field_goals: field_goals as i32,
            field_points: field_points as i32,
            total_points: total_points as i32,
        }
    }
}
