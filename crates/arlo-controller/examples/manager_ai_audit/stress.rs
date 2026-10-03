use super::AuditResult;
use arlo_analytics::PlayerPerformanceAggregator;
use arlo_domain::{Position, SlotRole};
use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink, MatchEvent, MatchEventEnvelope};
use arlo_manager_control::{
    ManagerDecisionInbox, PreparedPlanIntent, SubstitutionIntent, TacticalRealignmentIntent,
};
use arlo_match_runner::{
    build_default_aggregator_registry, resolve_segment_with_registry, DualEventSink,
    DEFAULT_MAX_SEGMENTS,
};
use arlo_tactics::PlayCall;

pub fn audit(input: &MatchInput, calls: &[PlayCall]) -> AuditResult<()> {
    let mut covered = [false; 3];
    for index in 0..64 {
        let seeded = super::scenario_input::rebuild_seed(
            input,
            input.home().clone(),
            input.seed() ^ (index + 1u64).wrapping_mul(0x517cc1b727220a95),
        )?;
        let (events, coverage) = simulate(&seeded, calls)?;
        if covered.iter().zip(coverage).any(|(old, new)| !old && new) {
            if (events.clone(), coverage) != simulate(&seeded, calls)? {
                return Err("Tactical lifecycle stress is not reproducible".into());
            }
        }
        for (value, found) in covered.iter_mut().zip(coverage) {
            *value |= found;
        }
        if covered.iter().all(|value| *value) {
            println!("Lifecycle stress: {} seeds; rested reentry, invalidation after plan/realignment and shorthanded plan activation; identical replay", index + 1);
            return Ok(());
        }
    }
    Err(format!("Lifecycle stress coverage incomplete: {covered:?}").into())
}

