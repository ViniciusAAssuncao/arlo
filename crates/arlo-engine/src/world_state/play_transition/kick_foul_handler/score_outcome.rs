use crate::match_decision::scoring::ScoringDecision;
use crate::world_state::play_transition::kick_foul_handler::possession_replacement::replace_possession_preserving_ball_and_clock;
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::play_transition::scoring_handler::apply_match_score;
use arlo_events::EventSink;
use arlo_math::units::Position as VectorPosition;
use uuid::Uuid;

pub fn apply_score_outcome(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    awarded_team_id: Uuid,
    scoring_decision: &ScoringDecision,
) {
    apply_match_score(publisher.state_mut(), awarded_team_id, scoring_decision);
    publisher.emit_scoring_event(scoring_decision);

    let center_scrimmage = VectorPosition::from_components(
        publisher.state().pitch().length().value() / 2.0,
        publisher.state().pitch().width().value() / 2.0,
        0.0,
    );

    let swapped_role = publisher.state().possession().role().swap();
    let mut new_series = publisher.state().possession().series_state().clone();
    new_series.reset(center_scrimmage);

    if matches!(scoring_decision, ScoringDecision::GoalPoint { .. }) {
        new_series.is_bonus_phase = true;
    }

    replace_possession_preserving_ball_and_clock(publisher.state_mut(), swapped_role, new_series);
}