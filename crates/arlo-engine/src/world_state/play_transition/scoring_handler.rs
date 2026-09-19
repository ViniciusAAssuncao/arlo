use crate::match_decision::scoring::ScoringDecision;
use crate::possession::{bonus_phase_scrimmage_x, LiveSequenceTracker, PossessionSnapshot};
use crate::world_state::match_state::MatchState;
use arlo_domain::sport_constants::AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM;
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
        if matches!(scoring_decision, ScoringDecision::GoalPoint { .. }) {
            let is_home = next_snapshot.role().offense() == state.home_team_id();
            let bonus_spot_x = bonus_phase_scrimmage_x(
                state.pitch().length_mirim(),
                AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM,
                is_home,
            );
            next_snapshot.series_state_mut().reset(bonus_spot_x);
            next_snapshot.possession_origin_mut().reset(bonus_spot_x);
            next_snapshot.series_state_mut().set_bonus_phase(true);
        } else {
            let center_scrimmage_x_mirim = state.pitch().length_mirim() / 2.0;
            next_snapshot
                .series_state_mut()
                .reset(center_scrimmage_x_mirim);
            next_snapshot
                .possession_origin_mut()
                .reset(center_scrimmage_x_mirim);
            next_snapshot.series_state_mut().set_bonus_phase(false);
        }
    }

    next_snapshot.live_sequence_mut().clear();
    next_snapshot
}
