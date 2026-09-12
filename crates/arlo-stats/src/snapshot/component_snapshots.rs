use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerArtrineDecisionSnapshot {
    pub player_id: Uuid,
    pub total_decisions: u32,
    pub total_successful_decisions: u32,
    pub total_failed_decisions: u32,
    pub total_mirins_advanced: f64,
    pub total_points_generated: u32,
    pub goal_points_generated: u32,
    pub field_points_generated: u32,
    pub field_goals_generated: u32,
    pub success_rate: f64,
    pub average_mirins_per_decision: f64,
    pub average_points_per_decision: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerDriveSnapshot {
    pub player_id: Uuid,
    pub total_drives: u32,
    pub central_drives: u32,
    pub left_lateral_drives: u32,
    pub right_lateral_drives: u32,
    pub lateral_drives: u32,
    pub max_drives_in_series: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerDuelSnapshot {
    pub player_id: Uuid,
    pub total_duels: u32,
    pub total_wins: u32,
    pub total_losses: u32,
    pub win_rate: f64,
    pub attacker_duels: u32,
    pub attacker_wins: u32,
    pub attacker_losses: u32,
    pub attacker_win_rate: f64,
    pub defender_duels: u32,
    pub defender_wins: u32,
    pub defender_losses: u32,
    pub defender_win_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerReceivingSnapshot {
    pub player_id: Uuid,
    pub targets: u32,
    pub receptions: u32,
    pub drops: u32,
    pub catch_rate: f64,
    pub drop_rate: f64,
    pub run_after_catch_mirins: f64,
    pub longest_reception_mirim: f64,
    pub receiving_mirins: f64,
    pub average_mirins_per_reception: f64,
    pub average_rac_per_reception: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerScoringAttemptSnapshot {
    pub player_id: Uuid,
    pub attempts: u32,
    pub converted: u32,
    pub missed: u32,
    pub goal_points_scored: u32,
    pub field_points_scored: u32,
    pub field_goals_scored: u32,
    pub total_points_scored: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerTouchSnapshot {
    pub player_id: Uuid,
    pub passes_attempted: u32,
    pub passes_received: u32,
    pub drives_recorded: u32,
    pub recoveries: u32,
    pub scoring_attempts: u32,
    pub total_touches: u32,
    pub turnovers_conceded: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamPossessionSnapshot {
    pub team_id: Uuid,
    pub total_possession_seconds: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerAssistSnapshot {
    pub player_id: Uuid,
    pub goalpoint_assists: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerFoulSnapshot {
    pub player_id: Uuid,
    pub fouls_committed: u32,
    pub fouls_drawn: u32,
    pub correct_calls_committed: u32,
    pub incorrect_calls_committed: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RefereeMatchSnapshot {
    pub calls_made: u32,
    pub calls_correct: u32,
    pub calls_incorrect: u32,
    pub peace_referee_interventions: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerPunishmentSnapshot {
    pub player_id: Uuid,
    pub yardage_loss_count: u32,
    pub loss_of_down_count: u32,
    pub loss_of_drive_count: u32,
    pub time_penalty_count: u32,
    pub expulsion_count: u32,
    pub invalidate_play_count: u32,
    pub total_yardage_loss_mirim: f64,
    pub total_loss_of_down_count: u32,
    pub total_time_penalty_seconds: f64,
    pub total_loss_of_drive_count: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerAvailabilitySnapshot {
    pub player_id: Uuid,
    pub total_suspended_seconds: f64,
    pub expulsion_count: u32,
    pub is_currently_expelled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlayerKickFoulSnapshot {
    pub player_id: Uuid,
    pub kick_foul_takes: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PlayerInjurySnapshot {
    pub player_id: Uuid,
    pub total_injuries: u32,
    pub contact_injuries: u32,
    pub non_contact_injuries: u32,
    pub grade_1_injuries: u32,
    pub grade_2_injuries: u32,
    pub grade_3_injuries: u32,
}