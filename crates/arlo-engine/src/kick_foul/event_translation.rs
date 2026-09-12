use crate::kick_foul::pending::KickFoulPending;
use arlo_domain::KickFoulDecisionKind;
use arlo_events::{KickFoulAwarded, KickFoulDecisionMade};
use arlo_math::units::MIRIM_TO_METERS;
use uuid::Uuid;

pub fn translate_kick_foul_awarded(
    pending: &KickFoulPending,
    offending_team_id: Uuid,
) -> KickFoulAwarded {
    let spot_x_mirim = pending.spot().raw().0 / MIRIM_TO_METERS;
    let spot_y_mirim = pending.spot().raw().1 / MIRIM_TO_METERS;
    KickFoulAwarded::new(
        pending.awarded_team_id(),
        offending_team_id,
        pending.scoring_tier(),
        spot_x_mirim,
        spot_y_mirim,
    )
}

pub fn translate_kick_foul_decision_made(
    taker_id: Uuid,
    decision: KickFoulDecisionKind,
) -> KickFoulDecisionMade {
    KickFoulDecisionMade::new(taker_id, decision)
}
