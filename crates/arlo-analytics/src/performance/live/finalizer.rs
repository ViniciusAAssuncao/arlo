use crate::performance::live::player_state::LivePlayerState;
use crate::performance::rating::{MatchOutcome, OutcomeAdjustmentPolicy};
use crate::performance::PlayerMatchRating;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatchFinalizationState {
    is_finalized: bool,
    outcomes: HashMap<Uuid, MatchOutcome>,
}

impl MatchFinalizationState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_finalized(&self) -> bool {
        self.is_finalized
    }

    pub fn outcome_for_team(&self, team_id: &Uuid) -> Option<MatchOutcome> {
        self.outcomes.get(team_id).copied()
    }

    pub fn outcomes(&self) -> &HashMap<Uuid, MatchOutcome> {
        &self.outcomes
    }

    pub fn mark_finalized(&mut self) {
        self.is_finalized = true;
    }

    pub fn record_outcome(&mut self, team_id: Uuid, outcome: MatchOutcome) {
        self.outcomes.insert(team_id, outcome);
    }

    pub fn clear(&mut self) {
        self.is_finalized = false;
        self.outcomes.clear();
    }
}

pub fn apply_outcome_to_players(
    players: &mut HashMap<Uuid, LivePlayerState>,
    team_id: Uuid,
    outcome: MatchOutcome,
    policy: &OutcomeAdjustmentPolicy,
) {
    let adjustment = policy.adjustment_for(outcome);
    for state in players.values_mut() {
        if state.team_id() == team_id {
            state.set_outcome_adjustment(adjustment);
        }
    }
}

pub fn clear_players_outcome(players: &mut HashMap<Uuid, LivePlayerState>) {
    for state in players.values_mut() {
        state.clear_outcome_adjustment();
    }
}

pub fn extract_player_match_ratings(
    players: &HashMap<Uuid, LivePlayerState>,
) -> Vec<PlayerMatchRating> {
    let mut list: Vec<PlayerMatchRating> = players
        .values()
        .map(|p| PlayerMatchRating::new(p.player_id(), p.final_rating().value()))
        .collect();
    list.sort_by_key(|r| r.player_id());
    list
}