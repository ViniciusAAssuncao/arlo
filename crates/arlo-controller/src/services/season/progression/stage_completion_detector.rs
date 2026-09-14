use crate::domain::season::{Fixture, FixtureStatus, KnockoutTie, SeasonStageInstance};
use arlo_domain::StageType;

pub fn is_stage_complete(
    stage: &SeasonStageInstance,
    fixtures: &[Fixture],
    knockout_ties: &[KnockoutTie],
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
        StageType::KnockoutBracket => {
            if knockout_ties.is_empty() {
                return false;
            }
            knockout_ties
                .iter()
                .all(|tie| tie.aggregate_winner_team_id().is_some())
        }
        StageType::RoundRobinTable | StageType::GroupedCompetitionTable => true,
    }
}
