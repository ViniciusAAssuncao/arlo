use crate::error::EngineResult;
use crate::world_state::match_state::matchday_squad::MatchdaySquad;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::Player;
use uuid::Uuid;

impl MatchState {
    pub fn home_squad(&self) -> &MatchdaySquad {
        &self.home_squad
    }

    pub fn away_squad(&self) -> &MatchdaySquad {
        &self.away_squad
    }

    pub fn squad_for_team(&self, team_id: Uuid) -> &MatchdaySquad {
        if team_id == self.teams.home_team_id() {
            &self.home_squad
        } else {
            &self.away_squad
        }
    }

    pub fn squad_for_team_mut(&mut self, team_id: Uuid) -> &mut MatchdaySquad {
        if team_id == self.teams.home_team_id() {
            &mut self.home_squad
        } else {
            &mut self.away_squad
        }
    }

    pub fn apply_substitution(
        &mut self,
        team_id: Uuid,
        outgoing: Uuid,
        incoming: Player,
    ) -> EngineResult<()> {
        let is_home = team_id == self.teams.home_team_id();
        let outgoing_player = self.teams.find_player(&outgoing).cloned();
        let incoming_id = incoming.id();

        let current_lineup = if is_home {
            self.teams.home_lineup()
        } else {
            self.teams.away_lineup()
        };

        let new_lineup = current_lineup.substitute(outgoing, incoming.clone())?;
        self.teams.replace_lineup(team_id, new_lineup);

        let squad = if is_home {
            &mut self.home_squad
        } else {
            &mut self.away_squad
        };
        squad.mark_substituted(outgoing);
        if let Some(out_p) = outgoing_player {
            squad.swap_bench(out_p, incoming_id);
        }

        self.substitute_fatigue_player(outgoing, incoming_id, is_home);
        self.substitute_impulse_player(outgoing, incoming_id, is_home);

        Ok(())
    }
}