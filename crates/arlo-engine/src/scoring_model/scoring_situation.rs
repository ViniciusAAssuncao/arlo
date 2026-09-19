use crate::resolution::DuelContext;
use crate::scoring_model::margin::MarginContext;
use crate::scoring_model::scoring_origin::ScoringOrigin;
use arlo_domain::PitchZone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ScoringSituation {
    pub zone: PitchZone,
    pub normalized_proximity: f64,
    pub drives_in_series: u32,
    pub territory_advance_mirim: f64,
    pub finisher_rating: f64,
    pub goalguard_rating: f64,
    pub defense_closed: bool,
    pub origin: ScoringOrigin,
    pub margin_context: Option<MarginContext>,
    pub duel_context: Option<DuelContext>,
}

impl ScoringSituation {
    pub fn new(
        zone: PitchZone,
        normalized_proximity: f64,
        drives_in_series: u32,
        territory_advance_mirim: f64,
        finisher_rating: f64,
        goalguard_rating: f64,
        defense_closed: bool,
        origin: ScoringOrigin,
    ) -> Self {
        Self {
            zone,
            normalized_proximity,
            drives_in_series,
            territory_advance_mirim,
            finisher_rating,
            goalguard_rating,
            defense_closed,
            origin,
            margin_context: None,
            duel_context: None,
        }
    }

    pub fn with_margin(mut self, margin_context: MarginContext) -> Self {
        self.margin_context = Some(margin_context);
        self
    }

    pub fn with_duel_context(mut self, duel_context: DuelContext) -> Self {
        self.duel_context = Some(duel_context);
        self
    }

    pub fn margin_context(&self) -> Option<MarginContext> {
        self.margin_context
    }

    pub fn duel_context(&self) -> Option<DuelContext> {
        self.duel_context
    }
}
