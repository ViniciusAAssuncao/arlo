use crate::domain::league_calendar::games_per_week_conflict_scope::GamesPerWeekConflictScope;
use crate::domain::validation::validate_integer_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RestGapPolicy {
    minimum_days_between_fixtures: u32,
    conflict_scope: GamesPerWeekConflictScope,
}

impl RestGapPolicy {
    pub fn new(
        minimum_days_between_fixtures: u32,
        conflict_scope: GamesPerWeekConflictScope,
    ) -> DomainResult<Self> {
        validate_integer_range(
            minimum_days_between_fixtures as i32,
            0,
            30,
            "minimum_days_between_fixtures",
        )?;

        Ok(Self {
            minimum_days_between_fixtures,
            conflict_scope,
        })
    }

    pub fn default_policy() -> Self {
        Self {
            minimum_days_between_fixtures: 3,
            conflict_scope: GamesPerWeekConflictScope::AcrossAllCompetitions,
        }
    }

    pub fn minimum_days_between_fixtures(&self) -> u32 {
        self.minimum_days_between_fixtures
    }

    pub fn conflict_scope(&self) -> GamesPerWeekConflictScope {
        self.conflict_scope
    }
}

impl Default for RestGapPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}
