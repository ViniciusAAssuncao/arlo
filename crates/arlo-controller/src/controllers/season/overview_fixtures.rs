use crate::domain::calendar::{CalendarDate, CalendarSystem, ResolvedCalendarDate};
use crate::dto::season::FixtureSummaryDto;
use crate::error::ControllerResult;
use crate::services::calendar::date_resolver;
use arlo_persistence::models::season::FixtureRow;
use arlo_persistence::repositories::match_repository;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn build_overview_fixtures(
    pool: &SqlitePool,
    fixture_rows: Vec<FixtureRow>,
    calendar: &CalendarSystem,
    team_name_map: &HashMap<Uuid, String>,
    team_home_venue_map: &HashMap<Uuid, Option<Uuid>>,
    venue_name_map: &HashMap<Uuid, String>,
) -> ControllerResult<Vec<FixtureSummaryDto>> {
    let fixture_uuids: Vec<Uuid> = fixture_rows
        .iter()
        .filter_map(|r| Uuid::parse_str(&r.id).ok())
        .collect();

    let match_id_map: HashMap<String, String> = if fixture_uuids.is_empty() {
        HashMap::new()
    } else {
        let match_rows = match_repository::list_by_fixture_ids(pool, &fixture_uuids).await?;
        match_rows
            .into_iter()
            .filter_map(|m| m.fixture_id.map(|fid| (fid, m.id)))
            .collect()
    };

    let mut fixtures = Vec::with_capacity(fixture_rows.len());

    for row in fixture_rows {
        let home_id = Uuid::parse_str(&row.home_team_id).unwrap_or_default();
        let away_id = Uuid::parse_str(&row.away_team_id).unwrap_or_default();
        let home_name = team_name_map
            .get(&home_id)
            .cloned()
            .unwrap_or_else(|| "Mandante".to_string());
        let away_name = team_name_map
            .get(&away_id)
            .cloned()
            .unwrap_or_else(|| "Visitante".to_string());

        let fixture_cal_date =
            CalendarDate::new(row.scheduled_year, row.scheduled_day_of_year as u32);

        let resolved_date = date_resolver::resolve(calendar, &fixture_cal_date);

        let (scheduled_month_name, scheduled_day_of_month, scheduled_week_day_name) =
            match resolved_date {
                ResolvedCalendarDate::RegularDay {
                    month_order_index,
                    day_of_month,
                    week_day_index,
                    ..
                } => {
                    let m_name = calendar
                        .months()
                        .iter()
                        .find(|m| m.order_index() == month_order_index)
                        .map(|m| m.name().to_string())
                        .unwrap_or_default();
                    let w_name = calendar
                        .week_days()
                        .iter()
                        .find(|w| w.order_index() == week_day_index)
                        .map(|w| w.name().to_string())
                        .unwrap_or_default();
                    (m_name, day_of_month, w_name)
                }
                ResolvedCalendarDate::IntercalaryDay {
                    intercalary_index,
                    week_day_index,
                    ..
                } => {
                    let w_name = week_day_index
                        .and_then(|idx| {
                            calendar
                                .week_days()
                                .iter()
                                .find(|w| w.order_index() == idx)
                                .map(|w| w.name().to_string())
                        })
                        .unwrap_or_default();
                    ("Sirdápis".to_string(), intercalary_index + 1, w_name)
                }
            };

        let target_venue_id = if row.is_neutral_venue {
            row.venue_id
                .as_deref()
                .and_then(|vid| Uuid::parse_str(vid).ok())
        } else {
            team_home_venue_map
                .get(&home_id)
                .copied()
                .flatten()
                .or_else(|| {
                    row.venue_id
                        .as_deref()
                        .and_then(|vid| Uuid::parse_str(vid).ok())
                })
        };

        let venue_name = target_venue_id.and_then(|vid| venue_name_map.get(&vid).cloned());
        let match_id = match_id_map.get(&row.id).cloned();

        fixtures.push(FixtureSummaryDto {
            id: row.id,
            match_id,
            round_index: row.round_index as u32,
            home_team_id: row.home_team_id.clone(),
            away_team_id: row.away_team_id.clone(),
            home_team_name: home_name,
            away_team_name: away_name,
            status: row.status,
            home_score: row.home_score,
            away_score: row.away_score,
            home_goal_points: row.home_goal_points,
            away_goal_points: row.away_goal_points,
            home_field_goals: row.home_field_goals,
            away_field_goals: row.away_field_goals,
            home_field_points: row.home_field_points,
            away_field_points: row.away_field_points,
            scheduled_year: row.scheduled_year,
            scheduled_day_of_year: row.scheduled_day_of_year as u32,
            scheduled_month_name,
            scheduled_day_of_month,
            scheduled_week_day_name,
            venue_name,
        });
    }

    fixtures.sort_by_key(|f| (f.round_index, f.scheduled_day_of_year));
    Ok(fixtures)
}
