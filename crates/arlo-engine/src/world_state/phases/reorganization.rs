use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::dynamic_anchor::{compute_dynamic_anchors, AnchorComputationContext};
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::{run_spatial_tick_loop_with_context, MovementContext};
use crate::team_identity::marking::{
    derive_block_marking_roles, eligible_block_marking_defenders,
    extract_manager_artro_strategy_fidelity,
};
use crate::team_identity::tempo::{
    effort_multiplier_from_value, huddle_duration_scale, individual_transition_effort_multiplier,
};
use crate::team_identity::transition::{counter_attack_depth_bias, counter_press_engagement_bias};
use crate::world_state::constants::{
    COUNTER_PRESS_SHIFT_PERCENTAGE, DEFAULT_ATTRIBUTE_VALUE, HUDDLE_BASE_MAX_SECONDS,
    HUDDLE_LEADERSHIP_WEIGHT, HUDDLE_MAX_SECONDS, HUDDLE_MIN_SECONDS, HUDDLE_TACTICAL_WEIGHT,
    PITCH_EDGE_MARGIN_MIRIM,
};
use crate::world_state::play_transition::fatigue_applier::apply_kinematic_movement_strain;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::{AttributeKey, Position as DomainPosition};
use arlo_events::EventSink;
use arlo_math::units::{Duration, Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashSet;
use uuid::Uuid;

pub fn derive_and_apply_reorganization(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    scrimmage_x_mirim: f64,
    is_post_turnover: bool,
    recovering_player_id: Option<Uuid>,
) -> (Duration, Duration) {
    let pitch = *publisher.state().pitch();
    let home_lineup = publisher.state().home_lineup().clone();
    let away_lineup = publisher.state().away_lineup().clone();
    let attribute_keys = publisher.state().attribute_keys().clone();

    let was_home_offense = publisher
        .state()
        .possession()
        .role()
        .is_offense(publisher.state().home_team_id());
    let is_home_offense = if is_post_turnover {
        !was_home_offense
    } else {
        was_home_offense
    };

    let home_instructions = *publisher.state().home_instructions();
    let away_instructions = *publisher.state().away_instructions();

    let press_reference_pos = if is_post_turnover {
        recovering_player_id.and_then(|id| publisher.state().spatial_map().get_position(&id))
    } else {
        None
    };

    let (home_block_roles, away_block_roles) = if is_post_turnover {
        if let Some(ref_pos) = press_reference_pos {
            if is_home_offense {
                let def_lineup = &away_lineup;
                let def_manager = publisher.state().away_manager();
                let def_instructions = &away_instructions;
                let def_pos_index = publisher
                    .state()
                    .defensive_position_index_for_team(publisher.state().away_team_id());
                let def_players = def_lineup.players();
                let eligible = eligible_block_marking_defenders(&def_players, def_pos_index);
                let press_block_shape = def_instructions.transition().press_block_shape();
                let execution_fidelity =
                    extract_manager_artro_strategy_fidelity(def_manager, &attribute_keys);
                let roles = derive_block_marking_roles(
                    &eligible,
                    ref_pos,
                    publisher.state().spatial_map(),
                    press_block_shape,
                    &attribute_keys,
                    execution_fidelity,
                );
                (None, Some(roles))
            } else {
                let def_lineup = &home_lineup;
                let def_manager = publisher.state().home_manager();
                let def_instructions = &home_instructions;
                let def_pos_index = publisher
                    .state()
                    .defensive_position_index_for_team(publisher.state().home_team_id());
                let def_players = def_lineup.players();
                let eligible = eligible_block_marking_defenders(&def_players, def_pos_index);
                let press_block_shape = def_instructions.transition().press_block_shape();
                let execution_fidelity =
                    extract_manager_artro_strategy_fidelity(def_manager, &attribute_keys);
                let roles = derive_block_marking_roles(
                    &eligible,
                    ref_pos,
                    publisher.state().spatial_map(),
                    press_block_shape,
                    &attribute_keys,
                    execution_fidelity,
                );
                (Some(roles), None)
            }
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let home_ctx = AnchorComputationContext {
        player_instructions_index: publisher
            .state()
            .instructions_index_for_team(publisher.state().home_team_id()),
        opposing_lineup: Some(&away_lineup),
        spatial_map: Some(publisher.state().spatial_map()),
        block_marking_roles: home_block_roles.as_ref(),
        press_reference_pos: if !is_home_offense {
            press_reference_pos
        } else {
            None
        },
    };
    let mut home_targets = compute_dynamic_anchors(
        &pitch,
        &home_lineup,
        scrimmage_x_mirim,
        is_home_offense,
        true,
        &attribute_keys,
        &home_instructions,
        &home_ctx,
    );

    let away_ctx = AnchorComputationContext {
        player_instructions_index: publisher
            .state()
            .instructions_index_for_team(publisher.state().away_team_id()),
        opposing_lineup: Some(&home_lineup),
        spatial_map: Some(publisher.state().spatial_map()),
        block_marking_roles: away_block_roles.as_ref(),
        press_reference_pos: if is_home_offense {
            press_reference_pos
        } else {
            None
        },
    };
    let mut away_targets = compute_dynamic_anchors(
        &pitch,
        &away_lineup,
        scrimmage_x_mirim,
        !is_home_offense,
        false,
        &attribute_keys,
        &away_instructions,
        &away_ctx,
    );

    let individual_release_tempo = if is_post_turnover {
        recovering_player_id.map(|id| {
            publisher
                .state()
                .player_instructions_for(&id)
                .transition()
                .release_tempo()
        })
    } else {
        None
    };

    if is_post_turnover {
        let pitch_len = pitch.length().value();
        let min_x = PITCH_EDGE_MARGIN_MIRIM * MIRIM_TO_METERS;
        let max_x = pitch_len - PITCH_EDGE_MARGIN_MIRIM * MIRIM_TO_METERS;

        if is_home_offense {
            let ca_bias = counter_attack_depth_bias(
                home_instructions.transition().counter_attack_intensity(),
                pitch_len,
                individual_release_tempo,
            );
            let cp_bias = counter_press_engagement_bias(
                away_instructions.transition().counter_press_intensity(),
                away_instructions.regroup_discipline(),
            );
            let cp_shift = cp_bias * pitch_len * COUNTER_PRESS_SHIFT_PERCENTAGE;

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
                individual_release_tempo,
            );
            let cp_bias = counter_press_engagement_bias(
                home_instructions.transition().counter_press_intensity(),
                home_instructions.regroup_discipline(),
            );
            let cp_shift = cp_bias * pitch_len * COUNTER_PRESS_SHIFT_PERCENTAGE;

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

    let home_fatigue = publisher.state().home_fatigue().clone();
    let away_fatigue = publisher.state().away_fatigue().clone();
    let fatigue_lookup = move |id: &Uuid| {
        home_fatigue
            .get(id)
            .or_else(|| away_fatigue.get(id))
            .copied()
            .unwrap_or_default()
    };

    let home_team_id = publisher.state().home_team_id();
    let away_team_id = publisher.state().away_team_id();
    let home_instr = publisher
        .state()
        .instructions_index_for_team(home_team_id)
        .clone();
    let away_instr = publisher
        .state()
        .instructions_index_for_team(away_team_id)
        .clone();
    let home_ids: HashSet<Uuid> = home_lineup
        .assignments()
        .iter()
        .map(|a| a.player().id())
        .collect();

    let (offense_instructions, defense_instructions) = if is_home_offense {
        (&home_instructions, &away_instructions)
    } else {
        (&away_instructions, &home_instructions)
    };

    let offense_tempo = offense_instructions.in_possession().tempo().value();
    let defense_pressing = defense_instructions
        .out_of_possession()
        .pressing_intensity()
        .value();

    let effort_multiplier_for = |id: &Uuid| {
        let is_home = home_ids.contains(id);
        let is_offense = is_home == is_home_offense;
        let base_mult = if is_offense {
            effort_multiplier_from_value(offense_tempo)
        } else {
            effort_multiplier_from_value(defense_pressing)
        };
        if is_post_turnover {
            let instr = if is_home {
                home_instr.get(id).copied().unwrap_or_default()
            } else {
                away_instr.get(id).copied().unwrap_or_default()
            };
            individual_transition_effort_multiplier(
                base_mult,
                instr.transition().transition_urgency(),
            )
        } else {
            base_mult
        }
    };

    let tick_result = run_spatial_tick_loop_with_context(
        publisher.state_mut().spatial_map_mut(),
        &movers,
        &attribute_keys,
        MovementContext::DeadBall,
        &pitch,
        &fatigue_lookup,
        &effort_multiplier_for,
    );

    apply_kinematic_movement_strain(publisher, tick_result.trajectories());

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
        let table = PlayerAttributeTable::from_player(artrine, &attribute_keys);
        let tac =
            extract_attribute_value(&table, AttributeKey::TacticalKnowledge);
        let lead = extract_attribute_value(&table, AttributeKey::Leadership);
        (tac, lead)
    } else {
        (DEFAULT_ATTRIBUTE_VALUE, DEFAULT_ATTRIBUTE_VALUE)
    };

    let base_huddle_seconds = (HUDDLE_BASE_MAX_SECONDS
        - (tac * HUDDLE_TACTICAL_WEIGHT + lead * HUDDLE_LEADERSHIP_WEIGHT))
        .clamp(HUDDLE_MIN_SECONDS, HUDDLE_MAX_SECONDS);
    let huddle_scale = huddle_duration_scale(offense_instructions.in_possession().tempo());
    let huddle_seconds =
        (base_huddle_seconds * huddle_scale).clamp(HUDDLE_MIN_SECONDS, HUDDLE_MAX_SECONDS);

    (
        Duration::new(tick_result.elapsed_seconds()),
        Duration::new(huddle_seconds),
    )
}