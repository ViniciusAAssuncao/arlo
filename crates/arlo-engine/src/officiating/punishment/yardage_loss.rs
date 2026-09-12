use crate::world_state::match_state::state::MatchState;
use arlo_domain::sport_constants::DEFAULT_YARDAGE_LOSS_MIRIM;
use arlo_math::units::{Position, MIRIM_TO_METERS};
use uuid::Uuid;

pub fn apply_yardage_loss(
    state: &mut MatchState,
    offending_team_id: Uuid,
    magnitude: Option<i32>,
) {
    let loss_mirim = magnitude
        .map(|m| m as f64)
        .unwrap_or(DEFAULT_YARDAGE_LOSS_MIRIM)
        .max(0.0);

    if state.possession().offense() == offending_team_id {
        let current_adv = state.possession().advanced_mirins();
        let delta = current_adv.min(loss_mirim);
        state.possession_mut().series_state_mut().record_advance(-delta);
    }

    let is_offender_home = offending_team_id == state.home_team_id();
    let loss_meters = loss_mirim * MIRIM_TO_METERS;
    let shift_meters = if is_offender_home {
        -loss_meters
    } else {
        loss_meters
    };

    let current_scrimmage = state.possession().scrimmage_point();
    let pitch_length_meters = state.pitch().length().value();

    let new_x = (current_scrimmage.raw().0 + shift_meters).clamp(0.0, pitch_length_meters);
    let new_scrimmage = Position::from_components(
        new_x,
        current_scrimmage.raw().1,
        current_scrimmage.raw().2,
    );

    state
        .possession_mut()
        .series_state_mut()
        .set_scrimmage_point(new_scrimmage);
}