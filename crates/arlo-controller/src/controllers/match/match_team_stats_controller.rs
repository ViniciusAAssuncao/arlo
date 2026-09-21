use crate::dto::r#match::{MatchTeamStatsDto, TeamImpulseSummaryDto, TeamPossessionSummaryDto};
use crate::error::{ControllerError, ControllerResult};
use arlo_persistence::models::MatchRow;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_match_team_stats(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<MatchTeamStatsDto> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    build_team_stats(pool, &match_row).await
}

pub async fn build_team_stats(
    pool: &SqlitePool,
    match_row: &MatchRow,
) -> ControllerResult<MatchTeamStatsDto> {
    let match_uuid = Uuid::parse_str(&match_row.id)?;

    let possessions =
        arlo_persistence::repositories::match_team_possession::list_by_match_id(pool, match_uuid)
            .await?;
    let impulses =
        arlo_persistence::repositories::match_team_impulse::list_by_match_id(pool, match_uuid)
            .await?;

    let home_pos = possessions
        .iter()
        .find(|p| p.team_id == match_row.home_team_id);
    let away_pos = possessions
        .iter()
        .find(|p| p.team_id == match_row.away_team_id);

    let home_sec = home_pos.map(|p| p.total_possession_seconds).unwrap_or(0.0);
    let away_sec = away_pos.map(|p| p.total_possession_seconds).unwrap_or(0.0);
    let total_sec = home_sec + away_sec;

    let (home_pct, away_pct) = if total_sec > 0.0 {
        (
            (home_sec / total_sec) * 100.0,
            (away_sec / total_sec) * 100.0,
        )
    } else {
        (50.0, 50.0)
    };

    let home_possession = TeamPossessionSummaryDto {
        team_id: match_row.home_team_id.clone(),
        possession_seconds: home_sec,
        possession_percentage: home_pct,
    };

    let away_possession = TeamPossessionSummaryDto {
        team_id: match_row.away_team_id.clone(),
        possession_seconds: away_sec,
        possession_percentage: away_pct,
    };

    let home_imp = impulses
        .iter()
        .find(|i| i.team_id == match_row.home_team_id)
        .map(|i| TeamImpulseSummaryDto {
            team_id: i.team_id.clone(),
            average_baseline: i.average_baseline,
            current_average_value: i.current_average_value,
            min_average_value: i.min_average_value,
            max_average_value: i.max_average_value,
            average_value: i.average_value,
            time_below_baseline_seconds: i.time_below_baseline_seconds,
            runs_count: i.runs_count,
            longest_run_duration_seconds: i.longest_run_duration_seconds,
            peak_run_average_value: i.peak_run_average_value,
            total_integrated_run_intensity: i.total_integrated_run_intensity,
            average_run_duration_seconds: i.average_run_duration_seconds,
            average_run_intensity: i.average_run_intensity,
        });

    let away_imp = impulses
        .iter()
        .find(|i| i.team_id == match_row.away_team_id)
        .map(|i| TeamImpulseSummaryDto {
            team_id: i.team_id.clone(),
            average_baseline: i.average_baseline,
            current_average_value: i.current_average_value,
            min_average_value: i.min_average_value,
            max_average_value: i.max_average_value,
            average_value: i.average_value,
            time_below_baseline_seconds: i.time_below_baseline_seconds,
            runs_count: i.runs_count,
            longest_run_duration_seconds: i.longest_run_duration_seconds,
            peak_run_average_value: i.peak_run_average_value,
            total_integrated_run_intensity: i.total_integrated_run_intensity,
            average_run_duration_seconds: i.average_run_duration_seconds,
            average_run_intensity: i.average_run_intensity,
        });

    Ok(MatchTeamStatsDto {
        home_possession,
        away_possession,
        home_impulse: home_imp,
        away_impulse: away_imp,
    })
}