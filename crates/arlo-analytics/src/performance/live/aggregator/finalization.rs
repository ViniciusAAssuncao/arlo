use super::PlayerPerformanceAggregator;
use crate::performance::live::finalizer::{
    apply_outcome_to_players, clear_players_outcome, extract_player_match_ratings,
};
use crate::performance::rating::{MatchOutcome, OutcomeAdjustmentPolicy};
use crate::performance::live::snapshot::LivePerformanceSnapshotRecord;
use crate::performance::PlayerMatchRating;
use uuid::Uuid;

impl PlayerPerformanceAggregator {
    pub fn apply_match_outcome(
        &mut self,
        team_id: Uuid,
        outcome: MatchOutcome,
        policy: &OutcomeAdjustmentPolicy,
    ) {
        apply_outcome_to_players(&mut self.players, team_id, outcome, policy);
        self.finalization.record_outcome(team_id, outcome);
    }

    pub fn apply_outcome(&mut self, team_id: Uuid, outcome: MatchOutcome) {
        let policy = *self.config.outcome_policy();
        self.apply_match_outcome(team_id, outcome, &policy);
    }

    pub fn finalize_match(
        &mut self,
        home_team_id: Uuid,
        home_outcome: MatchOutcome,
        away_team_id: Uuid,
        away_outcome: MatchOutcome,
        policy: &OutcomeAdjustmentPolicy,
    ) -> LivePerformanceSnapshotRecord {
        self.apply_match_outcome(home_team_id, home_outcome, policy);
        self.apply_match_outcome(away_team_id, away_outcome, policy);
        self.finalization.mark_finalized();

        let snapshot = self.create_snapshot(self.sequence_counter, self.last_clock);
        if let Some(last) = self.history.last_mut() {
            if last.sequence_number() == self.sequence_counter && last.clock() == self.last_clock {
                *last = snapshot.clone();
                return snapshot;
            }
        }

        self.history.push(snapshot.clone());
        snapshot
    }

    pub fn finalize_match_with_scores(
        &mut self,
        home_team_id: Uuid,
        home_score: u32,
        away_team_id: Uuid,
        away_score: u32,
        policy: &OutcomeAdjustmentPolicy,
    ) -> LivePerformanceSnapshotRecord {
        let home_outcome = MatchOutcome::from_scores(home_score, away_score);
        let away_outcome = MatchOutcome::from_scores(away_score, home_score);
        self.finalize_match(
            home_team_id,
            home_outcome,
            away_team_id,
            away_outcome,
            policy,
        )
    }

    pub fn finalize_with_config(
        &mut self,
        home_team_id: Uuid,
        home_outcome: MatchOutcome,
        away_team_id: Uuid,
        away_outcome: MatchOutcome,
    ) -> LivePerformanceSnapshotRecord {
        let policy = *self.config.outcome_policy();
        self.finalize_match(
            home_team_id,
            home_outcome,
            away_team_id,
            away_outcome,
            &policy,
        )
    }

    pub fn finalize_with_scores_and_config(
        &mut self,
        home_team_id: Uuid,
        home_score: u32,
        away_team_id: Uuid,
        away_score: u32,
    ) -> LivePerformanceSnapshotRecord {
        let policy = *self.config.outcome_policy();
        self.finalize_match_with_scores(
            home_team_id,
            home_score,
            away_team_id,
            away_score,
            &policy,
        )
    }

    pub fn is_finalized(&self) -> bool {
        self.finalization.is_finalized()
    }

    pub fn finalized_outcome_for_team(&self, team_id: &Uuid) -> Option<MatchOutcome> {
        self.finalization.outcome_for_team(team_id)
    }

    pub fn final_player_ratings(&self) -> Vec<PlayerMatchRating> {
        extract_player_match_ratings(&self.players)
    }

    pub fn final_player_rating(&self, player_id: &Uuid) -> Option<f64> {
        self.players
            .get(player_id)
            .map(|player| player.final_rating().value())
    }

    pub fn clear_match_outcomes(&mut self) {
        clear_players_outcome(&mut self.players);
        self.finalization.clear();
    }
}
