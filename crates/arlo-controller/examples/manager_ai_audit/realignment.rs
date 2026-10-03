use super::AuditResult;
use arlo_analytics::{PlayerPerformanceAggregator, PlayerPerformanceSnapshot};
use arlo_engine::MatchState;
use arlo_events::{MatchEvent, TacticalAssignment, TacticalPlanActivated, TacticalRealignmentMade};
use arlo_stats::AggregatorRegistry;

pub fn capture(
    registry: &AggregatorRegistry,
    event: &MatchEvent,
) -> AuditResult<Vec<PlayerPerformanceSnapshot>> {
    let assignments: &[TacticalAssignment] = match event {
        MatchEvent::TacticalRealignmentMade(event) => event.assignments(),
        MatchEvent::TacticalPlanActivated(event) => &event.assignments,
        _ => return Ok(Vec::new()),
    };
    let analytics = registry
        .get::<PlayerPerformanceAggregator>()
        .ok_or("Missing analytics")?;
    assignments
        .iter()
        .map(|assignment| {
            analytics
                .current_player_snapshot(&assignment.player_id)
                .ok_or_else(|| "Missing realigned player snapshot".into())
        })
        .collect()
}

pub fn verify(
    registry: &AggregatorRegistry,
    state: &MatchState,
    event: &TacticalRealignmentMade,
    before: &[PlayerPerformanceSnapshot],
) -> AuditResult<()> {
    verify_assignments(
        registry,
        state,
        event.team_id(),
        event.assignments(),
        before,
        true,
    )
}

pub fn verify_plan(
    registry: &AggregatorRegistry,
    state: &MatchState,
    event: &TacticalPlanActivated,
    before: &[PlayerPerformanceSnapshot],
) -> AuditResult<()> {
    verify_assignments(
        registry,
        state,
        event.team_id,
        &event.assignments,
        before,
        false,
    )
}

fn verify_assignments(
    registry: &AggregatorRegistry,
    state: &MatchState,
    team_id: uuid::Uuid,
    assignments: &[TacticalAssignment],
    before: &[PlayerPerformanceSnapshot],
    require_active: bool,
) -> AuditResult<()> {
    let analytics = registry
        .get::<PlayerPerformanceAggregator>()
        .ok_or("Missing analytics")?;
    let context = analytics.context().ok_or("Missing analysis context")?;
    let team = if state.home().team_id() == team_id {
        state.home()
    } else {
        state.away()
    };
    if assignments.len() != before.len() {
        return Err("Incomplete tactical snapshot".into());
    }
    for (assignment, old) in assignments.iter().zip(before) {
        let new = analytics
            .current_player_snapshot(&assignment.player_id)
            .ok_or("Missing realigned snapshot")?;
        if (require_active && !team.active_player_ids().contains(&assignment.player_id))
            || context.active_player_for_slot(team_id, assignment.formation_slot_index)
                != Some(assignment.player_id)
            || new.offensive_position() != assignment.offensive_position
            || new.defensive_position() != assignment.defensive_position
            || new.slot_role() != assignment.slot_role
            || new.seconds_played() != old.seconds_played()
            || new.effective_opportunities() != old.effective_opportunities()
            || new.breakdown() != old.breakdown()
            || new.performance_rating() != old.performance_rating()
            || new.confidence() != old.confidence()
            || analytics
                .players()
                .get(&assignment.player_id)
                .is_none_or(|player| {
                    player.is_active() != team.active_player_ids().contains(&assignment.player_id)
                })
        {
            return Err(
                "Realignment changed participation evidence or failed to update assignments".into(),
            );
        }
    }
    Ok(())
}
