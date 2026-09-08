use crate::artrine::execution::carry_duel::{prepare_artro_duel, process_failed_carry};
use crate::artrine::execution::carry_simulation::simulate_carry_kinematics;
use crate::artrine::execution::context::ActionExecutionContext;
use crate::artrine::execution::outcome::ArtrineExecutionOutcome;
use crate::physical::FatigueState;
use crate::resolution::aggregate_progression::AggregateProgressionStrategy;
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::Player;
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use rand::Rng;
use uuid::Uuid;

fn resolve_breakthrough<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    start_pos: VectorPosition,
    target_channel_y_m: f64,
    artro_duel_outcome: &crate::resolution::DuelOutcome,
    rng: &mut R,
) -> (f64, VectorPosition)
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let progression_strategy = AggregateProgressionStrategy::default();
    let raw_advance = progression_strategy.resolve_progression(artro_duel_outcome, rng);

    let start_x_mirim = start_pos.raw().0 / MIRIM_TO_METERS;
    let end_x_mirim = if ctx.attacking_positive_x {
        (start_x_mirim + raw_advance).min(ctx.pitch.length_mirim())
    } else {
        (start_x_mirim - raw_advance).max(0.0)
    };

    let target_pos = VectorPosition::from_components(
        end_x_mirim * MIRIM_TO_METERS,
        target_channel_y_m,
        0.0,
    );

    (raw_advance, target_pos)
}

pub fn execute_carry<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    artrine: &Player,
    spatial_map: &mut DynamicSpatialMap,
    start_pos: VectorPosition,
    rng: &mut R,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let prep = prepare_artro_duel(ctx, artrine, spatial_map, start_pos, rng);
    if !prep.artro_duel.outcome().attacker_won() {
        return process_failed_carry(ctx, artrine, spatial_map, start_pos, prep, rng);
    }

    let (raw_advance, target_pos) = resolve_breakthrough(
        ctx,
        start_pos,
        prep.target_channel_y_m,
        prep.artro_duel.outcome(),
        rng,
    );

    simulate_carry_kinematics(
        ctx,
        artrine,
        spatial_map,
        start_pos,
        target_pos,
        raw_advance,
        prep,
    )
}