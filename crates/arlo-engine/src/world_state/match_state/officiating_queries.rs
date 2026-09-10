use crate::officiating::ReviewableCall;
use crate::world_state::match_state::state::MatchState;
use uuid::Uuid;

impl MatchState {
    pub fn last_reviewable_call(&self) -> Option<&(Uuid, ReviewableCall)> {
        self.officiating.last_reviewable_call()
    }

    pub fn set_last_reviewable_call(&mut self, team_id: Uuid, call: ReviewableCall) {
        self.officiating.set_last_reviewable_call(team_id, call);
    }

    pub fn clear_last_reviewable_call(&mut self) {
        self.officiating.clear_last_reviewable_call();
    }
}
