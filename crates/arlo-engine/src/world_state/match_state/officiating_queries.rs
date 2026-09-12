use crate::officiating::ReviewableCall;
use crate::world_state::match_state::state::MatchState;
use uuid::Uuid;

impl MatchState {
    pub fn last_reviewable_call(&self) -> Option<&(Uuid, ReviewableCall)> {
        self.officiating.entry()
    }

    pub fn set_last_reviewable_call(&mut self, team_id: Uuid, call: ReviewableCall) {
        self.officiating.set(team_id, call);
    }

    pub fn clear_last_reviewable_call(&mut self) {
        self.officiating.clear();
    }
}