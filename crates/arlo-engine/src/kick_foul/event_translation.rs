use crate::kick_foul::pending::KickFoulPending;
use arlo_domain::KickFoulDecisionKind;
use arlo_events::{KickFoulAwarded, KickFoulDecisionMade};
use uuid::Uuid;

pub fn translate_kick_foul_awarded(
    pending: &KickFoulPending,
    offending_team_id: Uuid,
) -> KickFoulAwarded {
    KickFoulAwarded::new(
        pending.awarded_team_id(),
        offending_team_id,
        pending.scoring_tier(),
        pending.spot_x_mirim(),
        pending.spot_y_mirim(),
    )
}

pub fn translate_kick_foul_decision_made(
    taker_id: Uuid,
    decision: KickFoulDecisionKind,
) -> KickFoulDecisionMade {
    KickFoulDecisionMade::new(taker_id, decision)
}
