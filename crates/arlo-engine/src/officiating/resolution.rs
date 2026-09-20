use crate::officiating::reviewable_call::ReviewableCall;
use crate::resolution::{resolve_contest, ContestRequest, DuelContext, DuelKind};
use rand::Rng;

pub fn resolve_true_ruling<R: Rng + ?Sized>(call: &ReviewableCall, rng: &mut R) -> bool {
    let ambiguity_rating = call.ambiguity().value() * 10.0;
    let certainty_rating = (1.0 - call.ambiguity().value()) * 10.0;
    let context = DuelContext::neutral();
    let req = ContestRequest::for_contest(
        DuelKind::PassProtection,
        ambiguity_rating,
        certainty_rating,
        &context,
    )
    .with_slope(0.25);
    let call_was_incorrect = resolve_contest(req, rng).attacker_won();
    if call_was_incorrect {
        !call.on_field_favors_offense()
    } else {
        call.on_field_favors_offense()
    }
}
