use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerMatchSnapshot {
    pub player_id: Uuid,
    pub total_touches: u32,
    pub passes_attempted: u32,
    pub passes_received: u32,
    pub recoveries: u32,
    pub turnovers_conceded: u32,
    pub total_drives: u32,
    pub central_drives: u32,
    pub left_lateral_drives: u32,
    pub right_lateral_drives: u32,
    pub lateral_drives: u32,
    pub max_drives_in_series: u32,
    pub total_duels: u32,
    pub total_duel_wins: u32,
    pub total_duel_losses: u32,
    pub duel_win_rate: f64,
    pub attacker_duels: u32,
    pub attacker_duel_wins: u32,
    pub attacker_duel_losses: u32,
    pub attacker_duel_win_rate: f64,
    pub defender_duels: u32,
    pub defender_duel_wins: u32,
    pub defender_duel_losses: u32,
    pub defender_duel_win_rate: f64,
    pub targets: u32,
    pub receptions: u32,
    pub drops: u32,
    pub catch_rate: f64,
    pub drop_rate: f64,
    pub receiving_mirins: f64,
    pub run_after_catch_mirins: f64,
    pub longest_reception_mirim: f64,
    pub average_mirins_per_reception: f64,
    pub scoring_attempts: u32,
    pub scoring_conversions: u32,
    pub scoring_misses: u32,
    pub scoring_conversion_rate: f64,
    pub goal_points_scored: u32,
    pub field_points_scored: u32,
    pub field_goals_scored: u32,
    pub total_points_scored: u32,
    pub artrine_decisions_total: u32,
    pub artrine_decisions_successful: u32,
    pub artrine_decisions_failed: u32,
    pub artrine_success_rate: f64,
    pub artrine_mirins_advanced: f64,
    pub artrine_points_generated: u32,
    pub artrine_goal_points_generated: u32,
    pub artrine_field_points_generated: u32,
    pub artrine_field_goals_generated: u32,
}

impl PlayerMatchSnapshot {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            ..Default::default()
        }
    }
}