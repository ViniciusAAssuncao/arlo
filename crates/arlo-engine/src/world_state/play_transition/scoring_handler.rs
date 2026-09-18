use crate::match_decision::scoring::ScoringDecision;
use crate::possession::{LiveSequenceTracker, PossessionSnapshot};
use crate::rng::RngStream;
use crate::set_piece::execute_bonus_phase_conversion;
use crate::world_state::match_state::MatchState;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
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

pub fn process_goal_point_bonus_phase<S: EventSink>(
    publisher: &mut EventPublisher<'_, S>,
    offense_team_id: Uuid,
    scrimmage_x_mirim: f64,
) {
    let seq = publisher.state_mut().next_sequence();
    let mut bonus_rng = publisher
        .state()
        .rng_provider()
        .indexed_rng_for(RngStream::DuelResolution, seq);

    publisher
        .state_mut()
        .possession_mut()
        .series_state_mut()
        .set_bonus_phase(true);

    execute_bonus_phase_conversion(
        publisher,
        offense_team_id,
        scrimmage_x_mirim,
        &mut bonus_rng,
    );

    publisher
        .state_mut()
        .possession_mut()
        .series_state_mut()
        .set_bonus_phase(false);
    publisher.state_mut().reset_drives();
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
        next_snapshot.series_state_mut().set_bonus_phase(false);
    }

    next_snapshot.live_sequence.clear();
    next_snapshot
}