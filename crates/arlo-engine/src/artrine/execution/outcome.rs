use crate::injury::outcome::InjuryIncidentResolution;
use crate::match_decision::scoring::ScoringDecision;
use crate::officiating::foul::FoulResolution;
use crate::resolution::AttributedDuelOutcome;
use crate::time::DurationLedger;
use arlo_domain::ArtrineDecisionKind;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DistributionFlightInfo {
    pub receiver_id: Uuid,
    pub passer_id: Uuid,
    pub decision_kind: ArtrineDecisionKind,
    pub is_aerial: bool,
    pub distance_mirim: f64,
    pub caught: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArtrineExecutionOutcome {
    pub mirins_advanced: f64,
    pub drives_recorded: u32,
    pub turnover: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub scoring_decision: ScoringDecision,
    pub duration_ledger: DurationLedger,
    pub end_x_mirim: f64,
    pub end_y_mirim: f64,
    pub duels: Vec<AttributedDuelOutcome>,
    pub fouls: Vec<FoulResolution>,
    pub injuries: Vec<InjuryIncidentResolution>,
    pub receiver_id: Option<Uuid>,
    pub distribution_flight: Vec<DistributionFlightInfo>,
}

impl ArtrineExecutionOutcome {
    pub fn stopped(
        end_x_mirim: f64,
        end_y_mirim: f64,
        duels: Vec<AttributedDuelOutcome>,
        duration_ledger: DurationLedger,
        turnover: Option<Uuid>,
        recovering_player_id: Option<Uuid>,
    ) -> Self {
        Self {
            mirins_advanced: 0.0,
            drives_recorded: 0,
            turnover,
            recovering_player_id,
            scoring_decision: ScoringDecision::NoOpportunity,
            duration_ledger,
            end_x_mirim,
            end_y_mirim,
            duels,
            fouls: Vec::new(),
            injuries: Vec::new(),
            receiver_id: None,
            distribution_flight: Vec::new(),
        }
    }
}
