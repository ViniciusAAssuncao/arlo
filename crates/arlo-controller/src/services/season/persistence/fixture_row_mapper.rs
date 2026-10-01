use crate::domain::calendar::CalendarDate;
use crate::domain::season::{Fixture, FixtureResult, FixtureStatus};
use crate::error::ControllerResult;
use arlo_persistence::models::season::FixtureRow;
use uuid::Uuid;

pub fn map_row_to_fixture(row: &FixtureRow) -> ControllerResult<Fixture> {
    let id = Uuid::parse_str(&row.id)?;
    let season_stage_id = Uuid::parse_str(&row.season_stage_id)?;
    let home_team_id = Uuid::parse_str(&row.home_team_id)?;
    let away_team_id = Uuid::parse_str(&row.away_team_id)?;
    let venue_id = row.venue_id.as_deref().map(Uuid::parse_str).transpose()?;

    let status = match row.status.as_str() {
        "Scheduled" => FixtureStatus::Scheduled,
        "Postponed" => FixtureStatus::Postponed,
        "Completed" => FixtureStatus::Completed,
        "Cancelled" => FixtureStatus::Cancelled,
        "Walkover" => FixtureStatus::Completed,
        _ => FixtureStatus::Scheduled,
    };

    let result = match (row.home_score, row.away_score) {
        (Some(home_score), Some(away_score)) => {
            let home_goal_points = row.home_goal_points.unwrap_or(0) as u32;
            let away_goal_points = row.away_goal_points.unwrap_or(0) as u32;
            let home_field_goals = row.home_field_goals.unwrap_or(0) as u32;
            let away_field_goals = row.away_field_goals.unwrap_or(0) as u32;
            let home_field_points = row.home_field_points.unwrap_or(0) as u32;
            let away_field_points = row.away_field_points.unwrap_or(0) as u32;
            let winner_team_id = if home_score > away_score {
                Some(home_team_id)
            } else if away_score > home_score {
                Some(away_team_id)
            } else {
                None
            };
            Some(FixtureResult::new(
                home_score as u32,
                away_score as u32,
                home_goal_points,
                away_goal_points,
                home_field_goals,
                away_field_goals,
                home_field_points,
                away_field_points,
                winner_team_id,
            ))
        }
        _ => None,
    };

    let scheduled_date = CalendarDate::new(row.scheduled_year, row.scheduled_day_of_year as u32);

    Ok(Fixture::new(
        id,
        season_stage_id,
        row.round_index as u32,
        home_team_id,
        away_team_id,
        row.is_neutral_venue,
        venue_id,
        scheduled_date,
        status,
        result,
    ))
}
