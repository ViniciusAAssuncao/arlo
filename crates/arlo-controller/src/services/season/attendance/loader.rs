use crate::domain::calendar::{CalendarDate, CalendarDaySocialRole, ResolvedCalendarDate};
use crate::error::ControllerResult;
use crate::repositories::calendar::calendar_catalog_cache::get_or_load_calendar_catalog;
use crate::repositories::season::standings_cache::get_or_compute_standings;
use crate::services::calendar::date_resolver;
use crate::services::season::attendance::model::{self, EventDemand, TeamDemand, VenueDemand};
use arlo_domain::Team;
use arlo_persistence::models::season::FixtureRow;
use arlo_persistence::persister::match_persistence_context::AttendanceSnapshot;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

async fn recent_form(
    pool: &SqlitePool,
    team_id: Uuid,
    fixture: &FixtureRow,
) -> ControllerResult<f64> {
    let rows = sqlx::query(
        "SELECT home_team_id, home_score, away_score FROM fixtures \
         WHERE status = 'Completed' AND (home_team_id = ? OR away_team_id = ?) \
         AND (scheduled_year < ? OR (scheduled_year = ? AND scheduled_day_of_year < ?)) \
         ORDER BY scheduled_year DESC, scheduled_day_of_year DESC, id DESC LIMIT 8",
    )
    .bind(team_id.to_string())
    .bind(team_id.to_string())
    .bind(fixture.scheduled_year)
    .bind(fixture.scheduled_year)
    .bind(fixture.scheduled_day_of_year)
    .fetch_all(pool)
    .await?;
    let mut weighted = 0.0;
    let mut weights = 0.0;
    for (index, row) in rows.iter().enumerate() {
        let home_id: String = row.try_get("home_team_id")?;
        let home_score: Option<i32> = row.try_get("home_score")?;
        let away_score: Option<i32> = row.try_get("away_score")?;
        let (Some(home_score), Some(away_score)) = (home_score, away_score) else {
            continue;
        };
        let margin = if home_id == team_id.to_string() {
            home_score - away_score
        } else {
            away_score - home_score
        };
        let weight = 0.82_f64.powi(index as i32);
        weighted += weight * margin.signum() as f64;
        weights += weight;
    }
    Ok(if weights > 0.0 {
        weighted / weights
    } else {
        0.0
    })
}

async fn team_demand(
    pool: &SqlitePool,
    team: &Team,
    fixture: &FixtureRow,
    squad_quality: f64,
) -> ControllerResult<Option<TeamDemand>> {
    let (Some(minimum), Some(maximum)) = (team.min_attendance(), team.max_attendance()) else {
        return Ok(None);
    };
    let titles: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM titles WHERE winner_team_id = ?")
        .bind(team.id().to_string())
        .fetch_one(pool)
        .await?;
    Ok(Some(TeamDemand {
        id: team.id(),
        prestige: team.prestige(),
        minimum,
        maximum,
        titles,
        form: recent_form(pool, team.id(), fixture).await?,
        rating: None,
        squad_quality,
        country_id: team.country_id(),
        home_venue_id: team.home_venue_id(),
    }))
}

async fn latest_rating(
    pool: &SqlitePool,
    season_instance_id: &str,
    team_id: Uuid,
    fixture: &FixtureRow,
) -> ControllerResult<Option<f64>> {
    Ok(sqlx::query_scalar(
        "SELECT e.rating FROM power_ranking_entries e \
         JOIN power_ranking_snapshots s ON s.id = e.snapshot_id \
         WHERE s.season_instance_id = ? AND e.team_id = ? \
         AND (s.year < ? OR (s.year = ? AND s.day_of_year <= ?)) \
         ORDER BY s.year DESC, s.day_of_year DESC, s.model_version DESC LIMIT 1",
    )
    .bind(season_instance_id)
    .bind(team_id.to_string())
    .bind(fixture.scheduled_year)
    .bind(fixture.scheduled_year)
    .bind(fixture.scheduled_day_of_year)
    .fetch_optional(pool)
    .await?)
}

async fn calendar_effect(
    pool: &SqlitePool,
    save_uuid: Uuid,
    fixture: &FixtureRow,
) -> ControllerResult<f64> {
    let calendar_id: Option<String> = sqlx::query_scalar(
        "SELECT calendar_system_id FROM save_calendar_states WHERE save_uuid = ?",
    )
    .bind(save_uuid.to_string())
    .fetch_optional(pool)
    .await?;
    let Some(calendar_id) = calendar_id else {
        return Ok(0.0);
    };
    let calendar_id = Uuid::parse_str(&calendar_id)?;
    let catalog = get_or_load_calendar_catalog(pool).await?;
    let Some(calendar) = catalog.get(&calendar_id) else {
        return Ok(0.0);
    };
    let date = CalendarDate::new(fixture.scheduled_year, fixture.scheduled_day_of_year as u32);
    let weekday = match date_resolver::resolve(calendar, &date) {
        ResolvedCalendarDate::RegularDay { week_day_index, .. } => Some(week_day_index),
        ResolvedCalendarDate::IntercalaryDay { week_day_index, .. } => week_day_index,
    };
    let Some(index) = weekday.map(|index| index as usize) else {
        return Ok(0.0);
    };
    let days = calendar.week_days();
    let Some(day) = days.get(index) else {
        return Ok(0.0);
    };
    let effect = match day.social_role() {
        Some(CalendarDaySocialRole::RestDay) => 0.07,
        Some(CalendarDaySocialRole::Workday) => {
            let previous = days[(index + days.len() - 1) % days.len()].social_role();
            let next = days[(index + 1) % days.len()].social_role();
            if next == Some(CalendarDaySocialRole::RestDay) {
                0.025
            } else if previous == Some(CalendarDaySocialRole::RestDay) {
                -0.065
            } else {
                -0.035
            }
        }
        None => 0.0,
    };
    Ok(effect)
}

