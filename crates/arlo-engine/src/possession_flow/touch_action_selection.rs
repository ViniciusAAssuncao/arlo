use crate::ai::epv::DynamicEpvModel;
use crate::ai::evaluators::DecisionEvaluationContext;
use crate::artrine::available_decision_kinds;
use crate::open_play::carrier_sampler::{sample_carrier_decision_from_table, CarrierDecisionResult};
use crate::world_state::step::down_resolution::context::{DownStaticContext, TouchDynamicContext};
use arlo_domain::{ArtrineDecisionKind, AttributeKey};
use rand::Rng;
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

pub fn select_touch_action<R: Rng + ?Sized>(
    static_ctx: &DownStaticContext<'_>,
    touch_ctx: &TouchDynamicContext<'_>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    rng: &mut R,
) -> CarrierDecisionResult {
    let available_kinds = available_decision_kinds(
        touch_ctx.drives_in_series,
        touch_ctx.possession_advanced_mirins,
        touch_ctx.is_last_down,
        touch_ctx.is_bonus_phase,
        touch_ctx.down,
        touch_ctx.normalized_proximity,
        touch_ctx.is_true_artrine,
    );

    let epv_model = DynamicEpvModel::with_difficulty(
        static_ctx.offensive_gravity,
        static_ctx.scoring_difficulty,
    );
    let current_epv = epv_model.calculate_epa(
        touch_ctx.normalized_proximity,
        touch_ctx.down,
        touch_ctx.remaining_advance_mirim,
        touch_ctx.drives_in_series,
        touch_ctx.is_bonus_phase,
        &static_ctx.scoring_regime,
    );

    let eval_ctx = DecisionEvaluationContext::new(
        touch_ctx.build_carrier_context(),
        touch_ctx.build_situation_context(static_ctx),
        attribute_keys,
        epv_model,
        current_epv,
        touch_ctx.risk_profile,
        static_ctx.scoring_regime,
    );

    let mut utilities: SmallVec<[(ArtrineDecisionKind, f64); 5]> = SmallVec::with_capacity(available_kinds.len());
    for kind in available_kinds {
        let config = crate::ai::evaluators::get_action_config(kind);
        utilities.push((kind, crate::ai::evaluators::evaluate_action_utility(&eval_ctx, &config)));
    }

    sample_carrier_decision_from_table(
        touch_ctx.carrier,
        &touch_ctx.carrier_table,
        &utilities,
        &touch_ctx.carrier_fatigue,
        &touch_ctx.carrier_impulse,
        rng,
    )
}