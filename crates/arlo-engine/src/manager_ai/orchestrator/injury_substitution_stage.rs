use crate::manager_ai::substitutions::injury_substitution::execute_forced_injury_substitutions;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use uuid::Uuid;

pub fn evaluate_injury_substitution_stage(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
) {
    execute_forced_injury_substitutions(publisher, team_id);
}