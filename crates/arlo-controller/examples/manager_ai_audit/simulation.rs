use super::AuditResult;
use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink, MatchEvent};
use arlo_manager_control::ManagerDecisionInbox;
use arlo_match_runner::{
    build_default_aggregator_registry, manager_ai::assess_manager_decision,
    resolve_segment_with_registry, run_match_with_registry_and_play_calls, DualEventSink,
    DEFAULT_MAX_SEGMENTS,
};
use arlo_tactics::PlayCall;
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

#[derive(Debug, Default, serde::Serialize)]
pub struct MatchAudit {
    pub lifecycle: super::lifecycle::Lifecycle,
    pub home_score: u32,
    pub away_score: u32,
    pub assessments: usize,
    pub no_changes: usize,
    pub candidates: usize,
    pub substitutions: usize,
    pub realignments: usize,
    pub realignment_candidates: usize,
    pub plan_candidates: usize,
    pub plan_activations: usize,
    pub formation_changes: usize,
    pub reentries: usize,
    pub quick_returns: usize,
    pub quick_withdrawals: usize,
    pub invalidated_plays: usize,
    pub reasons: BTreeMap<String, usize>,
    pub reproducible: bool,
}

#[derive(Debug, Default, serde::Serialize)]
pub struct AuditTotals {
    pub lifecycle: super::lifecycle::Lifecycle,
    pub completed: usize,
    pub skipped: usize,
    pub assessments: usize,
    pub no_changes: usize,
    pub candidates: usize,
    pub substitutions: usize,
    pub realignments: usize,
    pub realignment_candidates: usize,
    pub plan_candidates: usize,
    pub plan_activations: usize,
    pub formation_changes: usize,
    pub reentries: usize,
    pub quick_returns: usize,
    pub quick_withdrawals: usize,
    pub invalidated_plays: usize,
    pub non_reproducible: usize,
    pub reasons: BTreeMap<String, usize>,
}

impl AuditTotals {
    pub fn include(&mut self, audit: &MatchAudit) {
        self.lifecycle.include(&audit.lifecycle);
        self.completed += 1;
        self.assessments += audit.assessments;
        self.no_changes += audit.no_changes;
        self.candidates += audit.candidates;
        self.substitutions += audit.substitutions;
        self.realignments += audit.realignments;
        self.realignment_candidates += audit.realignment_candidates;
        self.plan_candidates += audit.plan_candidates;
        self.plan_activations += audit.plan_activations;
        self.formation_changes += audit.formation_changes;
        self.reentries += audit.reentries;
        self.quick_returns += audit.quick_returns;
        self.quick_withdrawals += audit.quick_withdrawals;
        self.invalidated_plays += audit.invalidated_plays;
        self.non_reproducible += usize::from(!audit.reproducible);
        for (reason, count) in &audit.reasons {
            *self.reasons.entry(reason.clone()).or_default() += count;
        }
    }
}

pub fn audit_match(input: &MatchInput, play_calls: &[PlayCall]) -> AuditResult<MatchAudit> {
    let mut state = MatchState::new(input);
    let mut registry = build_default_aggregator_registry(input)?;
    let mut raw = InMemorySink::new();
    let inbox = ManagerDecisionInbox::new();
    let mut audit = MatchAudit::default();
    let mut segments = 0;
    let mut formations = HashMap::from([
        (input.home().team_id(), input.home().formation().id()),
        (input.away().team_id(), input.away().formation().id()),
    ]);
    while state.phase() != MatchPhase::Finished {
        if segments >= DEFAULT_MAX_SEGMENTS {
            return Err("Segment limit exceeded".into());
        }
        if state.phase() == MatchPhase::Stopped {
            for team_id in [input.home().team_id(), input.away().team_id()] {
                if let Some(assessment) = assess_manager_decision(input, &state, &registry, team_id)
                {
                    audit.lifecycle.assessment(&state, team_id, &assessment)?;
                    audit.assessments += 1;
                    audit.candidates += assessment.candidates.len();
                    audit.realignment_candidates += assessment.realignments.len();
                    audit.plan_candidates += assessment.prepared_plans.len();
                    audit.no_changes += usize::from(assessment.selected().is_none());
                }
            }
        }
        let reserves = super::lifecycle::Lifecycle::reserves(&state);
        let result =
            resolve_segment_with_registry(input, &mut state, &inbox, play_calls, &registry)?;
        let (events, outcome) = result.into_parts();
        for event in events {
            audit.lifecycle.event(&state, &event)?;
            let before = super::realignment::capture(&registry, event.event())?;
            let plan = if let MatchEvent::TacticalPlanActivated(value) = event.event() {
                Some(value.clone())
            } else {
                None
            };
            let realignment = if let MatchEvent::TacticalRealignmentMade(value) = event.event() {
                Some(value.clone())
            } else {
                None
            };
            let mut sink = DualEventSink::new(&mut raw, &mut registry);
            if let MatchEvent::PlayInvalidated(invalidated) = event.event() {
                audit.invalidated_plays += 1;
                sink.invalidate_play(invalidated.first_sequence(), invalidated.last_sequence());
            }
            sink.record(event);
            drop(sink);
            if let Some(event) = plan {
                audit.plan_activations += 1;
                audit.formation_changes += usize::from(
                    formations.insert(event.team_id, event.formation_id)
                        != Some(event.formation_id),
                );
                super::realignment::verify_plan(&registry, &state, &event, &before)?;
            }
            if let Some(event) = realignment {
                audit.realignments += 1;
                super::realignment::verify(&registry, &state, &event, &before)?;
            }
        }
        if let Err(error) = super::roster::verify(input, &state, &registry) {
            eprintln!(
                "Roster failure at {:?}, injuries {:?}: {error}",
                state.clock(),
                state.home().injured_player_ids()
            );
            for event in raw.events().iter().rev().take(24).rev() {
                eprintln!("Trace: {event:?}");
            }
            return Err(error);
        }
        audit.lifecycle.recovery(input, &state, &reserves)?;
        if matches!(outcome, StepOutcome::AwaitingDecision(_)) {
            return Err("Match requires a human manager decision".into());
        }
        segments += 1;
    }
    audit.home_score = state.home().score().total_points();
    audit.away_score = state.away().score().total_points();
    collect_substitutions(&raw, &mut audit);
    let mut repeated_state = MatchState::new(input);
    let repeated = run_match_with_registry_and_play_calls(
        input,
        &mut repeated_state,
        build_default_aggregator_registry(input)?,
        play_calls,
        DEFAULT_MAX_SEGMENTS,
    )?;
    audit.reproducible = raw.events() == repeated.raw_sink().events();
    Ok(audit)
}

fn collect_substitutions(raw: &InMemorySink, audit: &mut MatchAudit) {
    let mut exits: HashMap<Uuid, f64> = HashMap::new();
    let mut entries: HashMap<Uuid, f64> = HashMap::new();
    for envelope in raw.events() {
        let MatchEvent::SubstitutionMade(event) = envelope.event() else {
            continue;
        };
        let elapsed = envelope.clock().total_elapsed_seconds();
        audit.substitutions += 1;
        *audit
            .reasons
            .entry(event.reason().as_str().into())
            .or_default() += 1;
        if let Some(exit) = exits.get(&event.player_in()) {
            audit.reentries += 1;
            audit.quick_returns += usize::from(elapsed - exit < 900.0);
        }
        if let Some(entry) = entries.get(&event.player_out()) {
            audit.quick_withdrawals += usize::from(elapsed - entry < 600.0);
        }
        exits.insert(event.player_out(), elapsed);
        entries.insert(event.player_in(), elapsed);
    }
}
