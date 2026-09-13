use crate::world_state::match_state::forced_substitution_tracker::ForcedSubstitutionTracker;
use crate::world_state::match_state::state::MatchState;
use uuid::Uuid;

impl MatchState {
    pub fn forced_substitution_tracker(&self) -> &ForcedSubstitutionTracker {
        &self.forced_substitution_tracker
    }

    pub fn pending_forced_substitutions_for(&self, team_id: Uuid) -> &[Uuid] {
        let is_home = team_id == self.teams.home_team_id();
        self.forced_substitution_tracker.pending(is_home)
    }

    pub fn set_pending_forced_substitutions(&mut self, team_id: Uuid, ids: Vec<Uuid>) {
        let is_home = team_id == self.teams.home_team_id();
        self.forced_substitution_tracker.set_pending(is_home, ids);
    }

    pub fn clear_pending_forced_substitutions(&mut self, team_id: Uuid) {
        let is_home = team_id == self.teams.home_team_id();
        self.forced_substitution_tracker.clear(is_home);
    }
}