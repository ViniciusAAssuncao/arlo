use crate::artrine::constants::CARRY_HELPER_OFFSET_FACTOR;
use crate::artrine::execution::carry_duel::PreparedCarryDuel;
use crate::artrine::execution::context::ActionExecutionContext;
use crate::artrine::execution::drive_detection::detect_drive_crossings;
use crate::artrine::execution::outcome::ArtrineExecutionOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::physical::FatigueState;
use crate::spatial::{run_spatial_tick_loop_with_context, DynamicSpatialMap, MovementContext};
use crate::team_identity::tempo::effort_multiplier_from_value;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::Player;
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashSet;
use uuid::Uuid;

pub fn simulate_carry_kinematics<F>(
    ctx: &ActionExecutionContext<'_, F>,
    artrine: &Player,
    spatial_map: &mut DynamicSpatialMap,
    start_pos: VectorPosition,
    target_pos: VectorPosition,
    raw_advance: f64,
    prep: PreparedCarryDuel<'_>,
) -> ArtrineExecutionOutcome
where
    F: Fn(&Uuid) -> FatigueState,
{
    let artrine_state = ctx.fatigue(&artrine.id());
    spatial_map.set_position(artrine.id(), start_pos);
    spatial_map.apply_breakthrough_momentum(
        artrine,
        target_pos,
        prep.artro_duel.outcome().net_advantage(),
        ctx.attribute_keys,
        artrine_state.energy(),
    );

    let mut movers = Vec::with_capacity(1 + ctx.offense_helpers.len() + ctx.defenders.len());
    movers.push((artrine, target_pos));

    for &helper in ctx.offense_helpers {
        if let Some(pos) = spatial_map.get_position(&helper.id()) {
            let offset_x = if ctx.attacking_positive_x {
                raw_advance * CARRY_HELPER_OFFSET_FACTOR * MIRIM_TO_METERS
            } else {
                -raw_advance * CARRY_HELPER_OFFSET_FACTOR * MIRIM_TO_METERS
            };
            let helper_target =
                VectorPosition::from_components(pos.raw().0 + offset_x, pos.raw().1, 0.0);
            movers.push((helper, helper_target));
        }
    }

    for &defender in ctx.defenders {
        movers.push((defender, target_pos));
    }

    let defense_pressing_value = (ctx.defense_pressing_multiplier - 1.0).max(0.0);
    let offense_ids: HashSet<Uuid> = std::iter::once(artrine.id())
        .chain(ctx.offense_helpers.iter().map(|p| p.id()))
        .collect();
    let effort_multiplier_for = |id: &Uuid| {
        if offense_ids.contains(id) {
            effort_multiplier_from_value(ctx.offense_tempo_value)
        } else {
            effort_multiplier_from_value(defense_pressing_value)
        }
    };

    let tick_result = run_spatial_tick_loop_with_context(
        spatial_map,
        &movers,
        ctx.attribute_tables,
        MovementContext::LivePlay,
        ctx.pitch,
        ctx.fatigue_for,
        &effort_multiplier_for,
    );

    let end_position = spatial_map
        .get_position(&artrine.id())
        .unwrap_or(target_pos);
    let mirins_advanced = (end_position.raw().0 - start_pos.raw().0).abs() / MIRIM_TO_METERS;

    let drive_row_indices = detect_drive_crossings(
        artrine.id(),
        ctx.pitch,
        &tick_result,
        start_pos,
        end_position,
        ctx.attacking_positive_x,
    );
    let drives_recorded = drive_row_indices.len() as u32;

    let mut ledger = DurationLedger::new();
    ledger.record_live(
        DurationComponentKind::ArtroBreakthroughEngagement,
        prep.artro_duration,
    );
    ledger.record_live(
        DurationComponentKind::CarrierMovement,
        Duration::new(tick_result.elapsed_seconds()),
    );

    ArtrineExecutionOutcome {
        mirins_advanced,
        drives_recorded,
        drive_row_indices,
        turnover: None,
        recovering_player_id: None,
        scoring_decision: ScoringDecision::NoOpportunity,
        duration_ledger: ledger,
        end_position,
        duels: vec![prep.artro_duel],
        receiver_id: None,
        distribution_flight: None,
        kinematic_trajectories: tick_result.trajectories().clone(),
    }
}