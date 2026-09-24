use crate::dto::stats::{PlayerKickFoulDecisionStatsDto, PlayerKickFoulStatsDto};
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_kick_foul_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerKickFoulStatsDto> {
    let kf_row =
        arlo_persistence::repositories::match_player_kick_fouls::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    let kf_decision_rows = arlo_persistence::repositories::match_player_kick_fouls::list_by_decision_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    let by_decision: Vec<PlayerKickFoulDecisionStatsDto> = kf_decision_rows
        .into_iter()
        .map(|d| PlayerKickFoulDecisionStatsDto {
            decision_kind: d.decision_kind,
            takes_count: d.takes_count as u32,
        })
        .collect();

    Ok(match kf_row {
        Some(kf) => PlayerKickFoulStatsDto {
            kick_foul_takes: kf.kick_foul_takes as u32,
            by_decision,
        },
        None => PlayerKickFoulStatsDto {
            kick_foul_takes: 0,
            by_decision,
        },
    })
}
