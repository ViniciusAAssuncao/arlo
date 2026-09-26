use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KnockoutTieDto {
    pub id: String,
    pub round_index: u32,
    pub tie_index: u32,
    pub high_seed_team_id: String,
    pub high_seed_number: u32,
    pub low_seed_team_id: String,
    pub low_seed_number: u32,
    pub leg_one_fixture_id: String,
    pub leg_two_fixture_id: Option<String>,
    pub aggregate_winner_team_id: Option<String>,
}