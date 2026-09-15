use crate::error::{DbError, DbResult};
use arlo_domain::{
    GamesPerWeekConflictScope, NeutralOpenerSelectionStrategy, PostponementStrategyKind,
    ScheduleAlgorithmKind,
};

pub fn parse_schedule_algorithm_kind(code: &str) -> DbResult<ScheduleAlgorithmKind> {
    match code {
        "RoundRobinSingleLeg" | "round_robin_single_leg" => {
            Ok(ScheduleAlgorithmKind::RoundRobinSingleLeg)
        }
        "RoundRobinDoubleLeg" | "round_robin_double_leg" => {
            Ok(ScheduleAlgorithmKind::RoundRobinDoubleLeg)
        }
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid schedule algorithm kind: {code}"
        ))),
    }
}

pub fn schedule_algorithm_kind_to_code(kind: ScheduleAlgorithmKind) -> &'static str {
    match kind {
        ScheduleAlgorithmKind::RoundRobinSingleLeg => "RoundRobinSingleLeg",
        ScheduleAlgorithmKind::RoundRobinDoubleLeg => "RoundRobinDoubleLeg",
    }
}

pub fn parse_games_per_week_conflict_scope(code: &str) -> DbResult<GamesPerWeekConflictScope> {
    match code {
        "AcrossAllCompetitions" | "across_all_competitions" => {
            Ok(GamesPerWeekConflictScope::AcrossAllCompetitions)
        }
        "SameCompetitionOnly" | "same_competition_only" => {
            Ok(GamesPerWeekConflictScope::SameCompetitionOnly)
        }
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid games per week conflict scope: {code}"
        ))),
    }
}

pub fn games_per_week_conflict_scope_to_code(scope: GamesPerWeekConflictScope) -> &'static str {
    match scope {
        GamesPerWeekConflictScope::AcrossAllCompetitions => "AcrossAllCompetitions",
        GamesPerWeekConflictScope::SameCompetitionOnly => "SameCompetitionOnly",
    }
}

pub fn parse_postponement_strategy_kind(code: &str) -> DbResult<PostponementStrategyKind> {
    match code {
        "NextAvailableByeWeek" | "next_available_bye_week" => {
            Ok(PostponementStrategyKind::NextAvailableByeWeek)
        }
        _ => Err(DbError::InvalidEnum(format!(
            "Invalid postponement strategy kind: {code}"
        ))),
    }
}

pub fn postponement_strategy_kind_to_code(kind: PostponementStrategyKind) -> &'static str {
    match kind {
        PostponementStrategyKind::NextAvailableByeWeek => "NextAvailableByeWeek",
    }
}

pub fn parse_neutral_opener_selection_strategy(
    code: Option<&str>,
) -> DbResult<Option<NeutralOpenerSelectionStrategy>> {
    match code {
        Some("RandomSingleFixture" | "random_single_fixture") => {
            Ok(Some(NeutralOpenerSelectionStrategy::RandomSingleFixture))
        }
        Some(unknown) => Err(DbError::InvalidEnum(format!(
            "Invalid neutral opener selection strategy: {unknown}"
        ))),
        None => Ok(None),
    }
}

pub fn neutral_opener_selection_strategy_to_code(
    strategy: Option<NeutralOpenerSelectionStrategy>,
) -> Option<&'static str> {
    match strategy {
        Some(NeutralOpenerSelectionStrategy::RandomSingleFixture) => Some("RandomSingleFixture"),
        None => None,
    }
}
