use crate::world_state::match_state::state::MatchState;
use arlo_tactics::{PlayCall, PlayCallCategory};
use uuid::Uuid;

impl MatchState {
    pub fn set_home_series_script(&mut self, entries: Vec<PlayCall>) {
        self.play_calling.set_series_script(true, entries);
    }

    pub fn set_away_series_script(&mut self, entries: Vec<PlayCall>) {
        self.play_calling.set_series_script(false, entries);
    }

    pub fn set_active_play_call(&mut self, team_id: Uuid, play_call: PlayCall) {
        let is_home = team_id == self.teams.home_team_id();
        self.play_calling.set_manual_override(is_home, play_call);
    }

    pub fn has_queued_call_for_offense(&self) -> bool {
        let is_home_offense = self.possession.role().is_offense(self.home_team_id());
        let expected_category = if self.possession.is_bonus_phase() {
            PlayCallCategory::BonusPhaseConversion
        } else {
            PlayCallCategory::OpenPlay
        };
        self.play_calling
            .has_queued_call(is_home_offense, expected_category)
    }

    pub fn resolve_active_play_call_for_offense(&mut self) -> Option<PlayCall> {
        let is_home_offense = self.possession.role().is_offense(self.home_team_id());
        let expected_category = if self.possession.is_bonus_phase() {
            PlayCallCategory::BonusPhaseConversion
        } else {
            PlayCallCategory::OpenPlay
        };
        self.play_calling
            .resolve_and_consume(is_home_offense, expected_category)
    }
}
