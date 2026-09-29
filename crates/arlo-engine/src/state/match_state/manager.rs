use super::MatchState;
use crate::error::{EngineError, EngineResult};
use crate::state::MatchPhase;
use uuid::Uuid;

impl MatchState {
    pub fn challenges_used(&self, team_id: Uuid) -> EngineResult<u32> {
        Ok(self.team(team_id)?.challenges_used())
    }

    pub fn player_settling_factor(&self, player_id: Uuid) -> f64 {
        self.entered_at.get(&player_id).map_or(1.0, |entered| {
            (0.92 + (self.clock.total_elapsed_seconds() - entered).max(0.0) / 3000.0).min(1.0)
        })
    }

    pub(crate) fn record_challenge(&mut self, team_id: Uuid, limit: u32) -> EngineResult<u32> {
        let used = self.team(team_id)?.challenges_used();
        if used >= limit {
            return Err(EngineError::InvalidTransition("no challenges remain".into()));
        }
        self.team_mut(team_id)?.record_challenge();
        Ok(limit - used - 1)
    }

    pub(crate) fn substitute_voluntarily(
        &mut self,
        team_id: Uuid,
        outgoing: Uuid,
        incoming: Uuid,
    ) -> EngineResult<()> {
        if self.phase != MatchPhase::Stopped || self.clock.seconds_in_period() >= self.clock.period_limit_seconds() {
            return Err(EngineError::InvalidTransition("voluntary substitution requires a stoppage before the next play".into()));
        }
        let elapsed = self.clock.total_elapsed_seconds();
        let team = self.team(team_id)?;
        if outgoing == incoming || !team.active_player_ids().contains(&outgoing)
            || !team.reserve_player_ids().contains(&incoming)
            || team.injured_player_ids().contains(&incoming) {
            return Err(EngineError::InvalidInput("invalid voluntary substitution".into()));
        }
        self.team_mut(team_id)?.substitute_voluntarily(outgoing, incoming, elapsed);
        self.energy_participants.insert(incoming);
        self.entered_at.insert(incoming, elapsed);
        Ok(())
    }
}
