use crate::dto::player_match::PlayerMatchAvailabilityDto;
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_availability_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
    final_availability_status: String,
    final_suspended_remaining_seconds: Option<f64>,
) -> ControllerResult<PlayerMatchAvailabilityDto> {
    let availability_row =
        arlo_persistence::repositories::match_player_availability::get_by_match_id_and_player_id(
            pool, match_id, player_id,
        )
        .await?;

    Ok(match availability_row {
        Some(a) => PlayerMatchAvailabilityDto {
            final_availability_status,
            total_suspended_seconds: a.total_suspended_seconds,
            final_suspended_remaining_seconds,
            expulsion_count: a.expulsion_count as u32,
            is_currently_expelled: a.is_currently_expelled,
        },
        None => PlayerMatchAvailabilityDto {
            final_availability_status,
            total_suspended_seconds: 0.0,
            final_suspended_remaining_seconds,
            expulsion_count: 0,
            is_currently_expelled: false,
        },
    })
}
