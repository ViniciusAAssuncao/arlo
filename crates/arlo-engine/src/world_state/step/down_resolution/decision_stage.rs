use crate::artrine::event_translation::translate_artrine_decision_made;
use crate::match_decision::event_translation::create_envelope;
use crate::possession_flow::touch_action_selection::select_touch_action;
use crate::world_state::step::down_resolution::context::{DownStaticContext, TouchDynamicContext};
use arlo_domain::{ArtrineDecisionKind, AttributeKey};
use arlo_events::{EventSink, MatchClockInstant};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_decision<R: Rng + ?Sized>(
    static_ctx: &DownStaticContext<'_>,
    touch_ctx: &TouchDynamicContext<'_>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    seq: u64,
    clock_inst: MatchClockInstant,
    rng: &mut R,
    sink: &mut impl EventSink,
) -> ArtrineDecisionKind {
    let result = select_touch_action(static_ctx, touch_ctx, attribute_keys, rng);

    if touch_ctx.is_true_artrine {
        let decision_event = translate_artrine_decision_made(
            touch_ctx.carrier.id(),
            result.chosen(),
            touch_ctx.down as u32,
            result.chosen_probability(),
        );
        sink.record(create_envelope(seq, clock_inst, decision_event));
    }

    result.chosen()
}
