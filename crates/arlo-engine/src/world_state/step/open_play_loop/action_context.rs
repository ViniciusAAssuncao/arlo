use crate::ai::cognitive::RiskProfile;
use crate::ai::gravity::calculate_team_max_finishing_gravity_with_fatigue_from_tables;
use crate::artrine::calculate_normalized_proximity;
use crate::match_decision::target_selection::{calculate_player_target_weight_from_table, ReceptionRole};
use crate::playmaking::resolve_misdirection_logit_offset;
use crate::resolution::DuelContext;
use crate::spatial::{
    calculate_artro_advance_pitch_control_from_tables, calculate_player_expected_free_path_from_tables,
    find_next_artro_position,
};
use crate::world_state::context_analyzer::{analyze_match_state, GameStatePressure};
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use crate::world_state::step::target_weighting::resolve_decision_target_weights;
use arlo_domain::sport_constants::LAUNCHER_TARGET_WEIGHT_MULTIPLIER;
use arlo_domain::{Player, SlotRole};
use arlo_math::units::Position as VectorPosition;
use std::collections::HashMap;
use uuid::Uuid;

pub struct OpenPlayIterationContext<'a> {
    pub target_candidates: Vec<&'a Player>,
    pub normalized_proximity: f64,
    pub best_available_target_weight: f64,
    pub long_launch_target_weight: f64,
    pub openness_by_player: HashMap<Uuid, f64>,
    pub next_artro_pos: VectorPosition,
    pub pitch_control_ahead: f64,
    pub expected_free_path_mirim: f64,
    pub game_state_pressure: GameStatePressure,
    pub risk_profile: RiskProfile,
    pub offensive_gravity_mult: f64,
    pub duel_context: DuelContext,
    pub defense_pressing_multiplier: f64,
    pub offense_tempo_value: f64,
}

impl<'a> OpenPlayIterationContext<'a> {
    pub fn build(
        state: &mut MatchState,
        context: &CallToActionContext,
        pass_phase: &PassPhaseResult<'_>,
        loop_state: &OpenPlayLoopState,
        current_carrier: &'a Player,
        offense_players: &[&'a Player],
        defense_players: &[&Player],
    ) -> Self {
        let pitch = *state.pitch();
        let attribute_keys = state.attribute_keys().clone();
        let carrier_pos = loop_state.current_carrier_pos;

        let normalized_proximity =
            calculate_normalized_proximity(carrier_pos, &pitch, context.is_home_offense);

        let target_candidates: Vec<&Player> = offense_players
            .iter()
            .copied()
            .filter(|p| p.id() != current_carrier.id())
            .collect();

        let (best_available_target_weight, _base_target_weight, openness_by_player) =
            resolve_decision_target_weights(
                context,
                pass_phase,
                state,
                &target_candidates,
                defense_players,
                &attribute_keys,
            );

        let long_launch_target_weight = target_candidates
            .iter()
            .map(|p| {
                let p_state = state.fatigue_lookup().get(&p.id());
                let table = state.attribute_table_for(&p.id());
                let base_weight = calculate_player_target_weight_from_table(
                    p,
                    table,
                    state.spatial_map(),
                    &pitch,
                    &context.offense_pos_index,
                    &context.offense_instructions_index,
                    context.is_home_offense,
                    ReceptionRole::OpenPlayReceiver,
                    &openness_by_player,
                    Some(&p_state),
                );
                if context.offense_role_index.get(&p.id()) == Some(&SlotRole::Launcher) {
                    base_weight * LAUNCHER_TARGET_WEIGHT_MULTIPLIER
                } else {
                    base_weight
                }
            })
            .fold(0.0_f64, f64::max);

        let offensive_gravity = calculate_team_max_finishing_gravity_with_fatigue_from_tables(
            &target_candidates,
            state.teams.player_attribute_tables(),
            &context.offense_pos_index,
            state.spatial_map(),
            &pitch,
            context.is_home_offense,
            &|id| state.fatigue_lookup().get(id),
        );

        let next_artro_pos =
            find_next_artro_position(carrier_pos, &pitch, context.is_home_offense);

        let pitch_control_ahead = calculate_artro_advance_pitch_control_from_tables(
            current_carrier,
            &target_candidates,
            defense_players,
            state.spatial_map(),
            state.teams.player_attribute_tables(),
            &|id| state.fatigue_lookup().get(id),
            carrier_pos,
            next_artro_pos,
            &pitch,
            &context.offense_role_index,
        );

        let expected_free_path_mirim = calculate_player_expected_free_path_from_tables(
            carrier_pos,
            offense_players,
            defense_players,
            state.spatial_map(),
            state.teams.player_attribute_tables(),
            &|id| state.fatigue_lookup().get(id),
            &pitch,
            context.is_home_offense,
        );

        let carrier_fatigue = state.fatigue_lookup().get(&current_carrier.id());
        let carrier_impulse = state.impulse_for(&current_carrier.id());
        let game_state_pressure = analyze_match_state(state);
        let risk_profile = RiskProfile::from_table_with_impulse(
            current_carrier,
            state.attribute_table_for(&current_carrier.id()),
            &carrier_fatigue,
            &carrier_impulse,
        );

        let offense_instructions = *state.instructions_for_team(context.offense_team_id);
        let offense_tempo_value = offense_instructions.in_possession().tempo().value();
        let offense_physicality = offense_instructions.in_possession().physicality();
        let physicality_offset =
            crate::team_identity::physicality::offensive_contact_logit_offset(offense_physicality);
        let defense_instructions = *state.instructions_for_team(context.defense_team_id);
        let defense_pressing_multiplier = crate::team_identity::pressing::contest_radius_multiplier(
            defense_instructions
                .out_of_possession()
                .pressing_intensity(),
        );
        let defense_aggression = defense_instructions.out_of_possession().aggression();
        let aggression_offset =
            crate::team_identity::aggression::duel_logit_offset(defense_aggression);
        let misdirection_offset = resolve_misdirection_logit_offset(
            context.active_play_call.as_ref(),
            &context.offense_route_index,
            &context.offense_lineup,
        );
        let duel_context = DuelContext::with_offsets(
            context.is_home_offense,
            !context.is_home_offense,
            aggression_offset,
            misdirection_offset,
            physicality_offset,
        );

        Self {
            target_candidates,
            normalized_proximity,
            best_available_target_weight,
            long_launch_target_weight,
            openness_by_player,
            next_artro_pos,
            pitch_control_ahead,
            expected_free_path_mirim,
            game_state_pressure,
            risk_profile,
            offensive_gravity_mult: offensive_gravity.multiplier(),
            duel_context,
            defense_pressing_multiplier,
            offense_tempo_value,
        }
    }
}
