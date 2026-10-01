use crate::dto::r#match::{
    MatchPerformanceSummaryDto, PlayerMatchPerformanceDto, PlayerPerformanceBreakdownDto,
};
use crate::error::{ControllerError, ControllerResult};
use arlo_persistence::repositories::player::match_player_performance;
use sqlx::SqlitePool;
use std::collections::HashMap;
use uuid::Uuid;

pub async fn get_match_performance(
    pool: &SqlitePool,
    match_id: Uuid,
) -> ControllerResult<MatchPerformanceSummaryDto> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    let perf_rows = match_player_performance::list_by_match_id(pool, match_id).await?;

    let mut player_names: HashMap<String, String> = HashMap::new();
    for row in &perf_rows {
        if let Ok(pid) = Uuid::parse_str(&row.player_id) {
            if let Ok(Some(player)) = arlo_db::repositories::player::get_by_id(pool, pid).await {
                player_names.insert(row.player_id.clone(), player.name().to_string());
            }
        }
    }

    let mut home_ratings = Vec::new();
    let mut away_ratings = Vec::new();
    let mut home_sum = 0.0;
    let mut away_sum = 0.0;

    for row in perf_rows {
        let player_name = player_names.get(&row.player_id).cloned();
        let is_home = row.team_id == match_row.home_team_id;
        let effective_opportunities = u32::try_from(row.effective_opportunities).map_err(|_| {
            ControllerError::InvalidData(format!(
                "Invalid effective opportunities: {}",
                row.effective_opportunities
            ))
        })?;
        let model_version = u32::try_from(row.model_version).map_err(|_| {
            ControllerError::InvalidData(format!("Invalid model version: {}", row.model_version))
        })?;

        if is_home {
            home_sum += row.final_rating;
        } else {
            away_sum += row.final_rating;
        }

        let dto = PlayerMatchPerformanceDto {
            player_id: row.player_id.clone(),
            player_name,
            team_id: row.team_id.clone(),
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
        };

        if is_home {
            home_ratings.push(dto);
        } else {
            away_ratings.push(dto);
        }
    }

    let home_average_rating = if !home_ratings.is_empty() {
        home_sum / home_ratings.len() as f64
    } else {
        5.5
    };

    let away_average_rating = if !away_ratings.is_empty() {
        away_sum / away_ratings.len() as f64
    } else {
        5.5
    };

    let match_mvp = home_ratings
        .iter()
        .chain(away_ratings.iter())
        .max_by(|a, b| {
            a.rating
                .partial_cmp(&b.rating)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .cloned();

    Ok(MatchPerformanceSummaryDto {
        match_id: match_id.to_string(),
        home_average_rating,
        away_average_rating,
        home_ratings,
        away_ratings,
        match_mvp,
    })
}
