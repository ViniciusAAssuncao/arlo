use super::AuditResult;
use arlo_analytics::PlayerPerformanceAggregator;
use arlo_engine::{MatchInput, MatchState};
use arlo_stats::AggregatorRegistry;
use std::collections::HashSet;

pub fn verify(
    input: &MatchInput,
    state: &MatchState,
    registry: &AggregatorRegistry,
) -> AuditResult<()> {
    let analytics = registry
        .get::<PlayerPerformanceAggregator>()
        .ok_or("Missing analytics")?;
    let context = analytics.context().ok_or("Missing analysis context")?;
    for team in [input.home(), input.away()] {
        let current = state.team_state(team.team_id())?;
        let actual: HashSet<_> = current.active_player_ids().iter().copied().collect();
        let tracked: HashSet<_> = analytics
            .players()
            .values()
            .filter(|player| player.team_id() == team.team_id() && player.is_active())
            .map(|player| player.player_id())
            .collect();
        if actual != tracked || actual.len() > 14 {
            return Err(format!(
                "Roster disagreement for {}: engine={actual:?}, analytics={tracked:?}",
                team.team_id()
            )
            .into());
        }
        for slot in current.lineup(team).assignments() {
            let id = current.slot_player_id(slot.player_id());
            let formation = &current.formation(team).slots()[slot.formation_slot_index()];
            let assignment = context
                .assignment(&id)
                .ok_or("Missing occupant assignment")?;
            if assignment.slot_index() != slot.formation_slot_index()
                || assignment.offensive_position() != formation.offensive_position()
                || assignment.defensive_position() != formation.defensive_position()
                || assignment.slot_role() != slot.slot_role()
            {
                return Err("Engine and analytics disagree on effective roles".into());
            }
        }
        for player in team.roster() {
            let energy = state.player_energy(player.id());
            if !energy.is_finite() || !(0.0..=1.0).contains(&energy) {
                return Err("Invalid physical energy".into());
            }
        }
    }
    Ok(())
}
