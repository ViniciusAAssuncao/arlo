use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::extract_effective_attribute_value;
use crate::physical::PhysicalState;
use arlo_domain::sport_constants::{ATTRIBUTE_MAX, ATTRIBUTE_MIN, TACTICAL_STYLE_LOGIT_SCALE};
use arlo_domain::{AttributeKey, Player};
use arlo_math::stats::contrast::bradley_terry;
use arlo_math::Probability;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FalseArtrineEvaluation {
    bluff_rating: f64,
    deception_probability: Probability,
    defensive_hesitation_logit_offset: f64,
    space_creation_bonus: f64,
}

impl FalseArtrineEvaluation {
    pub fn new(
        bluff_rating: f64,
        deception_probability: Probability,
        defensive_hesitation_logit_offset: f64,
        space_creation_bonus: f64,
    ) -> Self {
        Self {
            bluff_rating,
            deception_probability,
            defensive_hesitation_logit_offset,
            space_creation_bonus,
        }
    }

    pub fn bluff_rating(&self) -> f64 {
        self.bluff_rating
    }

    pub fn deception_probability(&self) -> Probability {
        self.deception_probability
    }

    pub fn defensive_hesitation_logit_offset(&self) -> f64 {
        self.defensive_hesitation_logit_offset
    }

    pub fn space_creation_bonus(&self) -> f64 {
        self.space_creation_bonus
    }
}

pub fn calculate_false_artrine_bluff_rating(
    table: &PlayerAttributeTable,
    physical_state: &PhysicalState,
) -> f64 {
    let bluff =
        extract_effective_attribute_value(table, AttributeKey::FalseArtrineBluff, physical_state);
    let technique =
        extract_effective_attribute_value(table, AttributeKey::Technique, physical_state);
    let flair = extract_effective_attribute_value(table, AttributeKey::Flair, physical_state);
    let composure =
        extract_effective_attribute_value(table, AttributeKey::Composure, physical_state);

    let raw = bluff * 0.45 + technique * 0.25 + flair * 0.20 + composure * 0.10;
    raw.clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX)
}

pub fn calculate_bluff_deception_probability(
    bluff_rating: f64,
    defender_table: &PlayerAttributeTable,
    defender_physical_state: &PhysicalState,
) -> Probability {
    let anticipation = extract_effective_attribute_value(
        defender_table,
        AttributeKey::Anticipation,
        defender_physical_state,
    );
    let decisions = extract_effective_attribute_value(
        defender_table,
        AttributeKey::Decisions,
        defender_physical_state,
    );
    let defender_awareness = anticipation * 0.60 + decisions * 0.40;

    bradley_terry(bluff_rating, defender_awareness, 0.25)
}

pub fn calculate_false_artrine_hesitation_logit_offset(
    bluff_rating: f64,
    deception_prob: Probability,
) -> f64 {
    let norm_bluff = (bluff_rating / ATTRIBUTE_MAX).clamp(0.0, 1.0);
    norm_bluff * deception_prob.value() * TACTICAL_STYLE_LOGIT_SCALE * 0.75
}

pub fn calculate_false_artrine_space_creation_bonus(
    bluff_rating: f64,
    deception_prob: Probability,
) -> f64 {
    let norm_bluff = (bluff_rating / ATTRIBUTE_MAX).clamp(0.0, 1.0);
    norm_bluff * deception_prob.value() * 0.35
}

pub fn evaluate_false_artrine(
    false_artrine: &Player,
    table: &PlayerAttributeTable,
    physical_state: &PhysicalState,
    lead_defender_table: &PlayerAttributeTable,
    lead_defender_physical_state: &PhysicalState,
) -> FalseArtrineEvaluation {
    let _ = false_artrine;
    let bluff_rating = calculate_false_artrine_bluff_rating(table, physical_state);
    let deception_prob = calculate_bluff_deception_probability(
        bluff_rating,
        lead_defender_table,
        lead_defender_physical_state,
    );
    let logit_offset =
        calculate_false_artrine_hesitation_logit_offset(bluff_rating, deception_prob);
    let space_bonus = calculate_false_artrine_space_creation_bonus(bluff_rating, deception_prob);

    FalseArtrineEvaluation::new(bluff_rating, deception_prob, logit_offset, space_bonus)
}

pub fn apply_false_artrine_misdirection(
    base_misdirection_offset: f64,
    bluff_rating: f64,
) -> f64 {
    let norm_bluff = (bluff_rating / ATTRIBUTE_MAX).clamp(0.0, 1.0);
    base_misdirection_offset - (norm_bluff * 0.15)
}