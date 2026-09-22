pub mod api;
pub mod state;

pub use api::*;
pub use state::*;

use arlo_events::{MatchClockInstant, MatchEvent};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatchEventCategory {
    Action,
    Possession,
    Scoring,
    Physical,
    Psychological,
    Manager,
    Officiating,
    Availability,
    KickFoul,
    Injury,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatchEventKind {
    CallToActionStarted,
    PassCompleted,
    DistributionCompleted,
    ReceptionResolved,
    ArtrineDecisionMade,
    DriveRecorded,
    DuelResolved,
    Turnover,
    OutOfBounds,
    CountdownToSizeStarted,
    DownAdvanced,
    GoalPoint,
    FieldPoint,
    FieldGoal,
    ScoringAttemptMissed,
    PhysicalStrainRecorded,
    RecoveryIntervalProcessed,
    ImpulseShiftRecorded,
    ImpulseCriticalReached,
    PossessionTimeRecorded,
    SubstitutionMade,
    TimeCallUsed,
    ChallengeResolved,
    TacticalProfileActivated,
    PlayCallSelected,
    FoulRaised,
    AddedTimeAwarded,
    PlayerAvailabilityChanged,
    KickFoulAwarded,
    KickFoulDecisionMade,
    InjuryIncidentRecorded,
}

impl MatchEventKind {
    pub const COUNT: usize = 31;

    pub fn category(&self) -> MatchEventCategory {
        match self {
            Self::CallToActionStarted
            | Self::PassCompleted
            | Self::DistributionCompleted
            | Self::ReceptionResolved
            | Self::ArtrineDecisionMade
            | Self::DriveRecorded
            | Self::DuelResolved => MatchEventCategory::Action,
            Self::Turnover
            | Self::OutOfBounds
            | Self::CountdownToSizeStarted
            | Self::DownAdvanced
            | Self::PossessionTimeRecorded => MatchEventCategory::Possession,
            Self::GoalPoint
            | Self::FieldPoint
            | Self::FieldGoal
            | Self::ScoringAttemptMissed => MatchEventCategory::Scoring,
            Self::PhysicalStrainRecorded | Self::RecoveryIntervalProcessed => {
                MatchEventCategory::Physical
            }
            Self::ImpulseShiftRecorded | Self::ImpulseCriticalReached => {
                MatchEventCategory::Psychological
            }
            Self::SubstitutionMade
            | Self::TimeCallUsed
            | Self::ChallengeResolved
            | Self::TacticalProfileActivated
            | Self::PlayCallSelected => MatchEventCategory::Manager,
            Self::FoulRaised | Self::AddedTimeAwarded => MatchEventCategory::Officiating,
            Self::PlayerAvailabilityChanged => MatchEventCategory::Availability,
            Self::KickFoulAwarded | Self::KickFoulDecisionMade => MatchEventCategory::KickFoul,
            Self::InjuryIncidentRecorded => MatchEventCategory::Injury,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::CallToActionStarted => "CallToActionStarted",
            Self::PassCompleted => "PassCompleted",
            Self::DistributionCompleted => "DistributionCompleted",
            Self::ReceptionResolved => "ReceptionResolved",
            Self::ArtrineDecisionMade => "ArtrineDecisionMade",
            Self::DriveRecorded => "DriveRecorded",
            Self::DuelResolved => "DuelResolved",
            Self::Turnover => "Turnover",
            Self::OutOfBounds => "OutOfBounds",
            Self::CountdownToSizeStarted => "CountdownToSizeStarted",
            Self::DownAdvanced => "DownAdvanced",
            Self::GoalPoint => "GoalPoint",
            Self::FieldPoint => "FieldPoint",
            Self::FieldGoal => "FieldGoal",
            Self::ScoringAttemptMissed => "ScoringAttemptMissed",
            Self::PhysicalStrainRecorded => "PhysicalStrainRecorded",
            Self::RecoveryIntervalProcessed => "RecoveryIntervalProcessed",
            Self::ImpulseShiftRecorded => "ImpulseShiftRecorded",
            Self::ImpulseCriticalReached => "ImpulseCriticalReached",
            Self::PossessionTimeRecorded => "PossessionTimeRecorded",
            Self::SubstitutionMade => "SubstitutionMade",
            Self::TimeCallUsed => "TimeCallUsed",
            Self::ChallengeResolved => "ChallengeResolved",
            Self::TacticalProfileActivated => "TacticalProfileActivated",
            Self::PlayCallSelected => "PlayCallSelected",
            Self::FoulRaised => "FoulRaised",
            Self::AddedTimeAwarded => "AddedTimeAwarded",
            Self::PlayerAvailabilityChanged => "PlayerAvailabilityChanged",
            Self::KickFoulAwarded => "KickFoulAwarded",
            Self::KickFoulDecisionMade => "KickFoulDecisionMade",
            Self::InjuryIncidentRecorded => "InjuryIncidentRecorded",
        }
    }

    pub fn from_event(event: &MatchEvent) -> Self {
        match event {
            MatchEvent::CallToActionStarted(_) => Self::CallToActionStarted,
            MatchEvent::PassCompleted(_) => Self::PassCompleted,
            MatchEvent::DistributionCompleted(_) => Self::DistributionCompleted,
            MatchEvent::ReceptionResolved(_) => Self::ReceptionResolved,
            MatchEvent::ArtrineDecisionMade(_) => Self::ArtrineDecisionMade,
            MatchEvent::DriveRecorded(_) => Self::DriveRecorded,
            MatchEvent::DuelResolved(_) => Self::DuelResolved,
            MatchEvent::Turnover(_) => Self::Turnover,
            MatchEvent::OutOfBounds(_) => Self::OutOfBounds,
            MatchEvent::CountdownToSizeStarted(_) => Self::CountdownToSizeStarted,
            MatchEvent::DownAdvanced(_) => Self::DownAdvanced,
            MatchEvent::GoalPoint(_) => Self::GoalPoint,
            MatchEvent::FieldPoint(_) => Self::FieldPoint,
            MatchEvent::FieldGoal(_) => Self::FieldGoal,
            MatchEvent::ScoringAttemptMissed(_) => Self::ScoringAttemptMissed,
            MatchEvent::PhysicalStrainRecorded(_) => Self::PhysicalStrainRecorded,
            MatchEvent::RecoveryIntervalProcessed(_) => Self::RecoveryIntervalProcessed,
            MatchEvent::ImpulseShiftRecorded(_) => Self::ImpulseShiftRecorded,
            MatchEvent::ImpulseCriticalReached(_) => Self::ImpulseCriticalReached,
            MatchEvent::PossessionTimeRecorded(_) => Self::PossessionTimeRecorded,
            MatchEvent::SubstitutionMade(_) => Self::SubstitutionMade,
            MatchEvent::TimeCallUsed(_) => Self::TimeCallUsed,
            MatchEvent::ChallengeResolved(_) => Self::ChallengeResolved,
            MatchEvent::TacticalProfileActivated(_) => Self::TacticalProfileActivated,
            MatchEvent::PlayCallSelected(_) => Self::PlayCallSelected,
            MatchEvent::FoulRaised(_) => Self::FoulRaised,
            MatchEvent::AddedTimeAwarded(_) => Self::AddedTimeAwarded,
            MatchEvent::PlayerAvailabilityChanged(_) => Self::PlayerAvailabilityChanged,
            MatchEvent::KickFoulAwarded(_) => Self::KickFoulAwarded,
            MatchEvent::KickFoulDecisionMade(_) => Self::KickFoulDecisionMade,
            MatchEvent::InjuryIncidentRecorded(_) => Self::InjuryIncidentRecorded,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationPlayerSnapshot {
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
    pub goalpoint_assists: u32,
    pub fouls_committed: u32,
    pub fouls_drawn: u32,
    pub punishment_yardage_loss_mirim: f64,
    pub punishment_loss_of_down_count: u32,
    pub punishment_loss_of_drive_count: u32,
    pub punishment_time_penalty_seconds: f64,
    pub punishment_expulsion_count: u32,
    pub punishment_invalidated_play_count: u32,
    pub total_suspended_seconds: f64,
    pub is_expelled: bool,
    pub kick_foul_takes: u32,
    pub total_injuries: u32,
    pub contact_injuries: u32,
    pub non_contact_injuries: u32,
    pub grade_1_injuries: u32,
    pub grade_2_injuries: u32,
    pub grade_3_injuries: u32,
    pub artrine_decisions_total: u32,
    pub artrine_decisions_successful: u32,
    pub artrine_decisions_failed: u32,
    pub artrine_success_rate: f64,
    pub artrine_mirins_advanced: f64,
    pub artrine_points_generated: u32,
    pub artrine_goal_points_generated: u32,
    pub artrine_field_points_generated: u32,
    pub artrine_field_goals_generated: u32,
    pub end_energy_level: f64,
    pub peak_anaerobic_depletion: f64,
    pub total_distance_covered: f64,
    pub intra_match_recovery_amount: f64,
    pub impulse_baseline: f64,
    pub impulse_current: u8,
    pub impulse_min: u8,
    pub impulse_max: u8,
    pub impulse_average: f64,
    pub impulse_shifts_total: u32,
    pub impulse_time_below_baseline_seconds: f64,
    pub impulse_runs_count: u32,
    pub impulse_longest_run_seconds: f64,
    pub impulse_peak_run_value: u8,
    pub impulse_total_run_intensity: f64,
}

impl SimulationPlayerSnapshot {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            total_touches: 0,
            passes_attempted: 0,
            passes_received: 0,
            recoveries: 0,
            turnovers_conceded: 0,
            total_drives: 0,
            central_drives: 0,
            left_lateral_drives: 0,
            right_lateral_drives: 0,
            lateral_drives: 0,
            max_drives_in_series: 0,
            total_duels: 0,
            total_duel_wins: 0,
            total_duel_losses: 0,
            duel_win_rate: 0.0,
            attacker_duels: 0,
            attacker_duel_wins: 0,
            attacker_duel_losses: 0,
            attacker_duel_win_rate: 0.0,
            defender_duels: 0,
            defender_duel_wins: 0,
            defender_duel_losses: 0,
            defender_duel_win_rate: 0.0,
            targets: 0,
            receptions: 0,
            drops: 0,
            catch_rate: 0.0,
            drop_rate: 0.0,
            receiving_mirins: 0.0,
            run_after_catch_mirins: 0.0,
            longest_reception_mirim: 0.0,
            average_mirins_per_reception: 0.0,
            scoring_attempts: 0,
            scoring_conversions: 0,
            scoring_misses: 0,
            scoring_conversion_rate: 0.0,
            goal_points_scored: 0,
            field_points_scored: 0,
            field_goals_scored: 0,
            total_points_scored: 0,
            goalpoint_assists: 0,
            fouls_committed: 0,
            fouls_drawn: 0,
            punishment_yardage_loss_mirim: 0.0,
            punishment_loss_of_down_count: 0,
            punishment_loss_of_drive_count: 0,
            punishment_time_penalty_seconds: 0.0,
            punishment_expulsion_count: 0,
            punishment_invalidated_play_count: 0,
            total_suspended_seconds: 0.0,
            is_expelled: false,
            kick_foul_takes: 0,
            total_injuries: 0,
            contact_injuries: 0,
            non_contact_injuries: 0,
            grade_1_injuries: 0,
            grade_2_injuries: 0,
            grade_3_injuries: 0,
            artrine_decisions_total: 0,
            artrine_decisions_successful: 0,
            artrine_decisions_failed: 0,
            artrine_success_rate: 0.0,
            artrine_mirins_advanced: 0.0,
            artrine_points_generated: 0,
            artrine_goal_points_generated: 0,
            artrine_field_points_generated: 0,
            artrine_field_goals_generated: 0,
            end_energy_level: 1.0,
            peak_anaerobic_depletion: 0.0,
            total_distance_covered: 0.0,
            intra_match_recovery_amount: 0.0,
            impulse_baseline: 50.0,
            impulse_current: 50,
            impulse_min: 50,
            impulse_max: 50,
            impulse_average: 50.0,
            impulse_shifts_total: 0,
            impulse_time_below_baseline_seconds: 0.0,
            impulse_runs_count: 0,
            impulse_longest_run_seconds: 0.0,
            impulse_peak_run_value: 50,
            impulse_total_run_intensity: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationTeamSnapshot {
    pub team_id: Uuid,
    pub total_possession_seconds: f64,
}

impl SimulationTeamSnapshot {
    pub fn new(team_id: Uuid, total_possession_seconds: f64) -> Self {
        Self {
            team_id,
            total_possession_seconds,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulationPeriodicSnapshot {
    pub sequence_number: u64,
    pub clock: MatchClockInstant,
    pub player_snapshots: Vec<SimulationPlayerSnapshot>,
    pub team_snapshots: Vec<SimulationTeamSnapshot>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RefereeSnapshotData {
    pub calls_made: u32,
    pub calls_correct: u32,
    pub calls_incorrect: u32,
    pub peace_referee_interventions: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ManagerSnapshotData {
    pub team_id: Uuid,
    pub substitutions_made: u32,
    pub time_calls_used: u32,
    pub challenges_won: u32,
    pub challenges_lost: u32,
    pub tactical_profile_switches: u32,
}