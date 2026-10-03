use super::AuditResult;
use arlo_engine::{MatchInput, MatchPhase, MatchState, StepOutcome};
use arlo_events::{EventSink, InMemorySink, MatchEvent, MatchEventEnvelope};
use arlo_manager_control::{ManagerDecisionInbox, TacticalRealignmentIntent};
use arlo_match_runner::{
    build_default_aggregator_registry, resolve_segment_with_registry, DualEventSink,
    DEFAULT_MAX_SEGMENTS,
};
use arlo_persistence::models::MatchTacticalRealignmentRow;
use arlo_persistence::repositories::match_tactical_realignments;
use arlo_tactics::PlayCall;
use sqlx::SqlitePool;
use uuid::Uuid;

pub async fn audit(
    pool: &SqlitePool,
    input: &MatchInput,
    play_calls: &[PlayCall],
) -> AuditResult<()> {
    let first = simulate(input, play_calls)?;
    let second = simulate(input, play_calls)?;
    if first != second {
        return Err("Commanded realignment is not reproducible".into());
    }
    let match_id: Option<String> = sqlx::query_scalar("SELECT id FROM matches ORDER BY id LIMIT 1")
        .fetch_optional(pool)
        .await?;
    let Some(match_id) = match_id else {
        println!("Commanded realignment: 2 swaps, preserved evidence, identical replay; no persisted match available for storage audit");
        return Ok(());
    };
    let match_id = Uuid::parse_str(&match_id)?;
    let first_sequence: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(sequence_number), 0) + 1 FROM match_tactical_realignments WHERE match_id = ?").bind(match_id.to_string()).fetch_one(pool).await?;
    let mut tx = pool.begin().await?;
    for (index, envelope) in first.iter().enumerate() {
        let MatchEvent::TacticalRealignmentMade(event) = envelope.event() else {
            continue;
        };
        let row = MatchTacticalRealignmentRow {
            match_id: match_id.to_string(),
            sequence_number: first_sequence + index as i64,
            team_id: event.team_id().to_string(),
            period: envelope.clock().period() as i32,
            seconds_in_period: envelope.clock().seconds_in_period(),
            total_elapsed_seconds: envelope.clock().total_elapsed_seconds(),
            assignments: event.assignments().clone(),
        };
        match_tactical_realignments::insert(&mut tx, &row).await?;
    }
    tx.commit().await?;
    let rows = match_tactical_realignments::list_by_match_id(pool, match_id).await?;
    let rows: Vec<_> = rows
        .into_iter()
        .filter(|row| row.sequence_number >= first_sequence)
        .collect();
    if rows.len() != 2 {
        return Err("Unexpected persisted realignment count".into());
    }
    for (row, envelope) in rows.iter().zip(&first) {
        let MatchEvent::TacticalRealignmentMade(event) = envelope.event() else {
            unreachable!();
        };
        if row.assignments != *event.assignments()
            || row.total_elapsed_seconds != envelope.clock().total_elapsed_seconds()
        {
            return Err("Persisted realignment differs from original event".into());
        }
    }
    let timeline =
        arlo_controller::controllers::r#match::match_timeline_controller::get_match_timeline(
            pool, match_id,
        )
        .await?;
    let count = timeline.iter().filter(|event| matches!(event, arlo_controller::dto::r#match::MatchTimelineEventDto::TacticalRealignment(row) if row.sequence_number >= first_sequence as u64)).count();
    if count != 2 {
        return Err("Realignment is missing from match timeline".into());
    }
    sqlx::query(
        "DELETE FROM match_tactical_realignments WHERE match_id = ? AND sequence_number >= ?",
    )
    .bind(match_id.to_string())
    .bind(first_sequence)
    .execute(pool)
    .await?;
    println!("Commanded realignment: 2 swaps, preserved evidence, identical replay, persistence and timeline verified");
    Ok(())
}

fn simulate(input: &MatchInput, play_calls: &[PlayCall]) -> AuditResult<Vec<MatchEventEnvelope>> {
    let mut state = MatchState::new(input);
    let mut registry = build_default_aggregator_registry(input)?;
    let mut raw = InMemorySink::new();
    let inbox = ManagerDecisionInbox::new();
    for _ in 0..DEFAULT_MAX_SEGMENTS {
        if state.phase() == MatchPhase::Stopped
            && state.clock().total_elapsed_seconds() >= 1800.0
            && state.clock().seconds_in_period() < state.clock().period_limit_seconds()
            && state.pending_forced_substitutions().is_empty()
            && state.pending_injury_decisions().is_empty()
        {
            let team_id = input.home().team_id();
            let players = state.home().active_player_ids();
            if players.len() < 2 {
                return Err("Insufficient active players for commanded realignment".into());
            }
            let intent = TacticalRealignmentIntent::new(players[0], players[1]);
            let original = state.home().active_player_ids().to_vec();
            let mut events = Vec::new();
            for _ in 0..2 {
                inbox.submit_tactical_realignment(team_id, intent);
                let result = resolve_segment_with_registry(
                    input, &mut state, &inbox, play_calls, &registry,
                )?;
                let (resolved, _) = result.into_parts();
                if resolved.len() != 1 {
                    return Err("Command did not resolve atomically".into());
                }
                for event in resolved {
                    let before = super::realignment::capture(&registry, event.event())?;
                    let MatchEvent::TacticalRealignmentMade(realignment) = event.event() else {
                        return Err("Commanded realignment was not dispatched".into());
                    };
                    let realignment = realignment.clone();
                    registry.handle_envelope(&event);
                    super::realignment::verify(&registry, &state, &realignment, &before)?;
                    events.push(event);
                }
                if state.home().active_player_ids() != original {
                    return Err("Realignment changed active membership".into());
                }
            }
            return Ok(events);
        }
        let result =
            resolve_segment_with_registry(input, &mut state, &inbox, play_calls, &registry)?;
        let (events, outcome) = result.into_parts();
        let mut sink = DualEventSink::new(&mut raw, &mut registry);
        for event in events {
            if let MatchEvent::PlayInvalidated(value) = event.event() {
                sink.invalidate_play(value.first_sequence(), value.last_sequence());
            }
            sink.record(event);
        }
        if !matches!(outcome, StepOutcome::Resolved) {
            return Err("Match cannot reach commanded realignment stoppage".into());
        }
    }
    Err("Segment limit exceeded during commanded realignment audit".into())
}
