use crate::domain::league_calendar::league_movement_rule::LeagueMovementRule;
use crate::domain::validation::validate_integer_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromotionRelegationPolicy {
    standings_stage_order_index: u32,
    promotion_rule: LeagueMovementRule,
    promotion_target_league_id: Option<Uuid>,
    relegation_rule: LeagueMovementRule,
    relegation_target_league_id: Option<Uuid>,
}

impl PromotionRelegationPolicy {
    pub fn new(
        standings_stage_order_index: u32,
        promotion_rule: LeagueMovementRule,
        promotion_target_league_id: Option<Uuid>,
        relegation_rule: LeagueMovementRule,
        relegation_target_league_id: Option<Uuid>,
    ) -> DomainResult<Self> {
        match promotion_rule {
            LeagueMovementRule::None => {}
            LeagueMovementRule::Automatic { count }
            | LeagueMovementRule::PlayoffStage { count, .. } => {
                validate_integer_range(count as i32, 1, i32::MAX, "promotion_rule.count")?;
            }
        }

        match relegation_rule {
            LeagueMovementRule::None => {}
            LeagueMovementRule::Automatic { count }
            | LeagueMovementRule::PlayoffStage { count, .. } => {
                validate_integer_range(count as i32, 1, i32::MAX, "relegation_rule.count")?;
            }
        }

        Ok(Self {
            standings_stage_order_index,
            promotion_rule,
            promotion_target_league_id,
            relegation_rule,
            relegation_target_league_id,
        })
    }

    pub fn none() -> Self {
        Self {
            standings_stage_order_index: 0,
            promotion_rule: LeagueMovementRule::None,
            promotion_target_league_id: None,
            relegation_rule: LeagueMovementRule::None,
            relegation_target_league_id: None,
        }
    }

    pub fn standings_stage_order_index(&self) -> u32 {
        self.standings_stage_order_index
    }

    pub fn promotion_rule(&self) -> LeagueMovementRule {
        self.promotion_rule
    }

    pub fn promotion_target_league_id(&self) -> Option<Uuid> {
        self.promotion_target_league_id
    }

    pub fn relegation_rule(&self) -> LeagueMovementRule {
        self.relegation_rule
    }

    pub fn relegation_target_league_id(&self) -> Option<Uuid> {
        self.relegation_target_league_id
    }
}

impl Default for PromotionRelegationPolicy {
    fn default() -> Self {
        Self::none()
    }
}
