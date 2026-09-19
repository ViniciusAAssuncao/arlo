use crate::attributes::PlayerAttributeTable;
use crate::match_decision::scoring_types::ScoringOpportunity;
use crate::physical::PhysicalState;
use crate::resolution::DuelContext;
use crate::scoring_model::margin::MarginContext;
use crate::scoring_model::{ScoringDifficultyProfile, ScoringOrigin};
use arlo_domain::{AttributeKey, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub struct ScoringAttemptRequest<'a> {
    pub finisher: &'a Player,
    pub goalguard: &'a Player,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub team_id: Uuid,
    pub artrine_id: Uuid,
    pub assister_id: Option<Uuid>,
    pub opportunity: ScoringOpportunity,
    pub drives_completed: u32,
    pub territory_advance_mirim: f64,
    pub normalized_proximity: f64,
    pub pitch_length_mirim: f64,
    pub finisher_state: PhysicalState,
    pub goalguard_state: PhysicalState,
    pub context: DuelContext,
    pub finisher_table: Option<&'a PlayerAttributeTable>,
    pub goalguard_table: Option<&'a PlayerAttributeTable>,
    pub defense_closed: bool,
    pub origin: ScoringOrigin,
    pub difficulty_profile: Option<ScoringDifficultyProfile>,
    pub margin_context: Option<MarginContext>,
}

impl<'a> ScoringAttemptRequest<'a> {
    pub fn new(
        finisher: &'a Player,
        goalguard: &'a Player,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        team_id: Uuid,
        artrine_id: Uuid,
        assister_id: Option<Uuid>,
        opportunity: ScoringOpportunity,
        drives_completed: u32,
        territory_advance_mirim: f64,
        normalized_proximity: f64,
        pitch_length_mirim: f64,
        context: &DuelContext,
    ) -> Self {
        Self {
            finisher,
            goalguard,
            attribute_keys,
            team_id,
            artrine_id,
            assister_id,
            opportunity,
            drives_completed,
            territory_advance_mirim,
            normalized_proximity,
            pitch_length_mirim,
            finisher_state: PhysicalState::initial(),
            goalguard_state: PhysicalState::initial(),
            context: *context,
            finisher_table: None,
            goalguard_table: None,
            defense_closed: false,
            origin: ScoringOrigin::OpenPlay,
            difficulty_profile: None,
            margin_context: None,
        }
    }

    pub fn with_fatigue(
        mut self,
        finisher_state: PhysicalState,
        goalguard_state: PhysicalState,
    ) -> Self {
        self.finisher_state = finisher_state;
        self.goalguard_state = goalguard_state;
        self
    }

    pub fn with_tables(
        mut self,
        finisher_table: Option<&'a PlayerAttributeTable>,
        goalguard_table: Option<&'a PlayerAttributeTable>,
    ) -> Self {
        self.finisher_table = finisher_table;
        self.goalguard_table = goalguard_table;
        self
    }

    pub fn with_defense_closed(mut self, defense_closed: bool) -> Self {
        self.defense_closed = defense_closed;
        self
    }

    pub fn with_origin(mut self, origin: ScoringOrigin) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_difficulty_profile(
        mut self,
        difficulty_profile: ScoringDifficultyProfile,
    ) -> Self {
        self.difficulty_profile = Some(difficulty_profile);
        self
    }

    pub fn with_margin(mut self, margin_context: MarginContext) -> Self {
        self.margin_context = Some(margin_context);
        self
    }

    pub fn with_optional_margin(mut self, margin_context: Option<MarginContext>) -> Self {
        self.margin_context = margin_context;
        self
    }
}