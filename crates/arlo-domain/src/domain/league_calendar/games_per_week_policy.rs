use crate::domain::league_calendar::games_per_week_conflict_scope::GamesPerWeekConflictScope;
use crate::domain::validation::validate_integer_range;
use crate::error::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GamesPerWeekPolicy {
    max_games_per_team_per_week: u32,
    conflict_scope: GamesPerWeekConflictScope,
}

impl GamesPerWeekPolicy {
    pub fn new(
        max_games_per_team_per_week: u32,
        conflict_scope: GamesPerWeekConflictScope,
    ) -> DomainResult<Self> {
        validate_integer_range(
            max_games_per_team_per_week as i32,
            1,
            7,
            "max_games_per_team_per_week",
        )?;

        Ok(Self {
            max_games_per_team_per_week,
            conflict_scope,
        })
    }

    pub fn max_games_per_team_per_week(&self) -> u32 {
        self.max_games_per_team_per_week
    }

    pub fn conflict_scope(&self) -> GamesPerWeekConflictScope {
        self.conflict_scope
    }
}