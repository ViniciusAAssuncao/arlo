use crate::ai::cognitive::RiskProfile;
use crate::ai::evaluators::{ContextCarrier, ContextSituation};
use crate::artrine::DriveAwardProfile;
use crate::attributes::PlayerAttributeTable;
use crate::match_decision::target_selection::{calculate_player_target_weight, ReceptionRole};
use crate::physical::PhysicalState;
use crate::playmaking::resolve_misdirection_logit_offset;
use crate::possession::locate_zone;
use crate::psychology::state::ImpulseState;
use crate::resolution::{ContestOrientation, DuelContext};
use crate::scoring_model::ScoringDifficultyProfile;
use crate::scoring_regime::ScoringRegimePolicy;
use crate::world_state::context_analyzer::{analyze_match_state, GameStatePressure};
use crate::world_state::match_state::MatchState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::sport_constants::AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM;
use arlo_domain::{ArtroPlacement, PitchZone, Player, Position, SlotRole};
use arlo_tactics::{DecisionEmphasis, PassingRange, PlayerInstructions, Tempo};
use std::collections::HashMap;
use uuid::Uuid;

pub struct DownStaticContext<'a> {
    pub offense_team_id: Uuid,
    pub defense_team_id: Uuid,
    pub offense_players: Vec<&'a Player>,
    pub defense_players: Vec<&'a Player>,
    pub is_home_offense: bool,
    pub pitch_length_mirim: f64,
    pub offensive_gravity: f64,
    pub team_advantage: f64,
    pub game_state_pressure: GameStatePressure,
    pub passing_range: PassingRange,
    pub decision_emphasis: DecisionEmphasis,
    pub offense_tempo: Tempo,
    pub scoring_regime: ScoringRegimePolicy,
    pub scoring_difficulty: ScoringDifficultyProfile,
    pub drive_award_profile: DriveAwardProfile,
}

impl<'a> DownStaticContext<'a> {
    pub fn build(
        state: &MatchState,
        context: &CallToActionContext,
        offense_players: Vec<&'a Player>,
        defense_players: Vec<&'a Player>,
    ) -> Self {
        let pitch_length_mirim = state.pitch().length_mirim();
        let offensive_gravity = state.offensive_gravity_for_team(context.offense_team_id);
        let offense_power = state.power_for_team(context.offense_team_id);
        let defense_power = state.power_for_team(context.defense_team_id);
        let team_advantage = offense_power.offensive_power() - defense_power.defensive_power();
        let game_state_pressure = analyze_match_state(state);
        let offense_instructions = *state.instructions_for_team(context.offense_team_id);
        let passing_range = offense_instructions.in_possession().passing_range();
        let offense_tempo = offense_instructions.in_possession().tempo();
        let scoring_regime = ScoringRegimePolicy::default();
        let scoring_difficulty = *state.tuning().scoring_difficulty();
        let drive_award_profile = *state.tuning().drive_award();

        Self {
            offense_team_id: context.offense_team_id,
            defense_team_id: context.defense_team_id,
            offense_players,
            defense_players,
            is_home_offense: context.is_home_offense,
            pitch_length_mirim,
            offensive_gravity: offensive_gravity.multiplier(),
            team_advantage,
            game_state_pressure,
            passing_range,
            decision_emphasis: context.decision_emphasis,
            offense_tempo,
            scoring_regime,
            scoring_difficulty,
            drive_award_profile,
        }
    }
}

pub struct TouchDynamicContext<'a> {
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
    pub target_candidates: Vec<&'a Player>,
    pub normalized_proximity: f64,
    pub zone: PitchZone,
    pub channel: ArtroPlacement,
    pub drives_in_series: u32,
    pub remaining_advance_mirim: f64,
    pub down: u8,
    pub is_bonus_phase: bool,
    pub is_last_down: bool,
    pub is_true_artrine: bool,
    pub best_target_weight: f64,
    pub long_launch_target_weight: f64,
    pub pass_protection_advantage: f64,
    pub risk_profile: RiskProfile,
    pub duel_context: DuelContext,
    pub state_advanced_mirins: f64,
    pub possession_advanced_mirins: f64,
}

