use crate::domain::season::{Fixture, FixtureStatus, KnockoutTie, SeasonStageInstance};
use crate::services::season::stage::knockout_bracket_progress_detector::{
    detect_knockout_bracket_progress, KnockoutBracketProgress,
};
use arlo_domain::{StageType, TieBreakCriterion};

pub fn is_stage_complete(
    stage: &SeasonStageInstance,
    fixtures: &[Fixture],
    knockout_ties: &[KnockoutTie],
    criteria: &[TieBreakCriterion],
) -> bool {
    if fixtures.is_empty() {
        return false;
    }

    let all_fixtures_finished = fixtures.iter().all(|f| {
        matches!(
            f.status(),
            FixtureStatus::Completed | FixtureStatus::Cancelled
        )
    });

    if !all_fixtures_finished {
        return false;
    }

    match stage.stage_type() {
        StageType::KnockoutBracket => matches!(
            detect_knockout_bracket_progress(knockout_ties, fixtures, criteria),
            KnockoutBracketProgress::BracketComplete { .. }
        ),
        StageType::RoundRobinTable | StageType::GroupedCompetitionTable => true,
    }
}
