use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::{extract_effective_attribute_value, DegradationContext};
use arlo_domain::AttributeKey;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ArtrineAxisEvaluation {
    score: f64,
    multiplier: f64,
    current_ability: Option<i32>,
}

impl ArtrineAxisEvaluation {
    pub fn new(score: f64, multiplier: f64, current_ability: Option<i32>) -> Self {
        Self {
            score,
            multiplier,
            current_ability,
        }
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn multiplier(&self) -> f64 {
        self.multiplier
    }

    pub fn current_ability(&self) -> Option<i32> {
        self.current_ability
    }
}

pub struct ArtrineAxisEvaluator;

impl ArtrineAxisEvaluator {
    pub fn evaluate_score(
        table: &PlayerAttributeTable,
        deg_ctx: Option<&DegradationContext<'_>>,
    ) -> f64 {
        let (vision, decisions, passing, technique) = match deg_ctx {
            Some(ctx) => (
                extract_effective_attribute_value(table, AttributeKey::Vision, ctx),
                extract_effective_attribute_value(table, AttributeKey::Decisions, ctx),
                extract_effective_attribute_value(table, AttributeKey::Passing, ctx),
                extract_effective_attribute_value(table, AttributeKey::Technique, ctx),
            ),
            None => (
                table.get(AttributeKey::Vision),
                table.get(AttributeKey::Decisions),
                table.get(AttributeKey::Passing),
                table.get(AttributeKey::Technique),
            ),
        };

        vision * 0.30 + decisions * 0.30 + passing * 0.25 + technique * 0.15
    }

    pub fn calculate_multiplier(
        table: &PlayerAttributeTable,
        current_ability: Option<i32>,
        deg_ctx: Option<&DegradationContext<'_>>,
    ) -> f64 {
        let score = Self::evaluate_score(table, deg_ctx);
        let norm_score = (score - 10.0) / 10.0;
        let score_component = norm_score * 0.20;

        let ca_component = match current_ability {
            Some(ca) => {
                let norm_ca = ((ca.clamp(1, 200) as f64) - 100.0) / 100.0;
                norm_ca * 0.10
            }
            None => norm_score * 0.05,
        };

        (1.0 + score_component + ca_component).clamp(0.65, 1.40)
    }

    pub fn evaluate(
        table: &PlayerAttributeTable,
        current_ability: Option<i32>,
        deg_ctx: Option<&DegradationContext<'_>>,
    ) -> ArtrineAxisEvaluation {
        let score = Self::evaluate_score(table, deg_ctx);
        let multiplier = Self::calculate_multiplier(table, current_ability, deg_ctx);
        ArtrineAxisEvaluation::new(score, multiplier, current_ability)
    }

    pub fn apply_to_rating(rating: f64, multiplier: f64) -> f64 {
        (rating * multiplier).max(0.1)
    }
}