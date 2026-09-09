use crate::artrine::{
    available_decision_kinds, resolve_artrine_decision_with_context_and_impulse_and_free_path,
    translate_artrine_decision_made,
};
use crate::match_decision::event_translation::create_envelope;
use crate::open_play::{sample_carrier_decision, CarrierDecisionEvaluator};
use crate::physical::FatigueState;
use crate::rng::RngStream;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player, Position as DomainPosition, SlotRole};
use arlo_events::EventSink;
use arlo_math::units::MIRIM_TO_METERS;
use uuid::Uuid;

pub fn select_carrier_decision<F, S: EventSink>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &OpenPlayLoopState,
    current_carrier: &Player,
    fatigue_lookup: &F,
    sink: &mut S,
) -> ArtrineDecisionKind
where
    F: Fn(&Uuid) -> FatigueState,
{
    let pitch = *state.pitch();
    let attribute_keys = state.attribute_keys().clone();
    let is_bonus_phase = state.possession().is_bonus_phase();
    let is_last_down = state.possession().series_state().is_last_down();
    let is_true_artrine = current_carrier.id() == pass_phase.artrine.id();

    let total_drives = state.drives_in_current_series() + loop_state.accumulated_drives_recorded;
    let total_advance = state.possession().series_state().advanced_mirins()
        + loop_state.accumulated_mirins_advanced;
    let remaining_downs = state.possession().series_state().remaining_downs();

    let carrier_fatigue = fatigue_lookup(&current_carrier.id());
    let carrier_impulse = state.impulse_for(&current_carrier.id());

    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let passing_range = offense_instructions.in_possession().passing_range();

    if is_true_artrine && loop_state.loop_iteration == 1 {
        let seq_decision = state.next_sequence();
        let mut decision_rng = state
            .rng_provider()
            .indexed_rng_for(RngStream::ArtrineDecision, seq_decision);

        let decision_result = resolve_artrine_decision_with_context_and_impulse_and_free_path(
            current_carrier,
            &attribute_keys,
            iter_ctx.normalized_proximity,
            total_drives,
            remaining_downs,
            pass_phase.pass_duel_outcome.outcome().net_advantage(),
            is_last_down,
            is_bonus_phase,
            total_advance,
            iter_ctx.best_available_target_weight,
            iter_ctx.long_launch_target_weight,
            loop_state.current_carrier_pos,
            iter_ctx.next_artro_pos,
            iter_ctx.pitch_control_ahead,
            pitch.length_mirim(),
            iter_ctx.offensive_gravity_mult,
            passing_range,
            iter_ctx.risk_profile,
            iter_ctx.game_state_pressure,
            context.decision_emphasis,
            &carrier_fatigue,
            &carrier_impulse,
            iter_ctx.expected_free_path_mirim,
            &mut decision_rng,
        );

        let decision_event = translate_artrine_decision_made(
            current_carrier.id(),
            decision_result.chosen(),
            pass_phase.down_number,
            decision_result.chosen_probability(),
        );
        let seq = state.next_sequence();
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, decision_event));

        decision_result.chosen()
    } else {
        let carrier_pos_domain = context
            .offense_pos_index
            .get(&current_carrier.id())
            .copied()
            .unwrap_or_else(|| {
                current_carrier
                    .positions()
                    .first()
                    .map(|pp| pp.position())
                    .unwrap_or(DomainPosition::CenterOffense)
            });

        let carrier_role = context
            .offense_role_index
            .get(&current_carrier.id())
            .copied()
            .unwrap_or(SlotRole::Standard);

        let carrier_instructions = context
            .offense_instructions_index
            .get(&current_carrier.id())
            .copied()
            .unwrap_or_default();

        let available_kinds = available_decision_kinds(
            total_drives,
            total_advance,
            is_last_down,
            is_bonus_phase,
        );

        let down_idx = if is_last_down {
            4
        } else {
            (5 - remaining_downs).clamp(1, 4)
        };

        let distance_to_next_artro_mirim = (iter_ctx.next_artro_pos.raw().0
            - loop_state.current_carrier_pos.raw().0)
            .abs()
            / MIRIM_TO_METERS;

        let utilities = CarrierDecisionEvaluator::evaluate_action_utilities(
            current_carrier,
            carrier_pos_domain,
            carrier_role,
            carrier_instructions,
            &attribute_keys,
            &available_kinds,
            iter_ctx.normalized_proximity,
            total_drives,
            down_idx,
            (10.0 - total_advance).max(0.0),
            pass_phase.pass_duel_outcome.outcome().net_advantage(),
            iter_ctx.best_available_target_weight,
            iter_ctx.long_launch_target_weight,
            loop_state.current_carrier_pos,
            iter_ctx.pitch_control_ahead,
            distance_to_next_artro_mirim,
            pitch.length_mirim(),
            pitch.width_mirim(),
            iter_ctx.offensive_gravity_mult,
            passing_range,
            iter_ctx.risk_profile,
            iter_ctx.game_state_pressure,
            context.decision_emphasis,
            &carrier_fatigue,
            is_true_artrine,
            iter_ctx.expected_free_path_mirim,
        );

        let seq_decision = state.next_sequence();
        let mut decision_rng = state
            .rng_provider()
            .indexed_rng_for(RngStream::ArtrineDecision, seq_decision);

        let res = sample_carrier_decision(
            current_carrier,
            &attribute_keys,
            &utilities,
            &carrier_fatigue,
            &carrier_impulse,
            &mut decision_rng,
        );
        res.chosen()
    }
}