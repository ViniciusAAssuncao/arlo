use crate::officiating::reviewable_call::ReviewableCall;
use rand::Rng;

pub fn resolve_true_ruling<R: Rng + ?Sized>(call: &ReviewableCall, rng: &mut R) -> bool {
    let call_was_incorrect = call.ambiguity().sample(rng);
    if call_was_incorrect {
        !call.on_field_favors_offense()
    } else {
        call.on_field_favors_offense()
    }
}
