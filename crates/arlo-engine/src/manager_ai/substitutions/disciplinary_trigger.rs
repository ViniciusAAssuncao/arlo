use crate::world_state::AvailabilityState;
use arlo_domain::sport_constants::{DISCIPLINARY_ACTIVE_URGENCY, DISCIPLINARY_SUSPENSION_URGENCY};

pub fn disciplinary_urgency(state: AvailabilityState) -> f64 {
    match state {
        AvailabilityState::Suspended { .. } => DISCIPLINARY_SUSPENSION_URGENCY,
        AvailabilityState::Active | AvailabilityState::Expelled => DISCIPLINARY_ACTIVE_URGENCY,
    }
}