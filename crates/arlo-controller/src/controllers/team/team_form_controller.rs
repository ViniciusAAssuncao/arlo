use crate::controllers::season::league_overview::get_league_overview;
use crate::controllers::season::overview_calendar::resolve_overview_calendar;
use crate::domain::calendar::{CalendarDate, ResolvedCalendarDate};
use crate::dto::team::{MatchOutcome, TeamFormEntryDto, TeamStandingsPositionDto};
use crate::error::{ControllerError, ControllerResult};
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::services::calendar::date_resolver;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_team_recent_form(
    pool: &SqlitePool,
    team_id: Uuid,
    limit: u32,
) -> ControllerResult<Vec<TeamFormEntryDto>> {
    let fixture_rows = arlo_persistence::repositories::season::fixtures::list_completed_by_team_id_desc(
        pool,
        team_id,
        limit,
    )
    .await?;

    if fixture_rows.is_empty() {
        return Ok(Vec::new());
    }

    let catalog = get_or_load_calendar_catalog(pool).await?;
    let calendar = resolve_overview_calendar(pool, &catalog).await?;

    let teams = arlo_db::repositories::team::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let team_name_map: HashMap<Uuid, String> = teams
        .iter()
        .map(|t| (t.id(), t.name().to_string()))
        .collect();

    let all_venues = arlo_db::repositories::venue::list_all(pool)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let venue_name_map: HashMap<Uuid, String> = all_venues
        .into_iter()
        .map(|v| (v.id(), v.name().to_string()))
        .collect();

    let mut form_entries = Vec::with_capacity(fixture_rows.len());

    for row in fixture_rows {
        let home_id = Uuid::parse_str(&row.home_team_id).unwrap_or_default();
        let away_id = Uuid::parse_str(&row.away_team_id).unwrap_or_default();
        let is_home = home_id == team_id;
        let opponent_id = if is_home { away_id } else { home_id };
        let opponent_name = team_name_map
            .get(&opponent_id)
            .cloned()
            .unwrap_or_else(|| "Adversário".to_string());

        let home_score = row.home_score.unwrap_or(0);
        let away_score = row.away_score.unwrap_or(0);
        let (team_score, opponent_score) = if is_home {
            (home_score, away_score)
        } else {
            (away_score, home_score)
        };

        let outcome = match team_score.cmp(&opponent_score) {
            std::cmp::Ordering::Greater => MatchOutcome::Win,
            std::cmp::Ordering::Equal => MatchOutcome::Draw,
            std::cmp::Ordering::Less => MatchOutcome::Loss,
        };

        let outcome_code = match outcome {
            MatchOutcome::Win => "V".to_string(),
            MatchOutcome::Draw => "E".to_string(),
            MatchOutcome::Loss => "D".to_string(),
        };

        let fixture_cal_date = CalendarDate::new(
            row.scheduled_year,
            row.scheduled_day_of_year as u32,
        );
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

        let venue_name = row
            .venue_id
            .as_deref()
            .and_then(|vid| Uuid::parse_str(vid).ok())
            .and_then(|vid| venue_name_map.get(&vid).cloned());

        let score_display = format!("{} - {}", team_score, opponent_score);

        form_entries.push(TeamFormEntryDto {
            fixture_id: row.id,
            opponent_id: opponent_id.to_string(),
            opponent_name,
            is_home,
            outcome,
            outcome_code,
            score_display,
            team_score,
            opponent_score,
            scheduled_year: row.scheduled_year,
            scheduled_day_of_year: row.scheduled_day_of_year as u32,
            scheduled_month_name,
            scheduled_day_of_month,
            scheduled_week_day_name,
            venue_name,
        });
    }

    Ok(form_entries)
}

pub async fn get_team_standings_entry(
    pool: &SqlitePool,
    team_id: Uuid,
) -> ControllerResult<Option<TeamStandingsPositionDto>> {
    let team = arlo_db::repositories::team::get_by_id(pool, team_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Team {} not found", team_id)))?;

    let league_id = match team.league_id() {
        Some(id) => id,
        None => return Ok(None),
    };

    let overview = get_league_overview(pool, league_id).await?;
    if !overview.has_active_season || overview.standings.is_empty() {
        return Ok(None);
    }

    let team_id_str = team_id.to_string();
    let position_idx = match overview.standings.iter().position(|e| e.team_id == team_id_str) {
        Some(idx) => idx,
        None => return Ok(None),
    };

    let total_teams = overview.standings.len() as u32;
    let entry = overview.standings.into_iter().nth(position_idx).unwrap();

    Ok(Some(TeamStandingsPositionDto {
        position: (position_idx + 1) as u32,
        total_teams,
        entry,
    }))
}