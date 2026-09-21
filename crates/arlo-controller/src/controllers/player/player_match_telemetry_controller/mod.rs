pub mod artrine_stats_loader;
pub mod assist_stats_loader;
pub mod availability_stats_loader;
pub mod context_loader;
pub mod drive_stats_loader;
pub mod duel_stats_loader;
pub mod foul_stats_loader;
pub mod impulse_stats_loader;
pub mod injury_stats_loader;
pub mod kick_foul_stats_loader;
pub mod physical_stats_loader;
pub mod punishment_stats_loader;
pub mod receiving_stats_loader;
pub mod scoring_stats_loader;
pub mod touch_stats_loader;

pub use artrine_stats_loader::*;
pub use assist_stats_loader::*;
pub use availability_stats_loader::*;
pub use context_loader::*;
pub use drive_stats_loader::*;
pub use duel_stats_loader::*;
pub use foul_stats_loader::*;
pub use impulse_stats_loader::*;
pub use injury_stats_loader::*;
pub use kick_foul_stats_loader::*;
pub use physical_stats_loader::*;
pub use punishment_stats_loader::*;
pub use receiving_stats_loader::*;
pub use scoring_stats_loader::*;
pub use touch_stats_loader::*;

use crate::dto::player_match::PlayerMatchTelemetryDto;
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn get_player_match_telemetry(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerMatchTelemetryDto> {
    let context_result = context_loader::load_match_context(pool, match_id, player_id).await?;
    let touches = touch_stats_loader::load_touch_stats(pool, match_id, player_id).await?;
    let drives = drive_stats_loader::load_drive_stats(pool, match_id, player_id).await?;
    let duels = duel_stats_loader::load_duel_stats(pool, match_id, player_id).await?;
    let receiving = receiving_stats_loader::load_receiving_stats(pool, match_id, player_id).await?;
    let scoring = scoring_stats_loader::load_scoring_stats(pool, match_id, player_id).await?;
    let assists = assist_stats_loader::load_assist_stats(pool, match_id, player_id).await?;
    let artrine_decisions = artrine_stats_loader::load_artrine_stats(pool, match_id, player_id).await?;
    let punishment_result = punishment_stats_loader::load_punishment_stats(pool, match_id, player_id).await?;
    let fouls = foul_stats_loader::load_foul_stats(
        pool,
        match_id,
        player_id,
        punishment_result.expulsion_count,
        punishment_result.time_penalty_count,
    )
    .await?;
    let kick_fouls = kick_foul_stats_loader::load_kick_foul_stats(pool, match_id, player_id).await?;
    let impulse = impulse_stats_loader::load_impulse_stats(pool, match_id, player_id).await?;
    let physical = physical_stats_loader::load_physical_stats(pool, match_id, player_id).await?;
    let availability = availability_stats_loader::load_availability_stats(
        pool,
        match_id,
        player_id,
        context_result.final_availability_status,
        context_result.final_suspended_remaining_seconds,
    )
    .await?;
    let injuries = injury_stats_loader::load_injury_stats(pool, match_id, player_id).await?;

    Ok(PlayerMatchTelemetryDto {
        match_id: match_id.to_string(),
        player_id: player_id.to_string(),
        player_name: context_result.player_name,
        squad_number: context_result.squad_number,
        position: context_result.position,
        context: context_result.context,
        touches,
        drives,
        duels,
        receiving,
        scoring,
        assists,
        artrine_decisions,
        fouls,
        kick_fouls,
        impulse,
        physical,
        availability,
        injuries,
        punishments: punishment_result.punishments,
    })
}