fn simulate(
    input: &MatchInput,
    calls: &[PlayCall],
) -> AuditResult<(Vec<MatchEventEnvelope>, [bool; 3])> {
    let mut state = MatchState::new(input);
    let mut registry = build_default_aggregator_registry(input)?;
    let mut raw = InMemorySink::new();
    let inbox = ManagerDecisionInbox::new();
    let team = input.home();
    let plan = team
        .prepared_plans()
        .iter()
        .find(|plan| {
            plan.layout
                .lineup
                .assignments()
                .iter()
                .any(|slot| slot.slot_role() == SlotRole::FalseArtrine)
        })
        .ok_or("No fraud-capable prepared layout")?;
    let mut activated = false;
    let mut realigned = false;
    let mut withdrawn = None;
    let mut covered = [false; 3];
    for _ in 0..DEFAULT_MAX_SEGMENTS {
        let mut return_before = None;
        let mut shorthanded_before = None;
        if state.phase() == MatchPhase::Stopped
            && state.clock().seconds_in_period() < state.clock().period_limit_seconds()
            && state.pending_forced_substitutions().is_empty()
            && state.pending_injury_decisions().is_empty()
        {
            let elapsed = state.clock().total_elapsed_seconds();
            let current = state.home();
            if !activated && elapsed >= 600.0 {
                inbox.submit_prepared_plan(team.team_id(), PreparedPlanIntent::new(plan.id));
            } else if activated && !realigned && elapsed >= 900.0 {
                let players: Vec<_> = current
                    .lineup(team)
                    .assignments()
                    .iter()
                    .filter(|slot| !mandatory(slot.position()))
                    .map(|slot| current.slot_player_id(slot.player_id()))
                    .filter(|id| current.active_player_ids().contains(id))
                    .collect();
                if players.len() >= 2 {
                    inbox.submit_tactical_realignment(
                        team.team_id(),
                        TacticalRealignmentIntent::new(players[0], players[1]),
                    );
                }
            } else if withdrawn.is_none() && elapsed >= 1200.0 {
                let outgoing = current
                    .lineup(team)
                    .assignments()
                    .iter()
                    .filter(|slot| {
                        !mandatory(slot.position()) && slot.slot_role() != SlotRole::FalseArtrine
                    })
                    .map(|slot| current.slot_player_id(slot.player_id()))
                    .find(|id| current.active_player_ids().contains(id));
                let incoming = current
                    .reserve_player_ids()
                    .iter()
                    .copied()
                    .find(|id| current.is_available_reserve(*id));
                if let (Some(outgoing), Some(incoming)) = (outgoing, incoming) {
                    withdrawn = Some((outgoing, incoming, elapsed, state.player_energy(outgoing)));
                    inbox.submit_substitution(
                        team.team_id(),
                        SubstitutionIntent::new(outgoing, incoming),
                    );
                }
            } else if let Some((outgoing, incoming, exit, energy)) = withdrawn {
                if !covered[0]
                    && elapsed >= exit + 1200.0
                    && current.is_available_reserve(outgoing)
                    && current.active_player_ids().contains(&incoming)
                {
                    let snapshot = registry
                        .get::<PlayerPerformanceAggregator>()
                        .ok_or("Missing analytics")?
                        .current_player_snapshot(&outgoing)
                        .ok_or("Missing rested player")?;
                    return_before = Some((outgoing, energy, snapshot));
                    inbox.submit_substitution(
                        team.team_id(),
                        SubstitutionIntent::new(incoming, outgoing),
                    );
                }
            }
            if activated && !covered[2] && current.active_player_ids().len() < 14 {
                if let Some(target) = team
                    .prepared_plans()
                    .iter()
                    .find(|target| current.active_plan_id() != Some(target.id))
                {
                    shorthanded_before = Some((
                        current.active_player_ids().to_vec(),
                        current.drive_eligible(),
                    ));
                    inbox.submit_prepared_plan(team.team_id(), PreparedPlanIntent::new(target.id));
                }
            }
        }
        let (events, outcome) =
            resolve_segment_with_registry(input, &mut state, &inbox, calls, &registry)?
                .into_parts();
        for event in events {
            let before = super::realignment::capture(&registry, event.event())?;
            match event.event() {
                MatchEvent::TacticalPlanActivated(value) if value.team_id == team.team_id() => {
                    activated = true;
                    if let Some((active, drives)) = &shorthanded_before {
                        if state.home().active_player_ids() != active
                            || state.home().drive_eligible() != *drives
                        {
                            return Err(
                                "Plan restored shorthanded membership or Drive eligibility".into(),
                            );
                        }
                        covered[2] = true;
                    }
                }
                MatchEvent::TacticalRealignmentMade(value) if value.team_id() == team.team_id() => {
                    realigned = true
                }
                MatchEvent::PlayInvalidated(_) => covered[1] |= activated && realigned,
                MatchEvent::SubstitutionMade(value) => {
                    if let Some((outgoing, energy, _)) = &return_before {
                        if value.player_in() == *outgoing {
                            if state.player_energy(*outgoing) <= *energy {
                                return Err("Rested reentry recovered no energy".into());
                            }
                            covered[0] = true;
                        }
                    }
                }
                _ => {}
            }
            let event_type = event.event().clone();
            let mut sink = DualEventSink::new(&mut raw, &mut registry);
            if let MatchEvent::PlayInvalidated(value) = event.event() {
                sink.invalidate_play(value.first_sequence(), value.last_sequence());
            }
            sink.record(event);
            drop(sink);
            match &event_type {
                MatchEvent::TacticalPlanActivated(value) => {
                    super::realignment::verify_plan(&registry, &state, value, &before)?
                }
                MatchEvent::TacticalRealignmentMade(value) => {
                    super::realignment::verify(&registry, &state, value, &before)?
                }
                MatchEvent::SubstitutionMade(value) => {
                    if let Some((id, _, old)) = &return_before {
                        if value.player_in() == *id {
                            let new = registry
                                .get::<PlayerPerformanceAggregator>()
                                .unwrap()
                                .current_player_snapshot(id)
                                .unwrap();
                            if new.seconds_played() != old.seconds_played()
                                || new.effective_opportunities() != old.effective_opportunities()
                                || new.breakdown() != old.breakdown()
                            {
                                return Err("Reentry discarded existing evidence".into());
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        super::roster::verify(input, &state, &registry)?;
        if state.phase() == MatchPhase::Finished {
            return Ok((raw.events().to_vec(), covered));
        }
        if !matches!(outcome, StepOutcome::Resolved) {
            return Err("Stress match needs a human decision".into());
        }
    }
    Err("Lifecycle stress segment limit exceeded".into())
}

fn mandatory(position: Position) -> bool {
    matches!(
        position,
        Position::Goalguard | Position::Artrine | Position::Passer
    )
}
