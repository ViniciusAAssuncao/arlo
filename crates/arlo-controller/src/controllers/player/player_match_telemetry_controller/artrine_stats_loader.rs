use crate::dto::stats::{PlayerArtrineDecisionKindStatsDto, PlayerArtrineDecisionStatsDto};
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_artrine_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerArtrineDecisionStatsDto> {
    let artrine_row = arlo_persistence::repositories::match_player_artrine_decisions::get_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    let artrine_kind_rows = arlo_persistence::repositories::match_player_artrine_decisions::list_by_kind_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    let by_kind: Vec<PlayerArtrineDecisionKindStatsDto> = artrine_kind_rows
        .into_iter()
        .map(|k| PlayerArtrineDecisionKindStatsDto {
            decision_kind: k.decision_kind,
            total: k.total as u32,
            successful: k.successful as u32,
            failed: k.failed as u32,
            mirins_advanced: k.mirins_advanced,
            points_generated: k.points_generated as u32,
            success_rate: k.success_rate,
            average_mirins_advanced: k.average_mirins_advanced,
            average_points_generated: k.average_points_generated,
        })
        .collect();

    Ok(match artrine_row {
        Some(ad) => PlayerArtrineDecisionStatsDto {
            total_decisions: ad.total_decisions as u32,
            total_successful_decisions: ad.total_successful_decisions as u32,
            total_failed_decisions: ad.total_failed_decisions as u32,
            success_rate: ad.success_rate,
            total_mirins_advanced: ad.total_mirins_advanced,
            average_mirins_per_decision: ad.average_mirins_per_decision,
            total_points_generated: ad.total_points_generated as u32,
            goal_points_generated: ad.goal_points_generated as u32,
            field_points_generated: ad.field_points_generated as u32,
            field_goals_generated: ad.field_goals_generated as u32,
            by_kind,
        },
        None => PlayerArtrineDecisionStatsDto {
            by_kind,
            ..Default::default()
        },
    })
}
