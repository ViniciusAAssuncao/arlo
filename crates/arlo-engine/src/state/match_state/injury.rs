use super::MatchState;
use crate::error::EngineResult;
use crate::state::PendingInjuryDecision;
use arlo_domain::InjurySeverityGrade;
use uuid::Uuid;

impl MatchState {
    pub fn injury_decision_is_actionable(&self, pending: PendingInjuryDecision) -> bool {
        self.team(pending.team_id()).is_ok_and(|team| {
            team.active_player_ids().contains(&pending.player_id())
                && team.injured_player_ids().contains(&pending.player_id())
        })
    }
    pub(crate) fn match_injury(&self, player_id: Uuid) -> Option<(Uuid, InjurySeverityGrade)> {
        self.injuries.get(&player_id).copied()
    }

    pub(crate) fn set_match_injury(&mut self, player_id: Uuid, definition_id: Uuid, grade: InjurySeverityGrade) {
        self.injuries.insert(player_id, (definition_id, grade));
    }
    pub(crate) fn queue_injury_decision(&mut self, team_id: Uuid, player_id: Uuid, injury_definition_id: Uuid, severity_grade: InjurySeverityGrade) {
        self.pending_injury_decisions.push(PendingInjuryDecision::new(team_id, player_id, injury_definition_id, severity_grade));
    }

    pub(crate) fn mark_injury_out(&mut self) {
        self.injury_decisions_ready = !self.pending_injury_decisions.is_empty();
    }

    pub(crate) fn queue_forced_substitution(&mut self, team_id: Uuid, player_id: Uuid) {
        self.pending_forced_substitutions.push((team_id, player_id));
    }

    pub(crate) fn clear_forced_substitution(&mut self, team_id: Uuid, player_id: Uuid) {
        self.pending_forced_substitutions.retain(|pending| *pending != (team_id, player_id));
    }

    pub(crate) fn clear_injury_decision(&mut self, team_id: Uuid, player_id: Uuid) {
        self.pending_injury_decisions.retain(|pending| pending.team_id() != team_id || pending.player_id() != player_id);
        if self.pending_injury_decisions.is_empty() {
            self.injury_decisions_ready = false;
        }
    }

    pub(crate) fn withdraw_injured_player(
        &mut self,
        team_id: Uuid,
        player_id: Uuid,
        replacement_id: Option<Uuid>,
    ) -> EngineResult<()> {
        self.team_mut(team_id)?.withdraw_injured_player(player_id, replacement_id);
        self.clear_injury_decision(team_id, player_id);
        Ok(())
    }

    pub(crate) fn replace_withdrawn_player(
        &mut self,
        team_id: Uuid,
        player_id: Uuid,
        replacement_id: Uuid,
    ) -> EngineResult<()> {
        self.team_mut(team_id)?.withdraw_injured_player(player_id, Some(replacement_id));
        self.clear_forced_substitution(team_id, player_id);
        Ok(())
    }
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
