use crate::dto::player_match::PlayerMatchPhysicalDto;
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_physical_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<Option<PlayerMatchPhysicalDto>> {
    let physical_row =
        arlo_persistence::repositories::match_player_physical::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    Ok(physical_row.map(|p| PlayerMatchPhysicalDto {
        end_energy_level: p.end_energy_level,
        peak_anaerobic_depletion: p.peak_anaerobic_depletion,
        total_distance_covered: p.total_distance_covered,
        intra_match_recovery_amount: p.intra_match_recovery_amount,
    }))
}
