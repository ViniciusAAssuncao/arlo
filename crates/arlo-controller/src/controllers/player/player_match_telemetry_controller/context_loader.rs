use crate::dto::player_match::PlayerMatchContextDto;
use crate::dto::team::MatchOutcome;
use crate::error::{ControllerError, ControllerResult};
use crate::services::r#match::match_clock_math::{
    calculate_total_match_duration_seconds, format_elapsed_time, MatchClockDurationConfig,
};
use crate::services::r#match::minutes_played_calculator::calculate_player_minutes_played;
use arlo_db::models::position_code::position_to_code;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct MatchPlayerContextResult {
    pub player_name: String,
    pub squad_number: Option<i32>,
    pub position: String,
    pub context: PlayerMatchContextDto,
    pub final_availability_status: String,
    pub final_suspended_remaining_seconds: Option<f64>,
}

pub async fn load_match_context(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<MatchPlayerContextResult> {
    let match_row = arlo_persistence::repositories::match_repo::get_by_id(pool, match_id)
        .await?
        .ok_or_else(|| ControllerError::NotFound(format!("Match {} not found", match_id)))?;

    let player = arlo_db::repositories::player::get_by_id(pool, player_id)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?
        .ok_or_else(|| ControllerError::NotFound(format!("Player {} not found", player_id)))?;

    let player_id_str = player_id.to_string();

    let squad_rows =
        arlo_persistence::repositories::match_squad_selection::list_by_match_id(pool, match_id)
            .await?;

    let squad_row = squad_rows
        .iter()
        .find(|s| s.player_id == player_id_str)
        .ok_or_else(|| {
            ControllerError::NotFound(format!(
                "Player {} was not selected for match {}",
                player_id, match_id
            ))
        })?;

    let is_home = squad_row.team_id == match_row.home_team_id;
    let opponent_team_id_str = if is_home {
        match_row.away_team_id.to_string()
    } else {
        match_row.home_team_id.to_string()
    };

    let player_team_uuid = Uuid::parse_str(&squad_row.team_id)?;
    let opponent_team_uuid = Uuid::parse_str(&opponent_team_id_str)?;

    let player_team = arlo_db::repositories::team::get_by_id(pool, player_team_uuid)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;
    let opponent_team = arlo_db::repositories::team::get_by_id(pool, opponent_team_uuid)
        .await
        .map_err(|e| ControllerError::InvalidData(e.to_string()))?;

    let team_name = player_team
        .map(|t| t.name().to_string())
        .unwrap_or_else(|| "Time".to_string());
    let opponent_team_name = opponent_team
        .map(|t| t.name().to_string())
        .unwrap_or_else(|| "Adversário".to_string());

    let substitutions =
        arlo_persistence::repositories::match_substitutions::list_by_match_id(pool, match_id)
            .await?;

    let clock_config = MatchClockDurationConfig::from_match_row(&match_row);
    let total_match_duration =
        calculate_total_match_duration_seconds(&clock_config, match_row.final_period.max(1) as u32);

    let min_result = calculate_player_minutes_played(
        squad_row.was_starter,
        &squad_row.player_id,
        &substitutions,
        &squad_row.final_availability_status,
        squad_row.final_suspended_remaining_seconds,
        &clock_config,
        total_match_duration,
    );

    let entry_time_formatted = min_result.entry_instant_seconds.map(format_elapsed_time);
    let exit_time_formatted = min_result.exit_instant_seconds.map(format_elapsed_time);

    let team_scores =
        arlo_persistence::repositories::match_team_score::list_by_match_id(pool, match_id).await?;

    let home_score = team_scores
        .iter()
        .find(|s| s.is_home || s.team_id == match_row.home_team_id)
        .map(|s| s.total_points)
        .unwrap_or(0);
    let away_score = team_scores
        .iter()
        .find(|s| !s.is_home || s.team_id == match_row.away_team_id)
        .map(|s| s.total_points)
        .unwrap_or(0);

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

    let position = player
        .positions()
        .iter()
        .max_by_key(|pos| pos.proficiency())
        .map(|pos| position_to_code(pos.position()).to_string())
        .unwrap_or_else(|| "N/A".to_string());

    let context = PlayerMatchContextDto {
        was_starter: squad_row.was_starter,
        was_used: squad_row.was_used,
        formation_slot_index: squad_row.formation_slot_index,
        slot_role: squad_row.slot_role.clone(),
        minutes_played: min_result.minutes_played,
        seconds_played: min_result.seconds_played,
        entry_instant_seconds: min_result.entry_instant_seconds,
        exit_instant_seconds: min_result.exit_instant_seconds,
        entry_time_formatted,
        exit_time_formatted,
        left_due_to_incident: min_result.left_due_to_incident,
        team_id: squad_row.team_id.to_string(),
        team_name,
        opponent_team_id: opponent_team_id_str.to_string(),
        opponent_team_name,
        is_home,
        outcome,
        team_score,
        opponent_score,
    };

    Ok(MatchPlayerContextResult {
        player_name: player.name().to_string(),
        squad_number: player.squad_number(),
        position,
        context,
        final_availability_status: squad_row.final_availability_status.to_string(),
        final_suspended_remaining_seconds: squad_row.final_suspended_remaining_seconds,
    })
}
