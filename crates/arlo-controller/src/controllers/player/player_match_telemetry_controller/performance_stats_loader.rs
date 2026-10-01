use crate::dto::r#match::{PlayerMatchPerformanceDto, PlayerPerformanceBreakdownDto};
use crate::error::{ControllerError, ControllerResult};
use arlo_persistence::repositories::player::match_player_performance;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_performance_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<Option<PlayerMatchPerformanceDto>> {
    let Some(row) =
        match_player_performance::get_by_match_id_and_player_id(pool, match_id, player_id).await?
    else {
        return Ok(None);
    };

    let effective_opportunities = u32::try_from(row.effective_opportunities).map_err(|_| {
        ControllerError::InvalidData(format!(
            "Invalid effective opportunities: {}",
            row.effective_opportunities
        ))
    })?;
    let model_version = u32::try_from(row.model_version).map_err(|_| {
        ControllerError::InvalidData(format!("Invalid model version: {}", row.model_version))
    })?;

    Ok(Some(PlayerMatchPerformanceDto {
        player_id: row.player_id,
        player_name: None,
        team_id: row.team_id,
        offensive_position: row.offensive_position,
        defensive_position: row.defensive_position,
        slot_role: row.slot_role,
        rating: row.final_rating,
        performance_rating: row.performance_rating,
        outcome_adjustment: row.outcome_adjustment,
        confidence: row.confidence,
        seconds_played: row.seconds_played,
        effective_opportunities,
        breakdown: PlayerPerformanceBreakdownDto {
            execution: row.execution_score,
            production: row.production_score,
            defense: row.defense_score,
            ball_security: row.ball_security_score,
            discipline: row.discipline_score,
            high_impact: row.high_impact_score,
        },
        model_version,
    }))
}
