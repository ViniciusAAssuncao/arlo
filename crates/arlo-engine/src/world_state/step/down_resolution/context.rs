use crate::ai::cognitive::RiskProfile;
use crate::ai::gravity::calculate_team_max_finishing_gravity_with_fatigue_from_tables;
use crate::attributes::PlayerAttributeTable;
use crate::match_decision::target_selection::{calculate_player_target_weight, ReceptionRole};
use crate::physical::PhysicalState;
use crate::playmaking::resolve_misdirection_logit_offset;
use crate::psychology::state::ImpulseState;
use crate::resolution::DuelContext;
use crate::world_state::context_analyzer::{analyze_match_state, GameStatePressure};
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtroPlacement, PitchZone, Player, Position, SlotRole};
use arlo_tactics::{DecisionEmphasis, PassingRange, PlayerInstructions};
use std::collections::HashMap;
use uuid::Uuid;

pub struct DownResolutionContext<'a> {
    pub offense_team_id: Uuid,
    pub defense_team_id: Uuid,
    pub carrier: &'a Player,
    pub carrier_table: PlayerAttributeTable,
    pub carrier_fatigue: PhysicalState,
    pub carrier_impulse: ImpulseState,
    pub carrier_pos_domain: Position,
    pub carrier_role: SlotRole,
    pub carrier_instructions: PlayerInstructions,
    pub primary_defender: &'a Player,
    pub primary_defender_table: PlayerAttributeTable,
    pub primary_defender_fatigue: PhysicalState,
    pub primary_defender_pos_domain: Position,
    pub offense_players: Vec<&'a Player>,
    pub defense_players: Vec<&'a Player>,
    pub target_candidates: Vec<&'a Player>,
    pub normalized_proximity: f64,
    pub zone: PitchZone,
    pub channel: ArtroPlacement,
    pub drives_in_series: u32,
    pub remaining_advance_mirim: f64,
    pub down: u8,
    pub is_bonus_phase: bool,
    pub is_true_artrine: bool,
    pub is_home_offense: bool,
    pub pitch_length_mirim: f64,
    pub best_target_weight: f64,
    pub long_launch_target_weight: f64,
    pub offensive_gravity: f64,
    pub team_advantage: f64,
    pub pass_protection_advantage: f64,
    pub game_state_pressure: GameStatePressure,
    pub risk_profile: RiskProfile,
    pub duel_context: DuelContext,
    pub passing_range: PassingRange,
    pub decision_emphasis: DecisionEmphasis,
    pub defense_pressing_multiplier: f64,
    pub offense_tempo_value: f64,
    pub state_advanced_mirins: f64,
}

impl<'a> DownResolutionContext<'a> {
    pub fn build(
        state: &MatchState,
        context: &'a CallToActionContext,
        pass_phase: &'a PassPhaseResult<'a>,
        carrier: &'a Player,
        offense_players: &[&'a Player],
        defense_players: &[&'a Player],
    ) -> Self {
        let pitch_length_mirim = state.pitch().length_mirim();
        let cur_x_mirim = state.possession().scrimmage_x_mirim();

        let normalized_proximity = if context.is_home_offense {
            (cur_x_mirim / pitch_length_mirim.max(1.0)).clamp(0.0, 1.0)
        } else {
            ((pitch_length_mirim - cur_x_mirim) / pitch_length_mirim.max(1.0)).clamp(0.0, 1.0)
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
            .filter(|p| p.id() != carrier.id())
            .collect();

        let empty_openness = HashMap::new();
        let best_target_weight = target_candidates
            .iter()
            .map(|p| {
                let p_state = state.fatigue_lookup().get(&p.id());
                let table = state.attribute_table_for(&p.id());
                calculate_player_target_weight(
                    p,
                    table,
                    state.pitch(),
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

        let long_launch_target_weight = best_target_weight;

        let offensive_gravity = calculate_team_max_finishing_gravity_with_fatigue_from_tables(
            &target_candidates,
            state.teams.player_attribute_tables(),
            &context.offense_pos_index,
            state.pitch(),
            context.is_home_offense,
            &|id| state.fatigue_lookup().get(id),
        );

        let carrier_table = *state.attribute_table_for(&carrier.id());
        let carrier_fatigue = state.fatigue_lookup().get(&carrier.id());
        let carrier_impulse = state.impulse_for(&carrier.id());

        let primary_defender = defense_players.first().copied().unwrap_or(carrier);
        let primary_defender_table = *state.attribute_table_for(&primary_defender.id());
        let primary_defender_fatigue = state.fatigue_lookup().get(&primary_defender.id());

        let carrier_pos_domain = context
            .offense_pos_index
            .get(&carrier.id())
            .copied()
            .unwrap_or(Position::CenterOffense);

        let carrier_role = context
            .offense_role_index
            .get(&carrier.id())
            .copied()
            .unwrap_or(SlotRole::Standard);

        let carrier_instructions = context
            .offense_instructions_index
            .get(&carrier.id())
            .copied()
            .unwrap_or_default();

        let primary_defender_pos_domain = context
            .defense_pos_index
            .get(&primary_defender.id())
            .copied()
            .unwrap_or(Position::Centerback);

        let offense_power = state.power_for_team(context.offense_team_id);
        let defense_power = state.power_for_team(context.defense_team_id);
        let team_advantage = offense_power.offensive_power() - defense_power.defensive_power();

        let game_state_pressure = analyze_match_state(state);
        let risk_profile = RiskProfile::from_table_with_impulse(
            carrier,
            &carrier_table,
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

        let passing_range = offense_instructions.in_possession().passing_range();
        let is_true_artrine = carrier.id() == pass_phase.artrine.id();

        Self {
            offense_team_id: context.offense_team_id,
            defense_team_id: context.defense_team_id,
            carrier,
            carrier_table,
            carrier_fatigue,
            carrier_impulse,
            carrier_pos_domain,
            carrier_role,
            carrier_instructions,
            primary_defender,
            primary_defender_table,
            primary_defender_fatigue,
            primary_defender_pos_domain,
            offense_players: offense_players.to_vec(),
            defense_players: defense_players.to_vec(),
            target_candidates,
            normalized_proximity,
            zone,
            channel,
            drives_in_series: state.drives_in_current_series(),
            remaining_advance_mirim: state
                .possession()
                .series_state()
                .remaining_mirins_to_target(),
            down: state.possession().down(),
            is_bonus_phase: state.possession().is_bonus_phase(),
            is_true_artrine,
            is_home_offense: context.is_home_offense,
            pitch_length_mirim,
            best_target_weight,
            long_launch_target_weight,
            offensive_gravity: offensive_gravity.multiplier(),
            team_advantage,
            pass_protection_advantage: pass_phase.pass_duel_outcome.outcome().net_advantage(),
            game_state_pressure,
            risk_profile,
            duel_context,
            passing_range,
            decision_emphasis: context.decision_emphasis,
            defense_pressing_multiplier,
            offense_tempo_value,
            state_advanced_mirins: state.possession().series_state().advanced_mirins(),
        }
    }
}