use super::MatchState;
use crate::error::{EngineError, EngineResult};
use crate::input::TeamInput;
use crate::state::MatchPhase;
use arlo_tactics::TeamInstructions;
use uuid::Uuid;

impl MatchState {
    pub fn team_instructions(&self, team: &TeamInput) -> TeamInstructions {
        let active_id = if team.team_id() == self.home.team_id() {
            self.home.active_tactical_profile_id()
        } else {
            self.away.active_tactical_profile_id()
        };
        team.tactical_profiles().find(|profile| profile.id() == active_id)
            .map_or(*team.tactics().instructions(), |profile| *profile.instructions())
    }

    pub(crate) fn activate_tactical_profile(&mut self, team_id: Uuid, profile_id: Uuid) -> EngineResult<()> {
        if self.phase != MatchPhase::Stopped || self.clock.seconds_in_period() >= self.clock.period_limit_seconds() {
            return Err(EngineError::InvalidTransition("tactical switch requires a stoppage".into()));
        }
        let elapsed = self.clock.total_elapsed_seconds();
        let team = self.team_mut(team_id)?;
        if team.active_tactical_profile_id() == profile_id || team.tactical_switches() >= 3
            || team.last_tactical_switch_at().is_some_and(|last| elapsed - last < 1800.0) {
            return Err(EngineError::InvalidTransition("tactical switch is unavailable".into()));
        }
        team.activate_tactical_profile(profile_id, elapsed);
        Ok(())
    }

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
