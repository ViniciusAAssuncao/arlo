use crate::controllers::r#match::match_lineup_controller::build_match_lineups;
use crate::controllers::r#match::match_manager_controller::build_manager_report;
use crate::controllers::r#match::match_officiating_controller::build_officiating_summary;
use crate::controllers::r#match::match_summary_controller::build_match_summary_with_incidents;
use crate::controllers::r#match::match_team_stats_controller::build_team_stats;
use crate::controllers::r#match::match_timeline_controller::{
    build_timeline, load_all_match_incidents,
};
use crate::dto::r#match::MatchDetailDto;
use crate::error::{ControllerError, ControllerResult};
use crate::services::r#match::match_clock_math::MatchClockDurationConfig;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_match_detail(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<MatchDetailDto> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    let team_scores =
        arlo_persistence::repositories::match_team_score::list_by_match_id(pool, match_id).await?;

    let incidents_bundle = load_all_match_incidents(pool, match_id).await?;
    let clock_config = MatchClockDurationConfig::from_match_row(&match_row);
    let timeline = build_timeline(&clock_config, &incidents_bundle);

    let summary = build_match_summary_with_incidents(
        pool,
        &match_row,
        &team_scores,
        &incidents_bundle.scoring_plays,
        &incidents_bundle.added_time_awards,
    )
    .await?;

    let (home_lineup, away_lineup) =
        build_match_lineups(pool, &match_row, &incidents_bundle.substitutions).await?;

    let officiating = build_officiating_summary(pool, &match_row, &incidents_bundle.fouls).await?;

    let team_stats = build_team_stats(pool, &match_row).await?;

    let manager_summary = build_manager_report(pool, &match_row).await?;

    Ok(MatchDetailDto {
        summary,
        home_lineup,
        away_lineup,
        timeline,
        officiating,
        team_stats,
        manager_summary,
    })
}

pub async fn get_match_detail_by_fixture_id(
    pool: &SqlitePool,
    fixture_id: Uuid,
) -> ControllerResult<MatchDetailDto> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_fixture_id(pool, fixture_id)
        .await?
        .ok_or_else(|| {
            ControllerError::NotFound(format!("Match for fixture {} not found", fixture_id))
        })?;

    let match_id = Uuid::parse_str(&match_row.id)?;
    get_match_detail(pool, match_id).await
}
