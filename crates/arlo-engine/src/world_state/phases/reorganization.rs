use crate::attributes::DEFAULT_PLAYER_ATTRIBUTE_TABLE;
use crate::lineup_runtime::dynamic_anchor::{
    compute_dynamic_anchors_from_tables, AnchorComputationContext,
};
use crate::play_resolution::field_context::PitchState;
use crate::play_resolution::formation_snapshot::build_role_zone_map;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::team_identity::marking::{
    derive_block_marking_roles_from_tables, eligible_block_marking_defenders,
    extract_manager_artro_strategy_fidelity_from_table,
};
use crate::team_identity::tempo::huddle_duration_scale;
use crate::team_identity::transition::{counter_attack_depth_bias, counter_press_engagement_bias};
use crate::world_state::constants::{
    COUNTER_PRESS_SHIFT_PERCENTAGE, DEFAULT_ATTRIBUTE_VALUE, HUDDLE_BASE_MAX_SECONDS,
    HUDDLE_LEADERSHIP_WEIGHT, HUDDLE_MAX_SECONDS, HUDDLE_MIN_SECONDS, HUDDLE_TACTICAL_WEIGHT,
    PITCH_EDGE_MARGIN_MIRIM,
};
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::{AttributeKey, Position as DomainPosition};
use arlo_events::EventSink;
use arlo_math::units::{Duration, Position as VectorPosition, Velocity, MIRIM_TO_METERS};
use uuid::Uuid;

pub fn derive_and_apply_reorganization(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    scrimmage_x_mirim: f64,
    is_post_turnover: bool,
    recovering_player_id: Option<Uuid>,
) -> (Duration, Duration) {
    let pitch = *publisher.state().pitch();
    let home_lineup = publisher.state().home_lineup_arc();
    let away_lineup = publisher.state().away_lineup_arc();
    let player_attribute_tables = publisher.state().teams.player_attribute_tables().clone();

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
                let def_manager_table = publisher.state().teams.away_manager_table();
                let def_instructions = &away_instructions;
                let def_pos_index = publisher
                    .state()
                    .defensive_position_index_for_team(publisher.state().away_team_id());
                let def_players = def_lineup.players();
                let player_refs: Vec<&arlo_domain::Player> =
                    def_players.iter().map(|p| p.as_ref()).collect();
                let eligible = eligible_block_marking_defenders(&player_refs, def_pos_index);
                let press_block_shape = def_instructions.transition().press_block_shape();
                let execution_fidelity =
                    extract_manager_artro_strategy_fidelity_from_table(def_manager_table);
                let roles = derive_block_marking_roles_from_tables(
                    &eligible,
                    ref_pos,
                    publisher.state().spatial_map(),
                    press_block_shape,
                    &player_attribute_tables,
                    execution_fidelity,
                );
                (None, Some(roles))
            } else {
                let def_lineup = &home_lineup;
                let def_manager_table = publisher.state().teams.home_manager_table();
                let def_instructions = &home_instructions;
                let def_pos_index = publisher
                    .state()
                    .defensive_position_index_for_team(publisher.state().home_team_id());
                let def_players = def_lineup.players();
                let player_refs: Vec<&arlo_domain::Player> =
                    def_players.iter().map(|p| p.as_ref()).collect();
                let eligible = eligible_block_marking_defenders(&player_refs, def_pos_index);
                let press_block_shape = def_instructions.transition().press_block_shape();
                let execution_fidelity =
                    extract_manager_artro_strategy_fidelity_from_table(def_manager_table);
                let roles = derive_block_marking_roles_from_tables(
                    &eligible,
                    ref_pos,
                    publisher.state().spatial_map(),
                    press_block_shape,
                    &player_attribute_tables,
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
        attribute_tables: Some(&player_attribute_tables),
    };
    let mut home_targets = compute_dynamic_anchors_from_tables(
        &pitch,
        &home_lineup,
        &player_attribute_tables,
        scrimmage_x_mirim,
        is_home_offense,
        true,
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
        attribute_tables: Some(&player_attribute_tables),
    };
    let mut away_targets = compute_dynamic_anchors_from_tables(
        &pitch,
        &away_lineup,
        &player_attribute_tables,
        scrimmage_x_mirim,
        !is_home_offense,
        false,
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

    for (&id, &target) in &home_targets {
        publisher.state_mut().spatial_map_mut().set_position(id, target);
        publisher.state_mut().spatial_map_mut().set_velocity(id, Velocity::zero());
    }
    for (&id, &target) in &away_targets {
        publisher.state_mut().spatial_map_mut().set_position(id, target);
        publisher.state_mut().spatial_map_mut().set_velocity(id, Velocity::zero());
    }

    let norm_prox = (scrimmage_x_mirim / pitch.length_mirim()).clamp(0.0, 1.0);
    let pitch_state = PitchState::new(
        publisher.state().possession().down(),
        publisher.state().possession().series_state().remaining_mirins_to_target(),
        PitchState::determine_zone_from_proximity(norm_prox),
        arlo_domain::ArtroPlacement::Central,
        norm_prox,
        publisher.state().drives_in_current_series(),
        publisher.state().possession().is_bonus_phase(),
    );

    let (offense_team_id, defense_team_id, offense_instructions, defense_instructions, offense_lineup, defense_lineup) =
        if is_home_offense {
            (
                publisher.state().home_team_id(),
                publisher.state().away_team_id(),
                &home_instructions,
                &away_instructions,
                &home_lineup,
                &away_lineup,
            )
        } else {
            (
                publisher.state().away_team_id(),
                publisher.state().home_team_id(),
                &away_instructions,
                &home_instructions,
                &away_lineup,
                &home_lineup,
            )
        };

    let _role_zone_map = build_role_zone_map(
        offense_lineup,
        offense_team_id,
        offense_instructions,
        defense_lineup,
        defense_team_id,
        defense_instructions,
        &pitch_state,
    );

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
        let table = player_attribute_tables
            .get(&artrine.id())
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
        let tac = extract_attribute_value(table, AttributeKey::TacticalKnowledge);
        let lead = extract_attribute_value(table, AttributeKey::Leadership);
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

    (Duration::new(0.0), Duration::new(huddle_seconds))
}
