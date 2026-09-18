use crate::world_state::match_state::state::MatchState;
use arlo_domain::sport_constants::DEFAULT_YARDAGE_LOSS_MIRIM;
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
    let shift_mirim = if is_offender_home {
        -loss_mirim
    } else {
        loss_mirim
    };

    let current_x = state.possession().scrimmage_x_mirim();
    let pitch_length_mirim = state.pitch().length_mirim();

    let new_x = (current_x + shift_mirim).clamp(0.0, pitch_length_mirim);

    state
        .possession_mut()
        .series_state_mut()
        .set_scrimmage_x_mirim(new_x);
}
