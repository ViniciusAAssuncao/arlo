use crate::dto::player_match::{PlayerMatchInjuryByBodyRegionDto, PlayerMatchInjuryDto};
use crate::error::ControllerResult;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn load_injury_stats(
    pool: &SqlitePool,
    match_id: Uuid,
    player_id: Uuid,
) -> ControllerResult<PlayerMatchInjuryDto> {
    let injury_row = arlo_persistence::repositories::match_player_injuries::get_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    let injury_body_region_rows = arlo_persistence::repositories::match_player_injuries::list_by_body_region_by_match_id_and_player_id(
        pool,
        match_id,
        player_id,
    )
    .await?;

    let by_body_region = injury_body_region_rows
        .into_iter()
        .map(|r| PlayerMatchInjuryByBodyRegionDto {
            body_region: r.body_region,
            injuries_count: r.injuries_count as u32,
        })
        .collect();

    Ok(match injury_row {
        Some(inj) => PlayerMatchInjuryDto {
            total_injuries: inj.total_injuries as u32,
            contact_injuries: inj.contact_injuries as u32,
            non_contact_injuries: inj.non_contact_injuries as u32,
            grade_1_injuries: inj.grade_1_injuries as u32,
            grade_2_injuries: inj.grade_2_injuries as u32,
            grade_3_injuries: inj.grade_3_injuries as u32,
            by_body_region,
        },
        None => PlayerMatchInjuryDto {
            by_body_region,
            ..Default::default()
        },
    })
}
