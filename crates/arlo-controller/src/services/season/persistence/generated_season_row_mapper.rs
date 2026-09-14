use crate::domain::season::Fixture;
use crate::services::season::persistence::season_persistence_codes::{
    fixture_status_to_code, season_instance_status_to_code, stage_status_to_code,
    stage_type_to_code,
};
use crate::services::season::season_generator::GeneratedSeason;
use arlo_persistence::models::season::{FixtureRow, SeasonInstanceRow, SeasonStageRow};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn map_fixture_to_row(fixture: &Fixture) -> FixtureRow {
    let (home_score, away_score, home_goal_points, away_goal_points) = match fixture.result() {
        Some(res) => (
            Some(res.home_score()),
            Some(res.away_score()),
            Some(res.home_goal_points()),
            Some(res.away_goal_points()),
        ),
        None => (None, None, None, None),
    };

    FixtureRow::new(
        fixture.id(),
        fixture.season_stage_id(),
        fixture.round_index(),
        fixture.home_team_id(),
        fixture.away_team_id(),
        fixture.is_neutral_venue(),
        fixture.scheduled_date().year(),
        fixture.scheduled_date().day_of_year(),
        fixture_status_to_code(fixture.status()),
        home_score,
        away_score,
        home_goal_points,
        away_goal_points,
    )
}

pub fn map_generated_season_to_rows(
    generated: &GeneratedSeason,
) -> (SeasonInstanceRow, SeasonStageRow, Vec<FixtureRow>) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let season_row = SeasonInstanceRow::new(
        generated.season_instance.id(),
        generated.season_instance.competition_id(),
        generated.season_instance.reference_year(),
        generated.season_instance.current_stage_order_index(),
        season_instance_status_to_code(generated.season_instance.status()),
        now,
    );

    let stage_row = SeasonStageRow::new(
        generated.stage_instance.id(),
        generated.stage_instance.season_instance_id(),
        generated.stage_instance.stage_order_index(),
        stage_type_to_code(generated.stage_instance.stage_type()),
        stage_status_to_code(generated.stage_instance.status()),
    );

    let fixture_rows = generated.fixtures.iter().map(map_fixture_to_row).collect();

    (season_row, stage_row, fixture_rows)
}