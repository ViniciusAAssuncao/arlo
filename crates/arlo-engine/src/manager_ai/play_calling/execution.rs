use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_tactics::PlayCall;
use uuid::Uuid;

pub fn execute_play_call_selection(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    play_call: PlayCall,
) {
    let play_call_id = play_call.id();
    let play_call_name = play_call.name().to_string();
    let category = play_call.category();

    publisher
        .state_mut()
        .set_active_play_call(team_id, play_call);

    publisher.emit_play_call_selected(team_id, play_call_id, play_call_name, category);
}