use crate::dto::player_match::PlayerMatchPunishmentDto;
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct PunishmentStatsResult {
    pub punishments: PlayerMatchPunishmentDto,
    pub expulsion_count: u32,
    pub time_penalty_count: u32,
}

pub async fn load_punishment_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PunishmentStatsResult> {
    let punishment_row =
        arlo_persistence::repositories::match_player_punishments::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    let (expulsion_count, time_penalty_count) = match &punishment_row {
        Some(p) => (p.expulsion_count as u32, p.time_penalty_count as u32),
        None => (0, 0),
    };

    let punishments = match punishment_row {
        Some(p) => PlayerMatchPunishmentDto {
            yardage_loss_count: p.yardage_loss_count as u32,
            loss_of_down_count: p.loss_of_down_count as u32,
            loss_of_drive_count: p.loss_of_drive_count as u32,
            time_penalty_count: p.time_penalty_count as u32,
            expulsion_count: p.expulsion_count as u32,
            invalidate_play_count: p.invalidate_play_count as u32,
            total_yardage_loss_mirim: p.total_yardage_loss_mirim,
            total_loss_of_down_count: p.total_loss_of_down_count as u32,
            total_time_penalty_seconds: p.total_time_penalty_seconds,
            total_loss_of_drive_count: p.total_loss_of_drive_count as u32,
        },
        None => PlayerMatchPunishmentDto::default(),
    };

    Ok(PunishmentStatsResult {
        punishments,
        expulsion_count,
        time_penalty_count,
    })
}