impl<'a> TouchDynamicContext<'a> {
    pub fn build(
        state: &MatchState,
        static_ctx: &DownStaticContext<'a>,
        call_context: &CallToActionContext,
        carrier: &'a Player,
        current_x_mirim: f64,
        previous_advantage: f64,
        is_true_artrine: bool,
    ) -> Self {
        let normalized_proximity = if static_ctx.is_home_offense {
            (current_x_mirim / static_ctx.pitch_length_mirim.max(1.0)).clamp(0.0, 1.0)
        } else {
            ((static_ctx.pitch_length_mirim - current_x_mirim) / static_ctx.pitch_length_mirim.max(1.0)).clamp(0.0, 1.0)
        };

        let zone = locate_zone(
            normalized_proximity,
            static_ctx.pitch_length_mirim,
            AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM,
        );

        let channel = ArtroPlacement::Central;

        let target_candidates: Vec<&Player> = static_ctx.offense_players
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
                    &call_context.offense_pos_index,
                    &call_context.offense_instructions_index,
                    Some(&call_context.offense_role_index),
                    static_ctx.is_home_offense,
                    ReceptionRole::OpenPlayReceiver,
                    &empty_openness,
                    Some(&p_state),
                )
            })
            .fold(0.0_f64, f64::max);

        let long_launch_target_weight = best_target_weight;

        let carrier_table = *state.attribute_table_for(&carrier.id());
        let carrier_fatigue = state.fatigue_lookup().get(&carrier.id());
        let carrier_impulse = state.impulse_for(&carrier.id());

        let primary_defender = static_ctx.defense_players.first().copied().unwrap_or(carrier);
        let primary_defender_table = *state.attribute_table_for(&primary_defender.id());
        let primary_defender_fatigue = state.fatigue_lookup().get(&primary_defender.id());

        let carrier_pos_domain = call_context
            .offense_pos_index
            .get(&carrier.id())
            .copied()
            .unwrap_or(Position::CenterOffense);

        let carrier_role = call_context
            .offense_role_index
            .get(&carrier.id())
            .copied()
            .unwrap_or(SlotRole::Standard);

        let carrier_instructions = call_context
            .offense_instructions_index
            .get(&carrier.id())
            .copied()
            .unwrap_or_default();

        let primary_defender_pos_domain = call_context
            .defense_pos_index
            .get(&primary_defender.id())
            .copied()
            .unwrap_or(Position::Centerback);

        let risk_profile = RiskProfile::from_table_with_impulse(
            carrier,
            &carrier_table,
            &carrier_fatigue,
            &carrier_impulse,
        );

        let offense_instructions = *state.instructions_for_team(static_ctx.offense_team_id);
        let defense_instructions = *state.instructions_for_team(static_ctx.defense_team_id);

        let offense_physicality = offense_instructions.in_possession().physicality();
        let physicality_offset =
            crate::team_identity::physicality::offensive_contact_logit_offset(offense_physicality);
        let defense_aggression = defense_instructions.out_of_possession().aggression();
        let aggression_offset =
            crate::team_identity::aggression::duel_logit_offset(defense_aggression);
        let misdirection_offset = resolve_misdirection_logit_offset(
            call_context.active_play_call.as_ref(),
            &call_context.offense_route_index,
            &call_context.offense_lineup,
            state.tuning().home_advantage_profile.duel_logit(),
        );

        let duel_context = DuelContext::with_offsets(
            ContestOrientation::AttackerIsOffense,
            static_ctx.is_home_offense,
            state.tuning().home_advantage_profile.duel_logit(),
            aggression_offset,
            misdirection_offset,
            physicality_offset,
        );

        let drives_in_series = state.drives_in_current_series();
        let remaining_advance_mirim = state.possession().series_state().remaining_mirins_to_target();
        let down = state.possession().down();
        let is_bonus_phase = state.possession().is_bonus_phase();
        let is_last_down = state.possession().series_state().is_last_down();
        let state_advanced_mirins = state.possession().series_state().advanced_mirins();
        let possession_advanced_mirins = state.possession().possession_origin().total_advanced_mirins();

        Self {
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
            target_candidates,
            normalized_proximity,
            zone,
            channel,
            drives_in_series,
            remaining_advance_mirim,
            down,
            is_bonus_phase,
            is_last_down,
            is_true_artrine,
            best_target_weight,
            long_launch_target_weight,
            pass_protection_advantage: previous_advantage,
            risk_profile,
            duel_context,
            state_advanced_mirins,
            possession_advanced_mirins,
        }
    }

    pub fn build_carrier_context(&self) -> ContextCarrier<'_> {
        let prob_bounds = crate::ai::evaluators::DecisionEvaluationContext::calculate_probability_bounds(
            &self.carrier_table,
            &self.carrier_fatigue,
        );
        ContextCarrier {
            player: self.carrier,
            table: &self.carrier_table,
            position: self.carrier_pos_domain,
            role: self.carrier_role,
            instructions: self.carrier_instructions,
            physical_state: self.carrier_fatigue,
            probability_bounds: prob_bounds,
        }
    }

    pub fn build_situation_context(&self, static_ctx: &DownStaticContext) -> ContextSituation {
        let pitch_control = (0.50 + 0.04 * static_ctx.team_advantage - 0.08 * self.normalized_proximity
            + 0.04 * static_ctx.game_state_pressure.urgency_index())
        .clamp(0.15, 0.85);

        let expected_free_path =
            (pitch_control * (1.0 - self.normalized_proximity) * 35.0).clamp(1.5, 25.0);

        ContextSituation {
            drives_in_series: self.drives_in_series,
            down: self.down,
            remaining_advance_mirim: self.remaining_advance_mirim,
            pass_protection_net_advantage: self.pass_protection_advantage,
            target_quality: (self.best_target_weight - 8.0) / 10.0,
            long_launch_target_quality: (self.long_launch_target_weight - 8.0) / 10.0,
            team_advantage: static_ctx.team_advantage,
            channel: self.channel,
            pitch_control,
            expected_free_path,
            offensive_gravity: static_ctx.offensive_gravity,
            passing_range: static_ctx.passing_range,
            game_state_pressure: static_ctx.game_state_pressure,
            play_call_emphasis: static_ctx.decision_emphasis,
            is_true_artrine: self.is_true_artrine,
            is_bonus_phase: self.is_bonus_phase,
            normalized_proximity: self.normalized_proximity,
            pitch_length_mirim: static_ctx.pitch_length_mirim,
        }
    }
}
