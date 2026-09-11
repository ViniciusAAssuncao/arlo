use crate::artrine::{ArtrineExecutionOutcome, DistributionFlightInfo};
use crate::match_decision::scoring::ScoringDecision;
use crate::officiating::foul::FoulResolution;
use crate::resolution::AttributedDuelOutcome;
use crate::spatial::SpatialTrajectory;
use crate::time::DurationLedger;
use arlo_domain::ArtrineDecisionKind;
use arlo_math::units::Position as VectorPosition;
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

pub struct OpenPlayLoopState {
    pub current_carrier_id: Uuid,
    pub current_carrier_pos: VectorPosition,
    pub accumulated_mirins_advanced: f64,
    pub accumulated_drives_recorded: u32,
    pub accumulated_drive_row_indices: SmallVec<[usize; 4]>,
    pub accumulated_duels: Vec<AttributedDuelOutcome>,
    pub accumulated_fouls: Vec<FoulResolution>,
    pub accumulated_duration_ledger: DurationLedger,
    pub accumulated_trajectories: HashMap<Uuid, SpatialTrajectory>,
    pub scoring_decision: ScoringDecision,
    pub turnover_team: Option<Uuid>,
    pub recovering_player: Option<Uuid>,
    pub last_receiver_id: Option<Uuid>,
    pub last_distribution_flight: Option<DistributionFlightInfo>,
    pub primary_decision_kind: ArtrineDecisionKind,
    pub ball_in_play: bool,
    pub loop_iteration: usize,
}

impl OpenPlayLoopState {
    pub fn new(artrine_id: Uuid, reception_point: VectorPosition) -> Self {
        Self {
            current_carrier_id: artrine_id,
            current_carrier_pos: reception_point,
            accumulated_mirins_advanced: 0.0,
            accumulated_drives_recorded: 0,
            accumulated_drive_row_indices: SmallVec::new(),
            accumulated_duels: Vec::new(),
            accumulated_fouls: Vec::new(),
            accumulated_duration_ledger: DurationLedger::new(),
            accumulated_trajectories: HashMap::new(),
            scoring_decision: ScoringDecision::NoOpportunity,
            turnover_team: None,
            recovering_player: None,
            last_receiver_id: None,
            last_distribution_flight: None,
            primary_decision_kind: ArtrineDecisionKind::SelfCarry,
            ball_in_play: true,
            loop_iteration: 0,
        }
    }

    pub fn into_outcome(self) -> (ArtrineDecisionKind, ArtrineExecutionOutcome) {
        let outcome = ArtrineExecutionOutcome {
            mirins_advanced: self.accumulated_mirins_advanced,
            drives_recorded: self.accumulated_drives_recorded,
            drive_row_indices: self.accumulated_drive_row_indices,
            turnover: self.turnover_team,
            recovering_player_id: self.recovering_player,
            scoring_decision: self.scoring_decision,
            duration_ledger: self.accumulated_duration_ledger,
            end_position: self.current_carrier_pos,
            duels: self.accumulated_duels,
            fouls: self.accumulated_fouls,
            receiver_id: self.last_receiver_id,
            distribution_flight: self.last_distribution_flight,
            kinematic_trajectories: self.accumulated_trajectories,
        };
        (self.primary_decision_kind, outcome)
    }
}
