use crate::manager_ai::ManagerDecisionKind;
use crate::world_state::match_state::state::MatchState;
use uuid::Uuid;

impl MatchState {
    pub fn is_decision_ready(
        &self,
        team_id: Uuid,
        kind: ManagerDecisionKind,
        min_interval: f64,
    ) -> bool {
        let is_home = team_id == self.teams.home_team_id();
        let current_time = self.clock().to_instant().total_elapsed_seconds();
        self.decision_cooldown
            .is_ready(is_home, kind, current_time, min_interval)
    }

    pub fn mark_decision_triggered(&mut self, team_id: Uuid, kind: ManagerDecisionKind) {
        let is_home = team_id == self.teams.home_team_id();
        let current_time = self.clock().to_instant().total_elapsed_seconds();
        self.decision_cooldown
            .mark_triggered(is_home, kind, current_time);
    }
}