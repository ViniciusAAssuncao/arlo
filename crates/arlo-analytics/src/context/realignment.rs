use super::{AnalyticsError, AnalyticsResult, MatchAnalysisContext};
use arlo_events::TacticalRealignmentMade;

impl MatchAnalysisContext {
    pub fn handle_realignment_event(
        &mut self,
        event: &TacticalRealignmentMade,
    ) -> AnalyticsResult<()> {
        let [first, second] = event.assignments();
        if first.player_id == second.player_id
            || first.formation_slot_index == second.formation_slot_index
        {
            return Err(AnalyticsError::InvalidData(
                "realignment requires distinct players and slots".into(),
            ));
        }
        for (assignment, previous) in [(first, second), (second, first)] {
            let current = self
                .assignments
                .get(&assignment.player_id)
                .ok_or(AnalyticsError::PlayerNotFound(assignment.player_id))?;
            let destination = self
                .assignments
                .get(&previous.player_id)
                .ok_or(AnalyticsError::PlayerNotFound(previous.player_id))?;
            if current.team_id != event.team_id()
                || self.active_player_for_slot(event.team_id(), current.slot_index)
                    != Some(assignment.player_id)
                || destination.team_id != event.team_id()
                || destination.slot_index != assignment.formation_slot_index
                || destination.offensive_position != assignment.offensive_position
                || destination.defensive_position != assignment.defensive_position
                || destination.slot_role != assignment.slot_role
            {
                return Err(AnalyticsError::InvalidData(
                    "realignment must permute current team assignments".into(),
                ));
            }
        }
        for assignment in event.assignments() {
            let current = self
                .assignments
                .get_mut(&assignment.player_id)
                .expect("validated assignment");
            current.slot_index = assignment.formation_slot_index;
            current.offensive_position = assignment.offensive_position;
            current.defensive_position = assignment.defensive_position;
            current.slot_role = assignment.slot_role;
            self.active_slot_players.insert(
                (event.team_id(), assignment.formation_slot_index),
                assignment.player_id,
            );
        }
        Ok(())
    }
}
