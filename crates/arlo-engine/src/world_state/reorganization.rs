use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::{run_spatial_tick_loop_with_context, MovementContext};
use crate::tactics::dynamic_anchor::compute_dynamic_anchors;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::fatigue_applier::apply_kinematic_movement_strain;
use arlo_domain::{AttributeKey, Position as DomainPosition};
use arlo_events::EventSink;
use arlo_math::units::Duration;
use uuid::Uuid;

pub fn derive_and_apply_reorganization(
    state: &mut MatchState,
    scrimmage_x_mirim: f64,
    sink: &mut impl EventSink,
) -> (Duration, Duration) {
    let pitch = *state.pitch();
    let home_lineup = state.home_lineup().clone();
    let away_lineup = state.away_lineup().clone();
    let attribute_keys = state.attribute_keys().clone();

    let is_home_offense = state.possession().role().is_offense(state.home_team_id());

    let home_targets = compute_dynamic_anchors(
        &pitch,
        &home_lineup,
        scrimmage_x_mirim,
        is_home_offense,
        true,
        &attribute_keys,
    );
    let away_targets = compute_dynamic_anchors(
        &pitch,
        &away_lineup,
        scrimmage_x_mirim,
        !is_home_offense,
        false,
        &attribute_keys,
    );

    let mut movers = Vec::with_capacity(home_lineup.len() + away_lineup.len());
    for assignment in home_lineup.assignments() {
        if let Some(&target) = home_targets.get(&assignment.player().id()) {
            movers.push((assignment.player(), target));
        }
    }
    for assignment in away_lineup.assignments() {
        if let Some(&target) = away_targets.get(&assignment.player().id()) {
            movers.push((assignment.player(), target));
        }
    }

    let home_fatigue = state.home_fatigue().clone();
    let away_fatigue = state.away_fatigue().clone();
    let fatigue_lookup = move |id: &Uuid| {
        home_fatigue
            .get(id)
            .or_else(|| away_fatigue.get(id))
            .copied()
            .unwrap_or_default()
    };

    let tick_result = run_spatial_tick_loop_with_context(
        state.spatial_map_mut(),
        &movers,
        &attribute_keys,
        MovementContext::DeadBall,
        &pitch,
        &fatigue_lookup,
    );

    apply_kinematic_movement_strain(state, sink, tick_result.trajectories());

    let offense_lineup = if is_home_offense {
        &home_lineup
    } else {
        &away_lineup
    };

    let offense_artrine = offense_lineup
        .assignments()
        .iter()
        .map(|a| a.player())
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == DomainPosition::Artrine && pos.proficiency() > 0)
        });

    let (tac, lead) = if let Some(artrine) = offense_artrine {
        let tac = extract_attribute_value(artrine, &attribute_keys, AttributeKey::TacticalKnowledge);
        let lead = extract_attribute_value(artrine, &attribute_keys, AttributeKey::Leadership);
        (tac, lead)
    } else {
        (10.0, 10.0)
    };

    let huddle_seconds = (28.0 - (tac * 0.55 + lead * 0.45)).clamp(8.0, 32.0);

    (
        Duration::new(tick_result.elapsed_seconds()),
        Duration::new(huddle_seconds),
    )
}