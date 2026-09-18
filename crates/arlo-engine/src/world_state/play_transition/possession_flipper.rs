use crate::match_decision::scoring::ScoringDecision;
use crate::possession::{PossessionSnapshot, TransitionResult};
use crate::world_state::match_state::MatchState;
use uuid::Uuid;

pub fn finalize_possession_flip(
    state: &mut MatchState,
    scoring_decision: &ScoringDecision,
    offense_team_id: Uuid,
    transition_result: &TransitionResult,
) -> PossessionSnapshot {
    let is_possession_change =
        transition_result.snapshot.role().offense() != offense_team_id;

    let mut next_snapshot = transition_result.snapshot.clone();

    if scoring_decision.is_scored() || is_possession_change {
        state.reset_drives();
    }

    if scoring_decision.is_scored() {
        let center_scrimmage_x_mirim = state.pitch().length_mirim() / 2.0;
        next_snapshot.series_state_mut().reset(center_scrimmage_x_mirim);
        next_snapshot.series_state_mut().is_bonus_phase = false;
    }

    next_snapshot.live_sequence.clear();
    next_snapshot
}
