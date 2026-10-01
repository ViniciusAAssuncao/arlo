use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::league_calendar::competition_group::CompetitionGroup;
use crate::domain::league_calendar::games_per_week_policy::GamesPerWeekPolicy;
use crate::domain::league_calendar::league_calendar_config_builder::LeagueCalendarConfigBuilder;
use crate::domain::league_calendar::league_calendar_group_reference_validation::{
    validate_entry_rule_group_references, validate_group_order_indices_sequential,
    validate_no_duplicate_team_across_groups, validate_schedule_block_group_references,
};
use crate::domain::league_calendar::league_movement_rule::LeagueMovementRule;
use crate::domain::league_calendar::neutral_opener_policy::NeutralOpenerPolicy;
use crate::domain::league_calendar::postponement_policy::PostponementPolicy;
use crate::domain::league_calendar::promotion_relegation_policy::PromotionRelegationPolicy;
use crate::domain::league_calendar::qta_weighting_policy::QtaWeightingPolicy;
use crate::domain::league_calendar::rest_gap_policy::RestGapPolicy;
use crate::domain::league_calendar::schedule_algorithm_kind::ScheduleAlgorithmKind;
use crate::domain::league_calendar::season_timing::SeasonTiming;
use crate::domain::league_calendar::spa_scoring_policy::SpaScoringPolicy;
use crate::domain::league_calendar::stage_definition::StageDefinition;
use crate::domain::league_calendar::tie_break_criterion::TieBreakCriterion;
use crate::domain::validation::validate_no_duplicate_keys;
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LeagueCalendarConfig {
    id: Uuid,
    league_id: Uuid,
    algorithm: ScheduleAlgorithmKind,
    timing: SeasonTiming,
    games_per_week: GamesPerWeekPolicy,
    rest_gap_policy: RestGapPolicy,
    postponement: PostponementPolicy,
    neutral_opener: NeutralOpenerPolicy,
    spa_scoring_policy: SpaScoringPolicy,
    qta_weighting_policy: QtaWeightingPolicy,
    tie_break_criteria: Vec<TieBreakCriterion>,
    promotion_relegation_policy: PromotionRelegationPolicy,
    stages: Vec<StageDefinition>,
    groups: Vec<CompetitionGroup>,
    collective_agreement_ids: Vec<Uuid>,
}

