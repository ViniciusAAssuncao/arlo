use crate::dto::r#match::{PlayerMatchPerformanceDto, PlayerPerformanceBreakdownDto};
use crate::error::ControllerResult;
use crate::repositories::performance::match_player_performance_repository;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_performance_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<Option<PlayerMatchPerformanceDto>> {
    let row = match_player_performance_repository::get_by_match_and_player(pool, match_id, player_id)
        .await?;

    Ok(row.map(|r| PlayerMatchPerformanceDto {
        player_id: r.player_id,
        player_name: None,
        team_id: r.team_id,
        offensive_position: r.offensive_position,
        defensive_position: r.defensive_position,
        slot_role: r.slot_role,
        rating: r.final_rating,
        performance_rating: r.performance_rating,
        outcome_adjustment: r.outcome_adjustment,
        confidence: r.confidence,
        seconds_played: r.seconds_played,
        effective_opportunities: r.effective_opportunities as u32,
        breakdown: PlayerPerformanceBreakdownDto {
            execution: r.execution,
            production: r.production,
            defense: r.defense,
            ball_security: r.ball_security,
            discipline: r.discipline,
            high_impact: r.high_impact,
        },
        model_version: r.model_version,
    }))
}