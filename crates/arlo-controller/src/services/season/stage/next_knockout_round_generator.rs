use crate::domain::calendar::CalendarSystem;
use crate::domain::season::{BracketSeed, Fixture};
use crate::error::ControllerResult;
use crate::services::season::stage::knockout_bracket_generator::{
    generate_knockout_bracket, GeneratedKnockoutBracket,
};
use arlo_domain::{KnockoutLegFormat, SeasonTiming};
use uuid::Uuid;

pub fn generate_next_knockout_round(
    calendar: &CalendarSystem,
    timing: &SeasonTiming,
    reference_year: i64,
    stage_instance_id: Uuid,
    existing_fixtures: &[Fixture],
    winner_seeds: &[BracketSeed],
    leg_format: KnockoutLegFormat,
) -> ControllerResult<GeneratedKnockoutBracket> {
    let start_round_index = existing_fixtures
        .iter()
        .map(|f| f.round_index())
        .max()
        .map(|m| m + 1)
        .unwrap_or(0);

    generate_knockout_bracket(
        calendar,
        timing,
        reference_year,
        stage_instance_id,
        start_round_index,
        winner_seeds,
        leg_format,
    )
}
