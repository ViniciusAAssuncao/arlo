use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_tactics::TeamTacticalProfile;
use uuid::Uuid;

pub fn execute_tactical_adjustment(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    profile: &TeamTacticalProfile,
) {
    publisher
        .state_mut()
        .activate_tactical_profile(team_id, profile.clone());
    publisher.emit_tactical_profile_activated(team_id, profile.id(), profile.name());
}

pub fn execute_tactical_adjustment_by_id(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    profile_id: Uuid,
    available_profiles: &[TeamTacticalProfile],
) -> bool {
    if let Some(profile) = available_profiles.iter().find(|p| p.id() == profile_id) {
        execute_tactical_adjustment(publisher, team_id, profile);
        true
    } else {
        false
    }
}