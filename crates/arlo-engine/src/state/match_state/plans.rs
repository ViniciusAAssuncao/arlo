use super::MatchState;
use crate::{EngineResult, TeamInput, TeamState};
use arlo_domain::Formation;
use arlo_tactics::{TacticalLayout, TacticalLineup};
use uuid::Uuid;

impl MatchState {
    pub fn team_state(&self, team_id: Uuid) -> EngineResult<&TeamState> {
        self.team(team_id)
    }

    pub fn team_lineup<'a>(&'a self, team: &'a TeamInput) -> &'a TacticalLineup {
        if team.team_id() == self.home.team_id() {
            self.home.lineup(team)
        } else {
            self.away.lineup(team)
        }
    }

    pub fn team_formation<'a>(&'a self, team: &'a TeamInput) -> &'a Formation {
        if team.team_id() == self.home.team_id() {
            self.home.formation(team)
        } else {
            self.away.formation(team)
        }
    }

    pub(crate) fn activate_prepared_plan(
        &mut self,
        team_id: Uuid,
        plan_id: Uuid,
        profile_id: Uuid,
        layout: TacticalLayout,
    ) -> EngineResult<()> {
        let elapsed = self.clock.total_elapsed_seconds();
        self.team_mut(team_id)?
            .activate_prepared_plan(plan_id, profile_id, layout, elapsed);
        Ok(())
    }
}
