use crate::dto::stats::{PlayerDuelKindStatsDto, PlayerDuelStatsDto};
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_duel_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerDuelStatsDto> {
    let duel_row =
        arlo_persistence::repositories::match_player_duels::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    let duel_kinds_rows =
        arlo_persistence::repositories::match_player_duels::list_by_kind_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    let mut saves_attempted = 0u32;
    let mut saves_made = 0u32;
    let by_kind: Vec<PlayerDuelKindStatsDto> = duel_kinds_rows
        .into_iter()
        .map(|k| {
            let is_save_kind = k.duel_kind == "FinishingAttempt"
                || k.duel_kind == "FieldGoalAttempt"
                || k.duel_kind == "KickBlockAttempt";
            if is_save_kind {
                let def_attempts = (k.as_defender_wins + k.as_defender_losses) as u32;
                saves_attempted += def_attempts;
                saves_made += k.as_defender_wins as u32;
            }
            PlayerDuelKindStatsDto {
                duel_kind: k.duel_kind,
                total: k.total as u32,
                wins: k.wins as u32,
                losses: k.losses as u32,
                as_attacker_wins: k.as_attacker_wins as u32,
                as_attacker_losses: k.as_attacker_losses as u32,
                as_defender_wins: k.as_defender_wins as u32,
                as_defender_losses: k.as_defender_losses as u32,
                win_rate: k.win_rate,
            }
        })
        .collect();

    let save_percentage = if saves_attempted > 0 {
        ((saves_made as f64) / (saves_attempted as f64)) * 100.0
    } else {
        0.0
    };

    Ok(match duel_row {
        Some(d) => PlayerDuelStatsDto {
            total_duels: d.total_duels as u32,
            total_wins: d.total_wins as u32,
            total_losses: d.total_losses as u32,
            win_rate: d.win_rate,
            attacker_duels: d.attacker_duels as u32,
            attacker_wins: d.attacker_wins as u32,
            attacker_losses: d.attacker_losses as u32,
            attacker_win_rate: d.attacker_win_rate,
            defender_duels: d.defender_duels as u32,
            defender_wins: d.defender_wins as u32,
            defender_losses: d.defender_losses as u32,
            defender_win_rate: d.defender_win_rate,
            by_kind,
            saves_attempted,
            saves_made,
            save_percentage,
        },
        None => PlayerDuelStatsDto {
            by_kind,
            saves_attempted,
            saves_made,
            save_percentage,
            ..Default::default()
        },
    })
}
