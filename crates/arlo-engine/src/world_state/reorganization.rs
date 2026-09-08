use crate::lineup_runtime::dynamic_anchor::compute_dynamic_anchors;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::{run_spatial_tick_loop_with_context, MovementContext};
use crate::team_identity::tempo::huddle_duration_scale;
use crate::team_identity::transition::{
    counter_attack_depth_bias, counter_press_engagement_bias,
};
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::fatigue_applier::apply_kinematic_movement_strain;
use arlo_domain::{AttributeKey, Position as DomainPosition};
use arlo_events::EventSink;
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use uuid::Uuid;

pub fn derive_and_apply_reorganization(
    state: &mut MatchState,
    scrimmage_x_mirim: f64,
    is_post_turnover: bool,
    sink: &mut impl EventSink,
) -> (Duration, Duration) {
    let pitch = *state.pitch();
    let home_lineup = state.home_lineup().clone();
    let away_lineup = state.away_lineup().clone();
    let attribute_keys = state.attribute_keys().clone();

    let was_home_offense = state.possession().role().is_offense(state.home_team_id());
    let is_home_offense = if is_post_turnover {
        !was_home_offense
    } else {
        was_home_offense
    };

    let home_instructions = state.home_instructions().clone();
    let away_instructions = state.away_instructions().clone();

    let mut home_targets = compute_dynamic_anchors(
        &pitch,
        &home_lineup,
        scrimmage_x_mirim,
        is_home_offense,
        true,
        &attribute_keys,
        &home_instructions,
    );
    let mut away_targets = compute_dynamic_anchors(
        &pitch,
        &away_lineup,
        scrimmage_x_mirim,
        !is_home_offense,
        false,
        &attribute_keys,
        &away_instructions,
    );

    if is_post_turnover {
        let pitch_len = pitch.length().value();
        let min_x = 0.5 * MIRIM_TO_METERS;
        let max_x = pitch_len - 0.5 * MIRIM_TO_METERS;

        if is_home_offense {
            let ca_bias = counter_attack_depth_bias(
                home_instructions.transition().counter_attack_intensity(),
                pitch_len,
            );
            let cp_bias = counter_press_engagement_bias(
                away_instructions.transition().counter_press_intensity(),
                away_instructions.regroup_discipline(),
            );
            let cp_shift = cp_bias * pitch_len * 0.08;

            for pos in home_targets.values_mut() {
                let new_x = (pos.raw().0 + ca_bias).clamp(min_x, max_x);
                *pos = VectorPosition::from_components(new_x, pos.raw().1, pos.raw().2);
            }
            for pos in away_targets.values_mut() {
                let new_x = (pos.raw().0 - cp_shift).clamp(min_x, max_x);
                *pos = VectorPosition::from_components(new_x, pos.raw().1, pos.raw().2);
            }
        } else {
            let ca_bias = counter_attack_depth_bias(
                away_instructions.transition().counter_attack_intensity(),
                pitch_len,
            );
            let cp_bias = counter_press_engagement_bias(
                home_instructions.transition().counter_press_intensity(),
                home_instructions.regroup_discipline(),
            );
            let cp_shift = cp_bias * pitch_len * 0.08;

            for pos in away_targets.values_mut() {
                let new_x = (pos.raw().0 - ca_bias).clamp(min_x, max_x);
                *pos = VectorPosition::from_components(new_x, pos.raw().1, pos.raw().2);
            }
            for pos in home_targets.values_mut() {
                let new_x = (pos.raw().0 + cp_shift).clamp(min_x, max_x);
                *pos = VectorPosition::from_components(new_x, pos.raw().1, pos.raw().2);
            }
        }
    }

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

    let (offense_instructions, defense_instructions) = if is_home_offense {
        (&home_instructions, &away_instructions)
    } else {
        (&away_instructions, &home_instructions)
    };

    let offense_tempo = offense_instructions.in_possession().tempo().value();
    let defense_pressing = defense_instructions.out_of_possession().pressing_intensity().value();

    let tick_result = run_spatial_tick_loop_with_context(
        state.spatial_map_mut(),
        &movers,
        &attribute_keys,
        MovementContext::DeadBall,
        &pitch,
        &fatigue_lookup,
        offense_tempo,
        defense_pressing,
        is_home_offense,
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

    let base_huddle_seconds = (28.0 - (tac * 0.55 + lead * 0.45)).clamp(8.0, 32.0);
    let huddle_scale = huddle_duration_scale(offense_instructions.in_possession().tempo());
    let huddle_seconds = (base_huddle_seconds * huddle_scale).clamp(8.0, 32.0);

    (
        Duration::new(tick_result.elapsed_seconds()),
        Duration::new(huddle_seconds),
    )
}