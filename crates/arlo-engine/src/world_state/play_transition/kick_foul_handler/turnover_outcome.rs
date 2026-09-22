use crate::world_state::play_transition::kick_foul_handler::possession_replacement::replace_possession_preserving_ball_and_clock;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use uuid::Uuid;

pub fn apply_turnover_outcome(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    awarded_team_id: Uuid,
    turnover_spot_x_mirim: f64,
) {
    let opposing_team_id = if awarded_team_id == publisher.state().home_team_id() {
        publisher.state().away_team_id()
    } else {
        publisher.state().home_team_id()
    };
    let next_role = crate::possession::PossessionRole::new(opposing_team_id, awarded_team_id);
    let mut new_series = publisher.state().possession().series_state().clone();
    new_series.reset(turnover_spot_x_mirim);

    let new_origin = crate::possession::PossessionOrigin::new(turnover_spot_x_mirim);

    replace_possession_preserving_ball_and_clock(publisher.state_mut(), next_role, new_series, new_origin);
}