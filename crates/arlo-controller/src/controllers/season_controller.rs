use crate::domain::season::{
    Fixture, FixtureResult, FixtureStatus, GroupRankedStandingsEntry, SeasonInstance,
    StandingsEntry,
};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::league_calendar::league_calendar_config_cache::get_or_load_league_calendar_config;
use crate::services::season::season_generator::{self, GeneratedSeason};
use crate::services::season::standings::group_rank_annotator;
use crate::services::season::standings::standings_pipeline;
use arlo_domain::{CompetitionGroup, TieBreakCriterion};
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

pub async fn get_standings(
    pool: &SqlitePool,
    competition_id: Uuid,
    team_ids: &[Uuid],
    fixtures: &[Fixture],
    criteria: &[TieBreakCriterion],
) -> ControllerResult<Vec<StandingsEntry>> {
    let config = get_or_load_league_calendar_config(pool, competition_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "League calendar config for competition {} not found",
                competition_id
            ))
        })?;

    let active_criteria = if criteria.is_empty() {
        config.tie_break_criteria()
    } else {
        criteria
    };

    Ok(standings_pipeline::calculate_and_rank_standings(
        team_ids,
        fixtures,
        config.spa_scoring_policy(),
        config.qta_weighting_policy(),
        active_criteria,
    ))
}

pub async fn get_group_ranked_standings(
    pool: &SqlitePool,
    competition_id: Uuid,
    team_ids: &[Uuid],
    fixtures: &[Fixture],
    criteria: &[TieBreakCriterion],
    groups: &[CompetitionGroup],
) -> ControllerResult<Vec<GroupRankedStandingsEntry>> {
    let standings = get_standings(pool, competition_id, team_ids, fixtures, criteria).await?;
    Ok(group_rank_annotator::annotate_group_ranks(&standings, groups))
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
