use crate::dto::stats::{PlayerFoulOriginStatsDto, PlayerFoulStatsDto};
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_foul_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
    expulsions: u32,
    time_penalties: u32,
) -> ControllerResult<PlayerFoulStatsDto> {
    let foul_row =
        arlo_persistence::repositories::match_player_fouls::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    let foul_origin_rows = arlo_persistence::repositories::match_player_fouls::list_by_origin_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    let by_origin: Vec<PlayerFoulOriginStatsDto> = foul_origin_rows
        .into_iter()
        .map(|o| PlayerFoulOriginStatsDto {
            origin: o.origin,
            count: o.fouls_count as u32,
        })
        .collect();

    Ok(match foul_row {
        Some(f) => PlayerFoulStatsDto {
            fouls_committed: f.fouls_committed as u32,
            fouls_drawn: f.fouls_drawn as u32,
            correct_calls_committed: f.correct_calls_committed as u32,
            incorrect_calls_committed: f.incorrect_calls_committed as u32,
            expulsions,
            time_penalties,
            by_origin,
        },
        None => PlayerFoulStatsDto {
            expulsions,
            time_penalties,
            by_origin,
            ..Default::default()
        },
    })
}
