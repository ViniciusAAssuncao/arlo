pub mod restart_outcome;
pub mod score_outcome;
pub mod turnover_outcome;

pub use restart_outcome::apply_restart_outcome;
pub use score_outcome::apply_score_outcome;
pub use turnover_outcome::apply_turnover_outcome;

use crate::kick_foul::pending::KickFoulPending;
use crate::kick_foul::resolution::resolve_kick_foul;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use rand::Rng;

pub fn resolve_and_apply_kick_foul<R: Rng + ?Sized>(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    pending: &KickFoulPending,
    rng: &mut R,
) {
    let outcome_res = resolve_kick_foul(publisher.state(), pending, rng);

    if let Ok(outcome) = outcome_res {
        publisher.emit_kick_foul_decision_made(outcome.taker_id, outcome.decision);

        publisher.emit_duel_events(&outcome.duels, outcome.taker_id);

        if outcome.scoring_decision.is_scored() {
            apply_score_outcome(
                publisher,
                pending.awarded_team_id(),
                &outcome.scoring_decision,
            );
        } else if let Some(restart) = &outcome.restart {
            if restart.caught {
                apply_restart_outcome(publisher, restart.reception_point);
            } else {
                apply_turnover_outcome(publisher, restart.reception_point);
            }
        } else {
            apply_turnover_outcome(publisher, pending.spot());
        }
    }

    publisher.state_mut().clear_kick_foul_pending();
}
