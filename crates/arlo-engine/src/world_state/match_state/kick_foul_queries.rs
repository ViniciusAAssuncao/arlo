use crate::kick_foul::KickFoulPending;
use crate::world_state::match_state::state::MatchState;

impl MatchState {
    pub fn kick_foul_pending(&self) -> Option<&KickFoulPending> {
        self.kick_foul.pending()
    }

    pub fn set_kick_foul_pending(&mut self, pending: KickFoulPending) {
        self.kick_foul.set(pending);
    }

    pub fn take_kick_foul_pending(&mut self) -> Option<KickFoulPending> {
        self.kick_foul.take()
    }

    pub fn clear_kick_foul_pending(&mut self) {
        self.kick_foul.clear();
    }
}
