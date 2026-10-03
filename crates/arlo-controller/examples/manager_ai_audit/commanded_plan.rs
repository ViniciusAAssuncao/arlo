use super::AuditResult;
use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink, MatchEvent, MatchEventEnvelope};
use arlo_manager_control::{ManagerDecisionInbox, PreparedPlanIntent};
use arlo_match_runner::{
    build_default_aggregator_registry, resolve_segment_with_registry, DualEventSink,
    DEFAULT_MAX_SEGMENTS,
};
use arlo_tactics::PlayCall;

pub async fn audit(
    pool: &sqlx::SqlitePool,
    input: &MatchInput,
    calls: &[PlayCall],
) -> AuditResult<()> {
    let first = simulate(input, calls)?;
    if first != simulate(input, calls)? {
        return Err("Commanded plans are not reproducible".into());
    }
    super::plan_storage::audit(pool, input, &first).await?;
    println!("Commanded plans: {} activations, preserved membership, mandatory roles, series, clock, energy and evidence; identical replay", first.len());
    Ok(())
}

fn simulate(input: &MatchInput, calls: &[PlayCall]) -> AuditResult<Vec<MatchEventEnvelope>> {
    let mut state = MatchState::new(input);
    let mut registry = build_default_aggregator_registry(input)?;
    let mut raw = InMemorySink::new();
    let inbox = ManagerDecisionInbox::new();
    for _ in 0..DEFAULT_MAX_SEGMENTS {
        if state.phase() == MatchPhase::Stopped
            && state.clock().total_elapsed_seconds() >= 600.0
            && state.clock().seconds_in_period() < state.clock().period_limit_seconds()
            && state.pending_forced_substitutions().is_empty()
            && state.pending_injury_decisions().is_empty()
        {
            let mut events = Vec::new();
            for team in [input.home(), input.away()] {
                for plan in team
                    .prepared_plans()
                    .iter()
                    .skip(1)
                    .chain(team.prepared_plans().first())
                {
                    if state.team_state(team.team_id())?.active_plan_id() == Some(plan.id) {
                        continue;
                    }
                    let before_state = state.clone();
                    inbox.submit_prepared_plan(team.team_id(), PreparedPlanIntent::new(plan.id));
                    let (resolved, _) =
                        resolve_segment_with_registry(input, &mut state, &inbox, calls, &registry)?
                            .into_parts();
                    if resolved.len() != 1 {
                        return Err("Plan did not resolve atomically".into());
                    }
                    for event in resolved {
                        let before = super::realignment::capture(&registry, event.event())?;
                        let MatchEvent::TacticalPlanActivated(plan_event) = event.event() else {
                            return Err("Plan was not dispatched".into());
                        };
                        registry.handle_envelope(&event);
                        super::realignment::verify_plan(&registry, &state, plan_event, &before)?;
                        verify_state(&before_state, &state, team.team_id())?;
                        let actual = state.team_state(team.team_id())?;
                        if actual.formation(team).id() != plan_event.formation_id
                            || actual.active_plan_id() != Some(plan.id)
                        {
                            return Err("Engine did not activate the requested layout".into());
                        }
                        events.push(event);
                    }
                }
            }
            return Ok(events);
        }
        let (events, outcome) =
            resolve_segment_with_registry(input, &mut state, &inbox, calls, &registry)?
                .into_parts();
        let mut sink = DualEventSink::new(&mut raw, &mut registry);
        for event in events {
            if let MatchEvent::PlayInvalidated(value) = event.event() {
                sink.invalidate_play(value.first_sequence(), value.last_sequence());
            }
            sink.record(event);
        }
        if !matches!(outcome, StepOutcome::Resolved) {
            return Err("Cannot reach prepared plan stoppage".into());
        }
    }
    Err("Commanded plan segment limit exceeded".into())
}

fn verify_state(before: &MatchState, after: &MatchState, id: uuid::Uuid) -> AuditResult<()> {
    let old = before.team_state(id)?;
    let new = after.team_state(id)?;
    if old.active_player_ids() != new.active_player_ids()
        || old.reserve_player_ids() != new.reserve_player_ids()
        || old.artrine_id() != new.artrine_id()
        || old.passer_id() != new.passer_id()
        || old.drive_eligible() != new.drive_eligible()
        || old.drive_progress() != new.drive_progress()
        || old.drives_in_series() != new.drives_in_series()
        || old.score() != new.score()
        || before.clock() != after.clock()
        || before.series() != after.series()
        || before.possession() != after.possession()
        || old.active_player_ids().iter().any(|player| {
            before.player_energy(*player) != after.player_energy(*player)
                || before.player_last_entered_at(*player) != after.player_last_entered_at(*player)
        })
    {
        return Err(
            "Plan changed membership, specialists, clock, series, score or physical state".into(),
        );
    }
    Ok(())
}
