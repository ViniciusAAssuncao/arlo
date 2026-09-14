use crate::domain::season::{
    Fixture, FixtureResult, FixtureStatus, GroupRankedStandingsEntry, SeasonInstance,
    StandingsEntry, TieBreakCriterion,
};
use crate::error::ControllerResult;
use crate::services::season::season_generator::{self, GeneratedSeason};
use crate::services::season::standings::group_rank_annotator;
use crate::services::season::standings::standings_calculator;
use crate::services::season::standings::tie_break_resolver;
use arlo_domain::{CompetitionGroup, StandingsPointsPolicy};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn trigger_season_generation(
    pool: &SqlitePool,
    competition_id: Uuid,
    calendar_system_id: Uuid,
    reference_year: i64,
) -> ControllerResult<GeneratedSeason> {
    season_generator::generate_season_for_league(
        pool,
        competition_id,
        calendar_system_id,
        reference_year,
    )
    .await
}

pub fn get_season_instance(
    season_instances: &[SeasonInstance],
    id: Uuid,
) -> Option<SeasonInstance> {
    season_instances.iter().find(|s| s.id() == id).cloned()
}

pub fn list_fixtures_for_team(
    fixtures: &[Fixture],
    team_id: Uuid,
) -> Vec<Fixture> {
    fixtures
        .iter()
        .filter(|f| f.home_team_id() == team_id || f.away_team_id() == team_id)
        .copied()
        .collect()
}

pub fn get_standings(
    team_ids: &[Uuid],
    fixtures: &[Fixture],
    points_policy: &StandingsPointsPolicy,
    criteria: &[TieBreakCriterion],
) -> Vec<StandingsEntry> {
    let unranked = standings_calculator::calculate_standings(team_ids, fixtures, points_policy);
    tie_break_resolver::sort_standings(unranked, criteria)
}

pub fn get_group_ranked_standings(
    team_ids: &[Uuid],
    fixtures: &[Fixture],
    points_policy: &StandingsPointsPolicy,
    criteria: &[TieBreakCriterion],
    groups: &[CompetitionGroup],
) -> Vec<GroupRankedStandingsEntry> {
    let standings = get_standings(team_ids, fixtures, points_policy, criteria);
    group_rank_annotator::annotate_group_ranks(&standings, groups)
}

pub fn record_fixture_result(
    fixture: &Fixture,
    result: FixtureResult,
) -> Fixture {
    Fixture::new(
        fixture.id(),
        fixture.season_stage_id(),
        fixture.round_index(),
        fixture.home_team_id(),
        fixture.away_team_id(),
        fixture.is_neutral_venue(),
        fixture.scheduled_date(),
        FixtureStatus::Completed,
        Some(result),
    )
}