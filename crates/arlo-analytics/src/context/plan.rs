use super::{AnalyticsError, AnalyticsResult, MatchAnalysisContext};
use arlo_events::TacticalPlanActivated;
use std::collections::HashSet;

impl MatchAnalysisContext {
    pub fn handle_plan_event(&mut self, event: &TacticalPlanActivated) -> AnalyticsResult<()> {
        let occupants: HashSet<_> = self
            .active_slot_players
            .iter()
            .filter(|((team_id, _), _)| *team_id == event.team_id)
            .map(|(_, player)| *player)
            .collect();
        let next: HashSet<_> = event
            .assignments
            .iter()
            .map(|assignment| assignment.player_id)
            .collect();
        let slots: HashSet<_> = event
            .assignments
            .iter()
            .map(|assignment| assignment.formation_slot_index)
            .collect();
        if occupants != next
            || next.len() != event.assignments.len()
            || slots.len() != next.len()
            || slots.iter().any(|slot| *slot >= 14)
            || event.assignments.iter().any(|assignment| {
                self.assignments
                    .get(&assignment.player_id)
                    .is_none_or(|current| current.team_id != event.team_id)
            })
        {
            return Err(AnalyticsError::InvalidData(
                "prepared plan must preserve current team occupants".into(),
            ));
        }
        self.active_slot_players
            .retain(|(team_id, _), _| *team_id != event.team_id);
        for assignment in &event.assignments {
            let current = self
                .assignments
                .get_mut(&assignment.player_id)
                .expect("validated assignment");
            current.slot_index = assignment.formation_slot_index;
            current.offensive_position = assignment.offensive_position;
            current.defensive_position = assignment.defensive_position;
            current.slot_role = assignment.slot_role;
            self.active_slot_players.insert(
                (event.team_id, assignment.formation_slot_index),
                assignment.player_id,
            );
        }
        Ok(())
    }
}
