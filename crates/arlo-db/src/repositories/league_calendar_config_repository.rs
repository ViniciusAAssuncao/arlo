use crate::error::DbResult;
use crate::models::league_calendar::{
    LeagueCalendarConfigRow, LeagueCalendarMatchdayWeekdayRow, LeagueCalendarStageDefinitionRow,
};
use crate::repositories::fetch::{fetch_all_by_param, fetch_optional_by_param};
use arlo_domain::LeagueCalendarConfig;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_by_competition_id(
    pool: &SqlitePool,
    competition_id: Uuid,
) -> DbResult<Option<LeagueCalendarConfig>> {
    let config_row = fetch_optional_by_param::<LeagueCalendarConfigRow>(
        pool,
        "SELECT id, competition_id, schedule_algorithm_kind, season_start_month_order_index, season_start_day_of_month, season_length_weeks, max_games_per_team_per_week, games_per_week_conflict_scope, postponement_strategy_kind, neutral_opener_enabled, neutral_opener_selection_strategy, created_at_unix_seconds FROM league_calendar_configs WHERE competition_id = ?",
        &competition_id.to_string(),
    )
    .await?;

    let config_row = match config_row {
        Some(row) => row,
        None => return Ok(None),
    };

    let weekday_rows = fetch_all_by_param::<LeagueCalendarMatchdayWeekdayRow>(
        pool,
        "SELECT id, league_calendar_config_id, weekday_order_index FROM league_calendar_matchday_weekdays WHERE league_calendar_config_id = ? ORDER BY weekday_order_index ASC",
        &config_row.id,
    )
    .await?;

    let allowed_weekdays: Vec<u32> = weekday_rows
        .into_iter()
        .map(|row| row.weekday_order_index as u32)
        .collect();

    let stage_rows = fetch_all_by_param::<LeagueCalendarStageDefinitionRow>(
        pool,
        "SELECT id, league_calendar_config_id, stage_order_index, stage_type, entry_rule_kind, entry_rule_count, leg_format FROM league_calendar_stage_definitions WHERE league_calendar_config_id = ? ORDER BY stage_order_index ASC",
        &config_row.id,
    )
    .await?;

    let mut stages = Vec::with_capacity(stage_rows.len());
    for stage_row in stage_rows {
        stages.push(stage_row.to_domain()?);
    }

    let domain = config_row.to_domain(allowed_weekdays, stages)?;
    Ok(Some(domain))
}
