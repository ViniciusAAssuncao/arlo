use super::MatchState;
use crate::error::EngineResult;
use uuid::Uuid;

impl MatchState {
    pub(crate) fn record_injury(
        &mut self,
        team_id: Uuid,
        player_id: Uuid,
        withdraw: bool,
        replacement_id: Option<Uuid>,
    ) -> EngineResult<()> {
        self.team_mut(team_id)?.record_injury(player_id, withdraw, replacement_id);
        if withdraw && self.possessor_team_id() == team_id && self.carrier_id() == Some(player_id) {
            let next_carrier = self.team(team_id)?.active_player_ids().first().copied();
            if let Some(next_carrier) = next_carrier {
                self.possession = self.possession.with_carrier(next_carrier);
            }
        }
        Ok(())
    }
}
