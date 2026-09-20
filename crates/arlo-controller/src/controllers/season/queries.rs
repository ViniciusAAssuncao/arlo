use crate::domain::season::{Fixture, SeasonInstance};
use uuid::Uuid;

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