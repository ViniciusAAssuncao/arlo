use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::DuelOutcome;
use crate::time::DurationLedger;
use arlo_math::units::Position as VectorPosition;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtrineExecutionOutcome {
    pub mirins_advanced: f64,
    pub drives_recorded: u32,
    pub drive_row_indices: Vec<usize>,
    pub turnover: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub scoring_decision: ScoringDecision,
    pub duration_ledger: DurationLedger,
    pub end_position: VectorPosition,
    pub duels: Vec<DuelOutcome>,
    pub receiver_id: Option<Uuid>,
}