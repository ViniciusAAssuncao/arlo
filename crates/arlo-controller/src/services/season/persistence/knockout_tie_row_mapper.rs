use crate::domain::season::{BracketSeed, KnockoutTie};
use crate::error::ControllerResult;
use arlo_persistence::models::season::KnockoutTieRow;
use uuid::Uuid;

pub fn map_row_to_knockout_tie(row: &KnockoutTieRow) -> ControllerResult<KnockoutTie> {
    let id = Uuid::parse_str(&row.id)?;
    let season_stage_id = Uuid::parse_str(&row.season_stage_id)?;
    let high_seed_team_id = Uuid::parse_str(&row.high_seed_team_id)?;
    let low_seed_team_id = Uuid::parse_str(&row.low_seed_team_id)?;
    let leg_one_fixture_id = Uuid::parse_str(&row.leg_one_fixture_id)?;
    let leg_two_fixture_id = row
        .leg_two_fixture_id
        .as_deref()
        .map(Uuid::parse_str)
        .transpose()?;
    let aggregate_winner_team_id = row
        .aggregate_winner_team_id
        .as_deref()
        .map(Uuid::parse_str)
        .transpose()?;

    let high_seed = BracketSeed::new(row.high_seed_number as u32, high_seed_team_id);
    let low_seed = BracketSeed::new(row.low_seed_number as u32, low_seed_team_id);

    Ok(KnockoutTie::new(
        id,
        season_stage_id,
        row.round_index as u32,
        row.tie_index as u32,
        high_seed,
        low_seed,
        leg_one_fixture_id,
        leg_two_fixture_id,
        aggregate_winner_team_id,
    ))
}
