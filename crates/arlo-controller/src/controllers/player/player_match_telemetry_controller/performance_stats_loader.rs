use crate::controllers::r#match::match_performance_controller::map_player_performance_row;
use crate::dto::r#match::PlayerMatchPerformanceDto;
use crate::error::ControllerResult;
use arlo_persistence::repositories::player::{
    match_player_performance, match_player_performance_category,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_performance_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<Option<PlayerMatchPerformanceDto>> {
    let Some(row) =
        match_player_performance::get_by_match_id_and_player_id(pool, match_id, player_id).await?
    else {
        return Ok(None);
    };

    let categories = match_player_performance_category::list_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    Ok(Some(map_player_performance_row(
        row,
        categories,
        None,
    )?))
}
