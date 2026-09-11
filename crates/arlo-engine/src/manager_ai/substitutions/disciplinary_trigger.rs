use crate::world_state::AvailabilityState;

pub fn disciplinary_urgency(state: AvailabilityState) -> f64 {
    match state {
        AvailabilityState::Suspended { .. } => 1.0,
        AvailabilityState::Active | AvailabilityState::Expelled => 0.0,
    }
}