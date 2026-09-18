use crate::match_decision::scoring::ScoringDecision;
use crate::possession::{LiveSequenceTracker, PossessionSnapshot};
use crate::world_state::match_state::MatchState;
use uuid::Uuid;

pub fn enrich_scoring_decision_assister(
    scoring_decision: &mut ScoringDecision,
    live_sequence: &LiveSequenceTracker,
) {
    if let ScoringDecision::GoalPoint {
        scorer_id,
        assister_id,
        ..
    } = scoring_decision
    {
        if assister_id.is_none() {
            *assister_id = live_sequence.primary_assister(*scorer_id);
        }
    }
}

pub fn apply_match_score(
    state: &mut MatchState,
    offense_team_id: Uuid,
    scoring_decision: &ScoringDecision,
) {
    match scoring_decision {
        ScoringDecision::GoalPoint { .. } => {
            state.record_goal_point(offense_team_id);
        }
        ScoringDecision::FieldPoint { .. } => {
            state.record_field_point(offense_team_id);
        }
        ScoringDecision::FieldGoal { post, .. } => {
            state.record_field_goal(offense_team_id, *post);
        }
        _ => {}
    }
}

pub fn post_transition_score_reset(
    state: &mut MatchState,
    scoring_decision: &ScoringDecision,
    is_possession_change: bool,
    mut next_snapshot: PossessionSnapshot,
) -> PossessionSnapshot {
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