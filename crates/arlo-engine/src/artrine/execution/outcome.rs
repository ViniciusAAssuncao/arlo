use crate::match_decision::scoring::ScoringDecision;
use crate::officiating::foul::FoulResolution;
use crate::resolution::AttributedDuelOutcome;
use crate::spatial::SpatialTrajectory;
use crate::time::DurationLedger;
use arlo_domain::ArtrineDecisionKind;
use arlo_math::units::Position as VectorPosition;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionFlightInfo {
    pub receiver_id: Uuid,
    pub passer_id: Uuid,
    pub decision_kind: ArtrineDecisionKind,
    pub is_aerial: bool,
    pub reception_point: VectorPosition,
    pub distance_mirim: f64,
    pub caught: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtrineExecutionOutcome {
    pub mirins_advanced: f64,
    pub drives_recorded: u32,
    pub drive_row_indices: SmallVec<[usize; 4]>,
    pub turnover: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub scoring_decision: ScoringDecision,
    pub duration_ledger: DurationLedger,
    pub end_position: VectorPosition,
    pub duels: Vec<AttributedDuelOutcome>,
    pub fouls: Vec<FoulResolution>,
    pub receiver_id: Option<Uuid>,
    pub distribution_flight: Option<DistributionFlightInfo>,
    pub kinematic_trajectories: HashMap<Uuid, SpatialTrajectory>,
}

impl ArtrineExecutionOutcome {
    pub fn stopped(
        end_position: VectorPosition,
        duels: Vec<AttributedDuelOutcome>,
        duration_ledger: DurationLedger,
        turnover: Option<Uuid>,
        recovering_player_id: Option<Uuid>,
    ) -> Self {
        Self {
            mirins_advanced: 0.0,
            drives_recorded: 0,
            drive_row_indices: SmallVec::new(),
            turnover,
            recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            duration_ledger,
            end_position,
            duels,
            fouls: Vec::new(),
            receiver_id: None,
            distribution_flight: None,
            kinematic_trajectories: HashMap::new(),
        }
    }
}
