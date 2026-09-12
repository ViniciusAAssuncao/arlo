use crate::match_decision::event_translation::translate_duel_kind;
use crate::officiating::foul::{FoulOrigin as EngineFoulOrigin, FoulResolution};
use crate::world_state::match_state::availability::AvailabilityState;
use arlo_events::{
    AvailabilityStatus, FoulOrigin as EventFoulOrigin, FoulRaised, PlayerAvailabilityChanged,
};
use uuid::Uuid;

pub fn translate_foul_origin(origin: EngineFoulOrigin) -> EventFoulOrigin {
    match origin {
        EngineFoulOrigin::ContactDuel(kind) => {
            EventFoulOrigin::ContactDuel(translate_duel_kind(kind))
        }
        EngineFoulOrigin::LineFault => EventFoulOrigin::LineFault,
    }
}

pub fn translate_foul_raised(resolution: &FoulResolution) -> FoulRaised {
    FoulRaised::new(
        resolution.offending_player_id,
        resolution.offending_team_id,
        resolution.opposing_player_id,
        resolution.opposing_team_id,
        translate_foul_origin(resolution.origin()),
        resolution.original_call_correct,
        resolution.peace_referee_intervened,
        resolution.fault_definition_id,
        resolution.punishment_kind,
        resolution.punishment_magnitude,
    )
}

pub fn translate_availability_changed(
    player_id: Uuid,
    team_id: Uuid,
    previous: AvailabilityState,
    new: AvailabilityState,
) -> PlayerAvailabilityChanged {
    let (prev_status, _) = map_availability_state(previous);
    let (new_status, rem_secs) = map_availability_state(new);
    PlayerAvailabilityChanged::new(player_id, team_id, prev_status, new_status, rem_secs)
}

fn map_availability_state(state: AvailabilityState) -> (AvailabilityStatus, Option<f64>) {
    match state {
        AvailabilityState::Active => (AvailabilityStatus::Active, None),
        AvailabilityState::Suspended { remaining_seconds } => {
            (AvailabilityStatus::Suspended, Some(remaining_seconds))
        }
        AvailabilityState::Expelled => (AvailabilityStatus::Expelled, None),
        AvailabilityState::Injured => (AvailabilityStatus::Injured, None),
    }
}