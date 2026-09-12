use arlo_stats::PlayerInjuryStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerInjuryRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub total_injuries: i32,
    pub contact_injuries: i32,
    pub non_contact_injuries: i32,
    pub grade_1_injuries: i32,
    pub grade_2_injuries: i32,
    pub grade_3_injuries: i32,
}

impl MatchPlayerInjuryRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        total_injuries: u32,
        contact_injuries: u32,
        non_contact_injuries: u32,
        grade_1_injuries: u32,
        grade_2_injuries: u32,
        grade_3_injuries: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            total_injuries: total_injuries as i32,
            contact_injuries: contact_injuries as i32,
            non_contact_injuries: non_contact_injuries as i32,
            grade_1_injuries: grade_1_injuries as i32,
            grade_2_injuries: grade_2_injuries as i32,
            grade_3_injuries: grade_3_injuries as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerInjuryStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.total_injuries,
            stats.contact_injuries,
            stats.non_contact_injuries,
            stats.grade_1_injuries,
            stats.grade_2_injuries,
            stats.grade_3_injuries,
        )
    }
}
