use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::league_calendar::competition_group::CompetitionGroup;
use crate::domain::league_calendar::games_per_week_policy::GamesPerWeekPolicy;
use crate::domain::league_calendar::league_calendar_group_reference_validation::{
    validate_group_order_indices_sequential, validate_no_duplicate_team_across_groups,
    validate_schedule_block_group_references,
};
use crate::domain::league_calendar::neutral_opener_policy::NeutralOpenerPolicy;
use crate::domain::league_calendar::postponement_policy::PostponementPolicy;
use crate::domain::league_calendar::schedule_algorithm_kind::ScheduleAlgorithmKind;
use crate::domain::league_calendar::season_timing::SeasonTiming;
use crate::domain::league_calendar::stage_definition::StageDefinition;
use crate::domain::validation::validate_no_duplicate_keys;
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeagueCalendarConfig {
    id: Uuid,
    league_id: Uuid,
    algorithm: ScheduleAlgorithmKind,
    timing: SeasonTiming,
    games_per_week: GamesPerWeekPolicy,
    postponement: PostponementPolicy,
    neutral_opener: NeutralOpenerPolicy,
    stages: Vec<StageDefinition>,
    groups: Vec<CompetitionGroup>,
}

impl LeagueCalendarConfig {
    pub fn new(
        id: Uuid,
        league_id: Uuid,
        algorithm: ScheduleAlgorithmKind,
        timing: SeasonTiming,
        games_per_week: GamesPerWeekPolicy,
        postponement: PostponementPolicy,
        neutral_opener: NeutralOpenerPolicy,
        stages: Vec<StageDefinition>,
        groups: Vec<CompetitionGroup>,
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

        validate_no_duplicate_team_across_groups(&groups)?;
        validate_group_order_indices_sequential(&groups)?;
        validate_schedule_block_group_references(&stages, &groups)?;

        Ok(Self {
            id,
            league_id,
            algorithm,
            timing,
            games_per_week,
            postponement,
            neutral_opener,
            stages,
            groups,
        })
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

    pub fn postponement(&self) -> PostponementPolicy {
        self.postponement
    }

    pub fn neutral_opener(&self) -> NeutralOpenerPolicy {
        self.neutral_opener
    }

    pub fn stages(&self) -> &[StageDefinition] {
        &self.stages
    }

    pub fn groups(&self) -> &[CompetitionGroup] {
        &self.groups
    }
}
