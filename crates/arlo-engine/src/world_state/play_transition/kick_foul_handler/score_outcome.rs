use crate::match_decision::scoring::ScoringDecision;
use crate::possession::bonus_phase_scrimmage_x;
use crate::world_state::play_transition::kick_foul_handler::possession_replacement::replace_possession_preserving_ball_and_clock;
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::play_transition::scoring_handler::apply_match_score;
use arlo_domain::sport_constants::AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM;
use arlo_events::EventSink;
use uuid::Uuid;

pub fn apply_score_outcome(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    awarded_team_id: Uuid,
    scoring_decision: &ScoringDecision,
) {
    apply_match_score(publisher.state_mut(), awarded_team_id, scoring_decision);
    publisher.emit_scoring_event(scoring_decision);

    if matches!(scoring_decision, ScoringDecision::GoalPoint { .. }) {
        let is_home = awarded_team_id == publisher.state().home_team_id();
        let bonus_spot_x = bonus_phase_scrimmage_x(
            publisher.state().pitch().length_mirim(),
            AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM,
            is_home,
        );
        let current_role = *publisher.state().possession().role();
        let mut new_series = publisher.state().possession().series_state().clone();
        new_series.reset(bonus_spot_x);
        new_series.set_bonus_phase(true);

        let new_origin = crate::possession::PossessionOrigin::new(bonus_spot_x);

        replace_possession_preserving_ball_and_clock(
            publisher.state_mut(),
            current_role,
            new_series,
            new_origin,
        );
    } else {
        let center_scrimmage_x_mirim = publisher.state().pitch().length_mirim() / 2.0;

        let swapped_role = publisher.state().possession().role().swap();
        let mut new_series = publisher.state().possession().series_state().clone();
        new_series.reset(center_scrimmage_x_mirim);
        new_series.set_bonus_phase(false);

        let new_origin = crate::possession::PossessionOrigin::new(center_scrimmage_x_mirim);

        replace_possession_preserving_ball_and_clock(
            publisher.state_mut(),
            swapped_role,
            new_series,
            new_origin,
        );
    }
}