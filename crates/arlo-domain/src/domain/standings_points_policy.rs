use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::rule::{Rule, RuleCategory};
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StandingsPointsPolicy {
    points_for_win: i32,
    points_for_draw: i32,
    points_for_loss: i32,
}

impl StandingsPointsPolicy {
    pub fn new(points_for_win: i32, points_for_draw: i32, points_for_loss: i32) -> Self {
        Self {
            points_for_win,
            points_for_draw,
            points_for_loss,
        }
    }

    pub fn default_policy() -> Self {
        Self {
            points_for_win: 3,
            points_for_draw: 1,
            points_for_loss: 0,
        }
    }

    pub fn from_rules(rules: &[Rule]) -> DomainResult<Self> {
        let mut policy = Self::default_policy();

        for rule in rules {
            if rule.category() == RuleCategory::Scoring {
                match rule.rule_key() {
                    "points_for_win" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<i32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "points_for_win".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        policy.points_for_win = parsed;
                    }
                    "points_for_draw" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<i32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "points_for_draw".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        policy.points_for_draw = parsed;
                    }
                    "points_for_loss" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<i32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "points_for_loss".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        policy.points_for_loss = parsed;
                    }
                    _ => {}
                }
            }
        }

        Ok(policy)
    }

    pub fn points_for_win(&self) -> i32 {
        self.points_for_win
    }

    pub fn points_for_draw(&self) -> i32 {
        self.points_for_draw
    }

    pub fn points_for_loss(&self) -> i32 {
        self.points_for_loss
    }
}

impl Default for StandingsPointsPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}