impl LeagueCalendarConfig {
    pub fn new(
        id: Uuid,
        league_id: Uuid,
        algorithm: ScheduleAlgorithmKind,
        timing: SeasonTiming,
        games_per_week: GamesPerWeekPolicy,
        rest_gap_policy: RestGapPolicy,
        postponement: PostponementPolicy,
        neutral_opener: NeutralOpenerPolicy,
        spa_scoring_policy: SpaScoringPolicy,
        qta_weighting_policy: QtaWeightingPolicy,
        tie_break_criteria: Vec<TieBreakCriterion>,
        promotion_relegation_policy: PromotionRelegationPolicy,
        stages: Vec<StageDefinition>,
        groups: Vec<CompetitionGroup>,
        collective_agreement_ids: Vec<Uuid>,
    ) -> DomainResult<Self> {
        if stages.is_empty() {
            return Err(DomainError::InvalidInvariant {
                field: "stages".to_string(),
                violation: InvariantViolation::Empty,
            });
        }

        validate_no_duplicate_keys(
            &stages,
            |s| s.stage_order_index(),
            "stages",
            "stage_order_index",
        )?;

        validate_no_duplicate_keys(
            &collective_agreement_ids,
            |id| *id,
            "collective_agreement_ids",
            "collective_agreement_id",
        )?;

        let mut sorted_indices: Vec<u32> = stages.iter().map(|s| s.stage_order_index()).collect();
        sorted_indices.sort_unstable();

        for (expected_index, &actual_index) in sorted_indices.iter().enumerate() {
            if actual_index != expected_index as u32 {
                return Err(DomainError::InvalidInvariant {
                    field: "stages".to_string(),
                    violation: InvariantViolation::OutOfIntegerRange {
                        min: 0,
                        max: (stages.len() - 1) as i32,
                    },
                });
            }
        }

        let stage_order_indices_exist =
            |target_index: u32| stages.iter().any(|s| s.stage_order_index() == target_index);

        if !stage_order_indices_exist(promotion_relegation_policy.standings_stage_order_index()) {
            return Err(DomainError::InvalidInvariant {
                field: "promotion_relegation_policy.standings_stage_order_index".to_string(),
                violation: InvariantViolation::UnexpectedValue,
            });
        }

        if let LeagueMovementRule::PlayoffStage {
            stage_order_index, ..
        } = promotion_relegation_policy.promotion_rule()
        {
            if !stage_order_indices_exist(stage_order_index) {
                return Err(DomainError::InvalidInvariant {
                    field: "promotion_relegation_policy.promotion_rule.stage_order_index"
                        .to_string(),
                    violation: InvariantViolation::UnexpectedValue,
                });
            }
        }

        if let LeagueMovementRule::PlayoffStage {
            stage_order_index, ..
        } = promotion_relegation_policy.relegation_rule()
        {
            if !stage_order_indices_exist(stage_order_index) {
                return Err(DomainError::InvalidInvariant {
                    field: "promotion_relegation_policy.relegation_rule.stage_order_index"
                        .to_string(),
                    violation: InvariantViolation::UnexpectedValue,
                });
            }
        }

        validate_no_duplicate_team_across_groups(&groups)?;
        validate_group_order_indices_sequential(&groups)?;
        validate_schedule_block_group_references(&stages, &groups)?;
        validate_entry_rule_group_references(&stages, &groups)?;

        Ok(Self {
            id,
            league_id,
            algorithm,
            timing,
            games_per_week,
            rest_gap_policy,
            postponement,
            neutral_opener,
            spa_scoring_policy,
            qta_weighting_policy,
            tie_break_criteria,
            promotion_relegation_policy,
            stages,
            groups,
            collective_agreement_ids,
        })
    }

    pub fn builder(
        id: Uuid,
        league_id: Uuid,
        algorithm: ScheduleAlgorithmKind,
        timing: SeasonTiming,
        games_per_week: GamesPerWeekPolicy,
        postponement: PostponementPolicy,
    ) -> LeagueCalendarConfigBuilder {
        LeagueCalendarConfigBuilder::new(
            id,
            league_id,
            algorithm,
            timing,
            games_per_week,
            postponement,
        )
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn league_id(&self) -> Uuid {
        self.league_id
    }

    pub fn algorithm(&self) -> ScheduleAlgorithmKind {
        self.algorithm
    }

    pub fn timing(&self) -> &SeasonTiming {
        &self.timing
    }

    pub fn games_per_week(&self) -> GamesPerWeekPolicy {
        self.games_per_week
    }

    pub fn rest_gap_policy(&self) -> RestGapPolicy {
        self.rest_gap_policy
    }

    pub fn postponement(&self) -> PostponementPolicy {
        self.postponement
    }

    pub fn neutral_opener(&self) -> NeutralOpenerPolicy {
        self.neutral_opener
    }

    pub fn spa_scoring_policy(&self) -> &SpaScoringPolicy {
        &self.spa_scoring_policy
    }

    pub fn qta_weighting_policy(&self) -> &QtaWeightingPolicy {
        &self.qta_weighting_policy
    }

    pub fn tie_break_criteria(&self) -> &[TieBreakCriterion] {
        &self.tie_break_criteria
    }

    pub fn promotion_relegation_policy(&self) -> &PromotionRelegationPolicy {
        &self.promotion_relegation_policy
    }

    pub fn stages(&self) -> &[StageDefinition] {
        &self.stages
    }

    pub fn groups(&self) -> &[CompetitionGroup] {
        &self.groups
    }

    pub fn collective_agreement_ids(&self) -> &[Uuid] {
        &self.collective_agreement_ids
    }
}
