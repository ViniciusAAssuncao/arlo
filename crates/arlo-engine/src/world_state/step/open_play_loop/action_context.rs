use crate::ai::cognitive::RiskProfile;
use crate::ai::gravity::calculate_team_max_finishing_gravity_with_fatigue_from_tables;
use crate::match_decision::target_selection::{calculate_player_target_weight, ReceptionRole};
use crate::playmaking::resolve_misdirection_logit_offset;
use crate::resolution::DuelContext;
use crate::world_state::context_analyzer::{analyze_match_state, GameStatePressure};
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtroPlacement, AttributeKey, PitchZone, Player};
use std::collections::HashMap;

pub struct OpenPlayIterationContext<'a> {
    pub target_candidates: Vec<&'a Player>,
    pub normalized_proximity: f64,
    pub zone: PitchZone,
    pub channel: ArtroPlacement,
    pub best_available_target_weight: f64,
    pub long_launch_target_weight: f64,
    pub game_state_pressure: GameStatePressure,
    pub risk_profile: RiskProfile,
    pub offensive_gravity_mult: f64,
    pub carrier_rating: f64,
    pub team_advantage: f64,
    pub duel_context: DuelContext,
    pub defense_pressing_multiplier: f64,
    pub offense_tempo_value: f64,
}

impl<'a> OpenPlayIterationContext<'a> {
    pub fn build(
        state: &mut MatchState,
        context: &CallToActionContext,
        _pass_phase: &PassPhaseResult<'_>,
        current_carrier: &'a Player,
        offense_players: &[&'a Player],
        _defense_players: &[&Player],
    ) -> Self {
        let pitch = *state.pitch();
        let pitch_len_mirim = pitch.length_mirim().max(1.0);
        let cur_x_mirim = state.possession().scrimmage_x_mirim();

        let normalized_proximity = if context.is_home_offense {
            (cur_x_mirim / pitch_len_mirim).clamp(0.0, 1.0)
        } else {
            ((pitch_len_mirim - cur_x_mirim) / pitch_len_mirim).clamp(0.0, 1.0)
        };

        let zone = if normalized_proximity >= 0.88 {
            PitchZone::FirstZone
        } else if normalized_proximity >= 0.72 {
            PitchZone::SecondZone
        } else {
            PitchZone::OpenField
        };

        let channel = ArtroPlacement::Central;

        let target_candidates: Vec<&Player> = offense_players
            .iter()
            .copied()
            .filter(|p| p.id() != current_carrier.id())
            .collect();

        let empty_openness = HashMap::new();
        let best_available_target_weight = target_candidates
            .iter()
            .map(|p| {
                let p_state = state.fatigue_lookup().get(&p.id());
                let table = state.attribute_table_for(&p.id());
                calculate_player_target_weight(
                    p,
                    table,
                    &pitch,
                    &context.offense_pos_index,
                    &context.offense_instructions_index,
                    Some(&context.offense_role_index),
                    context.is_home_offense,
                    ReceptionRole::OpenPlayReceiver,
                    &empty_openness,
                    Some(&p_state),
                )
            })
            .fold(0.0_f64, f64::max);

        let long_launch_target_weight = best_available_target_weight;

        let offensive_gravity = calculate_team_max_finishing_gravity_with_fatigue_from_tables(
            &target_candidates,
            state.teams.player_attribute_tables(),
            &context.offense_pos_index,
            &pitch,
            context.is_home_offense,
            &|id| state.fatigue_lookup().get(id),
        );

        let carrier_table = state.attribute_table_for(&current_carrier.id());
        let carrier_fatigue = state.fatigue_lookup().get(&current_carrier.id());
        let carrier_impulse = state.impulse_for(&current_carrier.id());

        let carrier_rating = carrier_table.get(AttributeKey::ArloControl) * 0.5
            + carrier_table.get(AttributeKey::DriveTechnique) * 0.5;

        let offense_power = state.power_for_team(context.offense_team_id);
        let defense_power = state.power_for_team(context.defense_team_id);
        let team_advantage = offense_power.offensive_power() - defense_power.defensive_power();

        let game_state_pressure = analyze_match_state(state);
        let risk_profile = RiskProfile::from_table_with_impulse(
            current_carrier,
            carrier_table,
            &carrier_fatigue,
            &carrier_impulse,
        );

        let offense_instructions = *state.instructions_for_team(context.offense_team_id);
        let defense_instructions = *state.instructions_for_team(context.defense_team_id);

        let offense_tempo_value = offense_instructions.in_possession().tempo().value();
        let offense_physicality = offense_instructions.in_possession().physicality();
        let physicality_offset =
            crate::team_identity::physicality::offensive_contact_logit_offset(offense_physicality);
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
            zone,
            channel,
            best_available_target_weight,
            long_launch_target_weight,
            game_state_pressure,
            risk_profile,
            offensive_gravity_mult: offensive_gravity.multiplier(),
            carrier_rating,
            team_advantage,
            duel_context,
            defense_pressing_multiplier,
            offense_tempo_value,
        }
    }
}