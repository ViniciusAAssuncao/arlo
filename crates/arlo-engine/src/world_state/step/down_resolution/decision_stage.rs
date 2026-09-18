use crate::ai::epv::DynamicEpvModel;
use crate::ai::evaluators::DecisionEvaluationContext;
use crate::artrine::available_decision_kinds;
use crate::artrine::event_translation::translate_artrine_decision_made;
use crate::match_decision::event_translation::create_envelope;
pub use crate::open_play::carrier_sampler::carrier_decision_steepness;
use crate::open_play::carrier_sampler::sample_carrier_decision_from_table;
use crate::open_play::CarrierDecisionEvaluator;
use crate::world_state::match_state::MatchState;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use arlo_domain::ArtrineDecisionKind;
use arlo_events::EventSink;
use rand::Rng;

pub fn resolve_decision<R: Rng + ?Sized>(
    ctx: &DownResolutionContext<'_>,
    state: &mut MatchState,
    rng: &mut R,
    sink: &mut impl EventSink,
) -> ArtrineDecisionKind {
    let available_kinds = available_decision_kinds(
        ctx.drives_in_series,
        ctx.state_advanced_mirins,
        ctx.normalized_proximity,
        ctx.down >= 4,
        ctx.is_bonus_phase,
    );

    let epv_model = DynamicEpvModel::new(ctx.offensive_gravity);
    let current_epv = epv_model.calculate_epa(
        ctx.normalized_proximity,
        ctx.down,
        ctx.remaining_advance_mirim,
        ctx.drives_in_series,
    );

    let eval_ctx = DecisionEvaluationContext::new(
        ctx.carrier,
        &ctx.carrier_table,
        ctx.carrier_pos_domain,
        ctx.carrier_role,
        ctx.carrier_instructions,
        ctx.carrier_fatigue,
        state.attribute_keys(),
        epv_model,
        current_epv,
        ctx.normalized_proximity,
        ctx.drives_in_series,
        ctx.down,
        ctx.remaining_advance_mirim,
        ctx.pass_protection_advantage,
        ctx.best_target_weight,
        ctx.long_launch_target_weight,
        ctx.team_advantage,
        ctx.channel,
        ctx.offensive_gravity,
        ctx.passing_range,
        ctx.risk_profile,
        ctx.game_state_pressure,
        ctx.decision_emphasis,
        ctx.is_true_artrine,
    );

    let utilities =
        CarrierDecisionEvaluator::evaluate_action_utilities(&eval_ctx, &available_kinds);

    let result = sample_carrier_decision_from_table(
        ctx.carrier,
        &ctx.carrier_table,
        &utilities,
        &ctx.carrier_fatigue,
        &ctx.carrier_impulse,
        rng,
    );

    if ctx.is_true_artrine {
        let seq = state.next_sequence();
        let decision_event = translate_artrine_decision_made(
            ctx.carrier.id(),
            result.chosen(),
            ctx.down as u32,
            result.chosen_probability(),
        );
        let clock_inst = state.clock().to_instant();
        sink.record(create_envelope(seq, clock_inst, decision_event));
    }

    result.chosen()
}