use crate::MatchSession;
pub struct GameSimulationSession {
    session: MatchSession,
}

impl GameSimulationSession {
    pub fn session(&self) -> &MatchSession {
        &self.session
    }

    pub fn session_mut(&mut self) -> &mut MatchSession {
        &mut self.session
    }
}
