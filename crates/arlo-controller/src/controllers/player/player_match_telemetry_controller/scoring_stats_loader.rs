use crate::dto::stats::{PlayerScoringPostStatsDto, PlayerScoringStatsDto};
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_scoring_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerScoringStatsDto> {
    let scoring_row = arlo_persistence::repositories::match_player_scoring_attempts::get_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    let post_rows = arlo_persistence::repositories::match_player_scoring_attempts::list_by_post_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    let by_post: Vec<PlayerScoringPostStatsDto> = post_rows
        .into_iter()
        .map(|p| PlayerScoringPostStatsDto {
            scoring_post: p.scoring_post,
            attempts: p.attempts as u32,
            converted: p.converted as u32,
            missed: p.missed as u32,
            conversion_rate: p.conversion_rate,
        })
        .collect();

    Ok(match scoring_row {
        Some(s) => PlayerScoringStatsDto {
            attempts: s.attempts as u32,
            converted: s.converted as u32,
            missed: s.missed as u32,
            conversion_rate: s.conversion_rate,
            goal_points_scored: s.goal_points_scored as u32,
            field_points_scored: s.field_points_scored as u32,
            field_goals_scored: s.field_goals_scored as u32,
            total_points_scored: s.total_points_scored as u32,
            by_post,
        },
        None => PlayerScoringStatsDto {
            by_post,
            ..Default::default()
        },
    })
}