async fn table_stakes(
    pool: &SqlitePool,
    fixture: &FixtureRow,
    progress: f64,
) -> ControllerResult<f64> {
    let stage_id = Uuid::parse_str(&fixture.season_stage_id)?;
    let standings = get_or_compute_standings(pool, stage_id).await?;
    let count = standings.len();
    if count < 4 {
        return Ok(0.0);
    }
    let edge = (count as f64 * 0.25).max(1.0);
    let mut significance: f64 = 0.0;
    for team_id in [&fixture.home_team_id, &fixture.away_team_id] {
        if let Some(rank) = standings
            .iter()
            .position(|entry| entry.team_id().to_string() == *team_id)
        {
            let top = (1.0 - rank as f64 / edge).clamp(0.0, 1.0);
            let bottom = (1.0 - (count - rank - 1) as f64 / edge).clamp(0.0, 1.0);
            significance = significance.max(top.max(bottom));
        }
    }
    Ok(progress.clamp(0.0, 1.0) * significance)
}

pub async fn prepare_for_fixture(
    pool: &SqlitePool,
    save_uuid: Uuid,
    fixture: &FixtureRow,
    venue_id: Option<Uuid>,
    seed: u64,
    home_squad_quality: f64,
    away_squad_quality: f64,
) -> ControllerResult<Option<AttendanceSnapshot>> {
    let Some(venue_id) = venue_id else {
        return Ok(None);
    };
    let venue = arlo_db::repositories::venue::get_by_id(pool, venue_id)
        .await
        .map_err(|error| crate::error::ControllerError::InvalidData(error.to_string()))?;
    let Some(venue) = venue else { return Ok(None) };
    let Some(capacity) = venue.capacity().filter(|capacity| *capacity > 0) else {
        return Ok(None);
    };
    let home_id = Uuid::parse_str(&fixture.home_team_id)?;
    let away_id = Uuid::parse_str(&fixture.away_team_id)?;
    let home_team = arlo_db::repositories::team::get_by_id(pool, home_id)
        .await
        .map_err(|error| crate::error::ControllerError::InvalidData(error.to_string()))?;
    let away_team = arlo_db::repositories::team::get_by_id(pool, away_id)
        .await
        .map_err(|error| crate::error::ControllerError::InvalidData(error.to_string()))?;
    let Some(home_team) = home_team else {
        return Ok(None);
    };
    let Some(away_team) = away_team else {
        return Ok(None);
    };
    let Some(mut home) = team_demand(pool, &home_team, fixture, home_squad_quality).await? else {
        return Ok(None);
    };
    let Some(mut away) = team_demand(pool, &away_team, fixture, away_squad_quality).await? else {
        return Ok(None);
    };
    let row = sqlx::query(
        "SELECT s.stage_type, s.season_instance_id, c.prestige, c.scope FROM season_stages s \
         JOIN season_instances i ON i.id = s.season_instance_id \
         JOIN competitions c ON c.id = i.competition_id WHERE s.id = ?",
    )
    .bind(&fixture.season_stage_id)
    .fetch_one(pool)
    .await?;
    let stage_type: String = row.try_get("stage_type")?;
    let season_instance_id: String = row.try_get("season_instance_id")?;
    let competition_prestige: i32 = row.try_get("prestige")?;
    let scope: String = row.try_get("scope")?;
    home.rating = latest_rating(pool, &season_instance_id, home.id, fixture).await?;
    away.rating = latest_rating(pool, &season_instance_id, away.id, fixture).await?;
    let knockout = stage_type == "KnockoutBracket";
    let fixture_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM (SELECT home_team_id AS team_id FROM fixtures \
         WHERE season_stage_id = ? AND round_index = ? UNION \
         SELECT away_team_id AS team_id FROM fixtures \
         WHERE season_stage_id = ? AND round_index = ?)",
    )
    .bind(&fixture.season_stage_id)
    .bind(fixture.round_index)
    .bind(&fixture.season_stage_id)
    .bind(fixture.round_index)
    .fetch_one(pool)
    .await?;
    let max_round: Option<i32> =
        sqlx::query_scalar("SELECT MAX(round_index) FROM fixtures WHERE season_stage_id = ?")
            .bind(&fixture.season_stage_id)
            .fetch_one(pool)
            .await?;
    let stage_progress = match max_round {
        Some(last) if last > 0 => (fixture.round_index as f64 / last as f64).clamp(0.0, 1.0),
        _ => 0.0,
    };
    let event = EventDemand {
        competition_prestige,
        international: scope != "National",
        knockout,
        final_round: knockout && fixture_count <= 2,
        stage_progress,
        table_stakes: if knockout {
            0.0
        } else {
            table_stakes(pool, fixture, stage_progress).await?
        },
        calendar_effect: calendar_effect(pool, save_uuid, fixture).await?,
        declared_neutral: fixture.is_neutral_venue,
        seed,
    };
    let venue = VenueDemand {
        id: venue.id(),
        capacity,
        owner_team_id: venue.owner_team_id(),
        country_id: venue.country_id(),
    };
    Ok(Some(model::calculate(home, away, venue, event)))
}
