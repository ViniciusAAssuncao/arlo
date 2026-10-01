use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct KnockoutTieRow {
    pub id: String,
    pub season_stage_id: String,
    pub round_index: i32,
    pub tie_index: i32,
    pub high_seed_team_id: String,
    pub high_seed_number: i32,
    pub low_seed_team_id: String,
    pub low_seed_number: i32,
    pub leg_one_fixture_id: String,
    pub leg_two_fixture_id: Option<String>,
    pub aggregate_winner_team_id: Option<String>,
}

impl KnockoutTieRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        season_stage_id: Uuid,
        round_index: u32,
        tie_index: u32,
        high_seed_team_id: Uuid,
        high_seed_number: u32,
        low_seed_team_id: Uuid,
        low_seed_number: u32,
        leg_one_fixture_id: Uuid,
        leg_two_fixture_id: Option<Uuid>,
        aggregate_winner_team_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: id.to_string(),
            season_stage_id: season_stage_id.to_string(),
            round_index: round_index as i32,
            tie_index: tie_index as i32,
            high_seed_team_id: high_seed_team_id.to_string(),
            high_seed_number: high_seed_number as i32,
            low_seed_team_id: low_seed_team_id.to_string(),
            low_seed_number: low_seed_number as i32,
            leg_one_fixture_id: leg_one_fixture_id.to_string(),
            leg_two_fixture_id: leg_two_fixture_id.map(|id| id.to_string()),
            aggregate_winner_team_id: aggregate_winner_team_id.map(|id| id.to_string()),
        }
    }
}
