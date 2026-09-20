use crate::ai::epv::DynamicEpvModel;
use crate::ai::evaluators::DecisionEvaluationContext;
use crate::artrine::available_decision_kinds;
use crate::artrine::event_translation::translate_artrine_decision_made;
use crate::match_decision::event_translation::create_envelope;
pub use crate::open_play::carrier_sampler::carrier_decision_steepness;
use crate::open_play::carrier_sampler::sample_carrier_decision_from_table;
use crate::world_state::step::down_resolution::context::DownResolutionContext;
use arlo_domain::{ArtrineDecisionKind, AttributeKey};
use arlo_events::{EventSink, MatchClockInstant};
use rand::Rng;
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_decision<R: Rng + ?Sized>(
    ctx: &DownResolutionContext<'_>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    seq: u64,
    clock_inst: MatchClockInstant,
    rng: &mut R,
    sink: &mut impl EventSink,
) -> ArtrineDecisionKind {
    let available_kinds = available_decision_kinds(
        ctx.drives_in_series,
        ctx.possession_advanced_mirins,
        ctx.is_last_down,
        ctx.is_bonus_phase,
    );

    let epv_model = DynamicEpvModel::with_difficulty(
        ctx.offensive_gravity,
        ctx.scoring_difficulty,
    );
    let current_epv = epv_model.calculate_epa(
        ctx.normalized_proximity,
        ctx.down,
        ctx.remaining_advance_mirim,
        ctx.drives_in_series,
        ctx.is_bonus_phase,
        &ctx.scoring_regime,
    );

    let carrier_ctx = ctx.build_carrier_context();
    let sit_ctx = ctx.build_situation_context();

    let eval_ctx = DecisionEvaluationContext::new(
        carrier_ctx,
        sit_ctx,
        attribute_keys,
        epv_model,
        current_epv,
        ctx.risk_profile,
        ctx.scoring_regime,
    );

    let mut utilities: SmallVec<[(ArtrineDecisionKind, f64); 5]> =
        SmallVec::with_capacity(available_kinds.len());
    for kind in available_kinds {
        let config = crate::ai::evaluators::get_action_config(kind);
        utilities.push((kind, crate::ai::evaluators::evaluate_action_utility(&eval_ctx, &config)));
    }

    let result = sample_carrier_decision_from_table(
        ctx.carrier,
        &ctx.carrier_table,
        &utilities,
        &ctx.carrier_fatigue,
        &ctx.carrier_impulse,
        rng,
    );

    if ctx.is_true_artrine {
        let decision_event = translate_artrine_decision_made(
            ctx.carrier.id(),
            result.chosen(),
            ctx.down as u32,
            result.chosen_probability(),
        );
        sink.record(create_envelope(seq, clock_inst, decision_event));
    }

    result.chosen()
}
