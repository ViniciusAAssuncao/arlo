use crate::dto::stats::PlayerReceivingStatsDto;
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_receiving_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerReceivingStatsDto> {
    let receiving_row =
        arlo_persistence::repositories::match_player_receiving::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    Ok(match receiving_row {
        Some(r) => PlayerReceivingStatsDto {
            targets: r.targets as u32,
            receptions: r.receptions as u32,
            drops: r.drops as u32,
            catch_rate: r.catch_rate,
            drop_rate: r.drop_rate,
            receiving_mirins: r.receiving_mirins,
            run_after_catch_mirins: r.run_after_catch_mirins,
            longest_reception_mirim: r.longest_reception_mirim,
            average_mirins_per_reception: r.average_mirins_per_reception,
        },
        None => PlayerReceivingStatsDto::default(),
    })
}
