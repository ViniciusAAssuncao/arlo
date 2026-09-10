use crate::match_decision::scoring::{ScoringDecision, ScoringOpportunity};
use crate::open_play::CarrierDecisionResult;
use crate::possession::snapshot::PossessionSnapshot;
use crate::possession::transition::PlayOutcome;
use crate::psychology::systems::event_bus::DispatchedImpulseEvent;
use crate::psychology::systems::events::{ImpulseEvent, ImpulseEventKind};
use crate::resolution::outcome::DuelOutcome;
use uuid::Uuid;

pub fn create_duel_impulse_events(
    outcome: &DuelOutcome,
    attacker_id: Uuid,
    defender_id: Uuid,
) -> [DispatchedImpulseEvent; 2] {
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

    let att_event = ImpulseEvent::new(att_kind, att_surprisal, epv_delta, true);
    let def_event = ImpulseEvent::new(def_kind, def_surprisal, epv_delta, true);

    [
        DispatchedImpulseEvent::new(attacker_id, att_event),
        DispatchedImpulseEvent::new(defender_id, def_event),
    ]
}

pub fn instrument_duel_outcome(
    outcome: &DuelOutcome,
    attacker_id: Uuid,
    defender_id: Uuid,
) -> [DispatchedImpulseEvent; 2] {
    create_duel_impulse_events(outcome, attacker_id, defender_id)
}

pub fn create_scoring_impulse_events(
    decision: &ScoringDecision,
    _opportunity: ScoringOpportunity,
    attacker_id: Uuid,
    defender_id: Uuid,
    win_probability: f64,
) -> Vec<DispatchedImpulseEvent> {
    let p = win_probability.clamp(0.0001, 0.9999);
    let points = decision.points() as f64;

    if decision.is_scored() {
        let att_surprisal = -p.ln();
        let def_surprisal = -(1.0 - p).ln();

        let att_event = ImpulseEvent::new(ImpulseEventKind::ScoreFor, att_surprisal, points, true);
        let def_event =
            ImpulseEvent::new(ImpulseEventKind::ScoreAgainst, def_surprisal, points, true);

        vec![
            DispatchedImpulseEvent::new(attacker_id, att_event),
            DispatchedImpulseEvent::new(defender_id, def_event),
        ]
    } else {
        let att_surprisal = -(1.0 - p).ln();
        let def_surprisal = -p.ln();

        let att_event = ImpulseEvent::new(
            ImpulseEventKind::DuelLost,
            att_surprisal,
            points.max(1.0),
            true,
        );
        let def_event = ImpulseEvent::new(
            ImpulseEventKind::DuelWon,
            def_surprisal,
            points.max(1.0),
            true,
        );

        vec![
            DispatchedImpulseEvent::new(attacker_id, att_event),
            DispatchedImpulseEvent::new(defender_id, def_event),
        ]
    }
}

pub fn instrument_scoring_attempt(
    decision: &ScoringDecision,
    opportunity: ScoringOpportunity,
    attacker_id: Uuid,
    defender_id: Uuid,
    win_probability: f64,
) -> Vec<DispatchedImpulseEvent> {
    create_scoring_impulse_events(
        decision,
        opportunity,
        attacker_id,
        defender_id,
        win_probability,
    )
}

pub fn create_transition_impulse_events(
    outcome: &PlayOutcome,
    previous_snapshot: &PossessionSnapshot,
    new_snapshot: &PossessionSnapshot,
) -> Vec<DispatchedImpulseEvent> {
    let mut events = Vec::new();
    let prev_offense = previous_snapshot.role().offense();
    let prev_defense = previous_snapshot.role().defense();

    if let Some(new_offense) = outcome.turnover {
        let lost_event = ImpulseEvent::new(ImpulseEventKind::TurnoverCommitted, 1.20, 2.5, true);
        let won_event = ImpulseEvent::new(ImpulseEventKind::TurnoverWon, 1.20, 2.5, true);
        events.push(DispatchedImpulseEvent::new(prev_offense, lost_event));
        events.push(DispatchedImpulseEvent::new(new_offense, won_event));
    } else if previous_snapshot.series_state().should_turnover_on_downs() {
        let failure_event = ImpulseEvent::new(ImpulseEventKind::SeriesFailure, 0.90, 2.0, true);
        let success_event = ImpulseEvent::new(ImpulseEventKind::SeriesSuccess, 0.90, 2.0, true);
        events.push(DispatchedImpulseEvent::new(prev_offense, failure_event));
        events.push(DispatchedImpulseEvent::new(prev_defense, success_event));
    } else if new_snapshot.series_state().down() == 1
        && previous_snapshot.series_state().down() > 1
        && !outcome.score_occurred
    {
        let success_event = ImpulseEvent::new(ImpulseEventKind::SeriesSuccess, 0.70, 1.5, false);
        let failure_event = ImpulseEvent::new(ImpulseEventKind::SeriesFailure, 0.70, 1.5, false);
        events.push(DispatchedImpulseEvent::new(prev_offense, success_event));
        events.push(DispatchedImpulseEvent::new(prev_defense, failure_event));
    }

    events
}

pub fn instrument_transition(
    outcome: &PlayOutcome,
    previous_snapshot: &PossessionSnapshot,
    new_snapshot: &PossessionSnapshot,
) -> Vec<DispatchedImpulseEvent> {
    create_transition_impulse_events(outcome, previous_snapshot, new_snapshot)
}

pub fn create_artrine_decision_impulse_event(
    artrine_id: Uuid,
    result: &CarrierDecisionResult,
) -> DispatchedImpulseEvent {
    let p = result.chosen_probability().value().clamp(0.0001, 0.9999);
    let surprisal = -p.ln();
    let event = ImpulseEvent::new(
        ImpulseEventKind::BigPlayCompleted,
        surprisal * 0.40,
        0.5,
        true,
    );
    DispatchedImpulseEvent::new(artrine_id, event)
}

pub fn instrument_artrine_decision(
    artrine_id: Uuid,
    result: &CarrierDecisionResult,
) -> DispatchedImpulseEvent {
    create_artrine_decision_impulse_event(artrine_id, result)
}
