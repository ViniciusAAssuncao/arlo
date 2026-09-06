use crate::match_decision::scoring::ScoringDecision;
use crate::possession::PlayOutcome as PossessionPlayOutcome;
use crate::resolution::AttributedDuelOutcome;
use arlo_math::units::Position;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DetailedPlayOutcome {
    pub offense_team_id: Uuid,
    pub defense_team_id: Uuid,
    pub passer_id: Uuid,
    pub artrine_id: Uuid,
    pub down_number: u32,
    pub scrimmage_x_mirim: f64,
    pub pass_completed: bool,
    pub pass_is_aerial: bool,
    pub reception_point: Position,
    pub drives_recorded: u32,
    pub mirins_advanced: f64,
    pub duels: Vec<AttributedDuelOutcome>,
    pub turnover: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub out_of_bounds: bool,
    pub arbitral_stoppage: bool,
    pub last_valid_possession_point: Position,
    pub possession_control_seconds: Option<f64>,
    pub scoring_decision: ScoringDecision,
}

impl DetailedPlayOutcome {
    pub fn to_possession_outcome(&self) -> PossessionPlayOutcome {
        PossessionPlayOutcome {
            turnover: self.turnover,
            out_of_bounds: self.out_of_bounds,
            arbitral_stoppage: self.arbitral_stoppage,
            mirins_advanced: self.mirins_advanced,
            last_valid_possession_point: self.last_valid_possession_point,
            possession_control_seconds: self.possession_control_seconds,
            score_occurred: self.scoring_decision.is_scored(),
            is_goal_point: matches!(self.scoring_decision, ScoringDecision::GoalPoint { .. }),
        }
    }
}
