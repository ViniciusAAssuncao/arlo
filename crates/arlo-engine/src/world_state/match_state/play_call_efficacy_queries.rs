use crate::manager_ai::ManagerSnapshot;
use crate::team_identity::extract_manager_attribute_value;
use crate::world_state::match_state::state::MatchState;
use arlo_domain::sport_constants::{
    ATTRIBUTE_MAX, BELIEF_EVIDENCE_DECAY_MAX, BELIEF_EVIDENCE_DECAY_MIN,
    BELIEF_PRIOR_STRENGTH_MAX, BELIEF_PRIOR_STRENGTH_MIN,
};
use arlo_domain::AttributeKey;
use arlo_math::stats::BetaBelief;
use arlo_tactics::PlayCall;
use std::collections::HashMap;
use uuid::Uuid;

impl MatchState {
    pub fn ensure_play_call_beliefs_seeded(
        &mut self,
        team_id: Uuid,
        playbook: &[PlayCall],
        ranked: &[(usize, f64)],
        manager_snapshot: &ManagerSnapshot,
    ) {
        let is_home = team_id == self.teams.home_team_id();
        let norm_adapt = if manager_snapshot.adaptability > 1.0 {
            (manager_snapshot.adaptability / 20.0).clamp(0.0, 1.0)
        } else {
            manager_snapshot.adaptability.clamp(0.0, 1.0)
        };
        let prior_strength = BELIEF_PRIOR_STRENGTH_MAX
            - norm_adapt * (BELIEF_PRIOR_STRENGTH_MAX - BELIEF_PRIOR_STRENGTH_MIN);

        for &(idx, fit_score) in ranked {
            if let Some(play_call) = playbook.get(idx) {
                self.play_call_efficacy.get_or_seed(
                    is_home,
                    play_call.id(),
                    fit_score,
                    prior_strength,
                );
            }
        }
    }

    pub fn play_call_efficacy_snapshot(&self, team_id: Uuid) -> HashMap<Uuid, BetaBelief> {
        let is_home = team_id == self.teams.home_team_id();
        self.play_call_efficacy.snapshot(is_home)
    }

    pub fn record_play_call_outcome(
        &mut self,
        team_id: Uuid,
        play_call_id: Uuid,
        success: bool,
    ) {
        let is_home = team_id == self.teams.home_team_id();
        let manager = self.teams.manager_for_team(team_id);
        let raw_adaptability = extract_manager_attribute_value(
            manager,
            &self.attribute_keys,
            AttributeKey::Adaptability,
        );
        let norm_adaptability = (raw_adaptability / ATTRIBUTE_MAX).clamp(0.0, 1.0);
        let decay_factor = BELIEF_EVIDENCE_DECAY_MAX
            - norm_adaptability * (BELIEF_EVIDENCE_DECAY_MAX - BELIEF_EVIDENCE_DECAY_MIN);

        self.play_call_efficacy
            .record_outcome(is_home, play_call_id, success, decay_factor);
    }
}