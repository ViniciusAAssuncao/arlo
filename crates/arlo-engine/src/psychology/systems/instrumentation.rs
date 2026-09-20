use crate::match_decision::scoring::ScoringDecision;
use crate::open_play::CarrierDecisionResult;
use crate::psychology::systems::events::{ImpulseEvent, ImpulseEventKind};
use crate::resolution::outcome::DuelOutcome;

pub fn duel_impulse_events(outcome: &DuelOutcome) -> (ImpulseEvent, ImpulseEvent) {
    let win_p = outcome.win_probability().value().clamp(0.0001, 0.9999);
    let net_adv = outcome.net_advantage();
    let epv_delta = (net_adv * 0.1).abs();

    let (att_kind, att_p) = if outcome.attacker_won() {
        (ImpulseEventKind::DuelWon, win_p)
    } else {
        (ImpulseEventKind::DuelLost, 1.0 - win_p)
    };

    let (def_kind, def_p) = if outcome.attacker_won() {
        (ImpulseEventKind::DuelLost, win_p)
    } else {
        (ImpulseEventKind::DuelWon, 1.0 - win_p)
    };

    let att_surprisal = -att_p.ln();
    let def_surprisal = -def_p.ln();

    (
        ImpulseEvent::new(att_kind, att_surprisal, epv_delta, true),
        ImpulseEvent::new(def_kind, def_surprisal, epv_delta, true),
    )
}

pub fn scoring_impulse_events(
    decision: &ScoringDecision,
    win_probability: f64,
) -> Option<(ImpulseEvent, ImpulseEvent)> {
    let p = win_probability.clamp(0.0001, 0.9999);
    let points = decision.points() as f64;

    if decision.is_scored() {
        let att_surprisal = -p.ln();
        let def_surprisal = -(1.0 - p).ln();
        Some((
            ImpulseEvent::new(ImpulseEventKind::ScoreFor, att_surprisal, points, true),
            ImpulseEvent::new(ImpulseEventKind::ScoreAgainst, def_surprisal, points, true),
        ))
    } else if matches!(decision, ScoringDecision::Missed { .. }) {
        let att_surprisal = -(1.0 - p).ln();
        let def_surprisal = -p.ln();
        Some((
            ImpulseEvent::new(ImpulseEventKind::DuelLost, att_surprisal, points.max(1.0), true),
            ImpulseEvent::new(ImpulseEventKind::DuelWon, def_surprisal, points.max(1.0), true),
        ))
    } else {
        None
    }
}

pub fn decision_impulse_event(result: &CarrierDecisionResult) -> ImpulseEvent {
    let p = result.chosen_probability().value().clamp(0.0001, 0.9999);
    let surprisal = -p.ln();
    ImpulseEvent::new(
        ImpulseEventKind::BigPlayCompleted,
        surprisal * 0.40,
        0.5,
        true,
    )
}
