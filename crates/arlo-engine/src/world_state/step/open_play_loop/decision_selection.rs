use crate::ai::epv::DynamicEpvModel;
use crate::ai::evaluators::DecisionEvaluationContext;
use crate::artrine::available_decision_kinds;
use crate::artrine::event_translation::translate_artrine_decision_made;
use crate::match_decision::event_translation::create_envelope;
use crate::open_play::carrier_sampler::sample_carrier_decision_from_table;
use crate::open_play::CarrierDecisionEvaluator;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::world_state::cta_pass::PassPhaseResult;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::open_play_loop::action_context::OpenPlayIterationContext;
use crate::world_state::step::open_play_loop::loop_state::OpenPlayLoopState;
use crate::world_state::step::setup::CallToActionContext;
use arlo_domain::{ArtrineDecisionKind, Player, Position, SlotRole};
use arlo_events::EventSink;
use rand::Rng;

pub fn select_carrier_decision<R: Rng + ?Sized>(
    state: &mut MatchState,
    context: &CallToActionContext,
    iter_ctx: &OpenPlayIterationContext<'_>,
    pass_phase: &PassPhaseResult<'_>,
    loop_state: &OpenPlayLoopState,
    current_carrier: &Player,
    rng: &mut R,
    sink: &mut impl EventSink,
) -> ArtrineDecisionKind {
    let total_drives = state.drives_in_current_series() + loop_state.accumulated_drives_recorded;
    let total_advance = state.possession().series_state().advanced_mirins()
        + loop_state.accumulated_mirins_advanced;
    let is_last_down = state.possession().series_state().is_last_down();
    let is_bonus_phase = state.possession().is_bonus_phase();

    let available_kinds =
        available_decision_kinds(total_drives, total_advance, is_last_down, is_bonus_phase);

    let epv_model = DynamicEpvModel::new(iter_ctx.offensive_gravity_mult);
    let remaining_advance = (state
        .possession()
        .series_state()
        .remaining_mirins_to_target()
        - loop_state.accumulated_mirins_advanced)
        .max(0.0);
    let current_epv = epv_model.calculate_epa(
        iter_ctx.normalized_proximity,
        state.possession().down(),
        remaining_advance,
        total_drives,
    );

    let carrier_pos_domain = context
        .offense_pos_index
        .get(&current_carrier.id())
        .copied()
        .unwrap_or_else(|| {
            current_carrier
                .positions()
                .first()
                .map(|pp| pp.position())
                .unwrap_or(Position::CenterOffense)
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

    let carrier_physical_state = state.fatigue_lookup().get(&current_carrier.id());

    let distance_to_next_artro_mirim =
        calculate_distance_mirim(loop_state.current_carrier_pos, iter_ctx.next_artro_pos);

    let offense_instructions = *state.instructions_for_team(context.offense_team_id);
    let passing_range = offense_instructions.in_possession().passing_range();

    let is_true_artrine = loop_state.current_carrier_id == pass_phase.artrine.id();

    let carrier_table = *state.attribute_table_for(&current_carrier.id());

    let eval_ctx = DecisionEvaluationContext::new(
        current_carrier,
        &carrier_table,
        carrier_pos_domain,
        carrier_role,
        carrier_instructions,
        carrier_physical_state,
        state.attribute_keys(),
        epv_model,
        current_epv,
        iter_ctx.normalized_proximity,
        total_drives,
        state.possession().down(),
        remaining_advance,
        pass_phase.pass_duel_outcome.outcome().net_advantage(),
        iter_ctx.best_available_target_weight,
        iter_ctx.long_launch_target_weight,
        iter_ctx.pitch_control_ahead,
        distance_to_next_artro_mirim,
        state.pitch().length_mirim(),
        state.pitch().width_mirim(),
        loop_state.current_carrier_pos,
        iter_ctx.offensive_gravity_mult,
        passing_range,
        iter_ctx.risk_profile,
        iter_ctx.game_state_pressure,
        context.decision_emphasis,
        is_true_artrine,
        iter_ctx.expected_free_path_mirim,
    );

    let utilities =
        CarrierDecisionEvaluator::evaluate_action_utilities(&eval_ctx, &available_kinds);

    let carrier_impulse = state.impulse_for(&current_carrier.id());

    let result = sample_carrier_decision_from_table(
        current_carrier,
        &carrier_table,
        &utilities,
        &carrier_physical_state,
        &carrier_impulse,
        rng,
    );

    if is_true_artrine {
        let seq = state.next_sequence();
        let decision_event = translate_artrine_decision_made(
            current_carrier.id(),
            result.chosen(),
            state.possession().down() as u32,
            result.chosen_probability(),
        );
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, decision_event));

        crate::psychology::systems::instrumentation::instrument_artrine_decision(
            current_carrier.id(),
            &result,
        );
    }

    result.chosen()
}
