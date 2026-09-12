use crate::world_state::match_state::foul_review::FoulReviewRecord;
use crate::world_state::match_state::state::MatchState;
use uuid::Uuid;

impl MatchState {
    pub fn last_reviewable_foul(&self) -> Option<&(Uuid, FoulReviewRecord)> {
        self.foul_review.entry()
    }

    pub fn set_last_reviewable_foul(&mut self, team_id: Uuid, record: FoulReviewRecord) {
        self.foul_review.set(team_id, record);
    }

    pub fn clear_last_reviewable_foul(&mut self) {
        self.foul_review.clear();
    }
}