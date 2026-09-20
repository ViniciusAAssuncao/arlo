use crate::domain::season::{Fixture, FixtureResult, FixtureStatus};
use crate::repositories::season::standings_cache;

pub async fn record_fixture_result(
    fixture: &Fixture,
    result: FixtureResult,
) -> Fixture {
    standings_cache::invalidate(&fixture.season_stage_id()).await;
    Fixture::new(
        fixture.id(),
        fixture.season_stage_id(),
        fixture.round_index(),
        fixture.home_team_id(),
        fixture.away_team_id(),
        fixture.is_neutral_venue(),
        fixture.venue_id(),
        fixture.scheduled_date(),
        FixtureStatus::Completed,
        Some(result),
    )
}