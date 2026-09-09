use crate::error::EngineResult;
use crate::manager_ai::substitutions::decision::SubstitutionPlan;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use uuid::Uuid;

pub fn execute_substitutions(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    plans: &[SubstitutionPlan],
) -> EngineResult<usize> {
    let mut executed = 0;
    for plan in plans {
        let is_home = team_id == publisher.state().home_team_id();
        let incoming_player = {
            let squad = if is_home {
                publisher.state().home_squad()
            } else {
                publisher.state().away_squad()
            };
            squad
                .bench()
                .iter()
                .find(|p| p.id() == plan.incoming_id)
                .cloned()
        };

        if let Some(incoming) = incoming_player {
            publisher
                .state_mut()
                .apply_substitution(team_id, plan.outgoing_id, incoming)?;

            let clock_inst = publisher.state().clock().to_instant();
            publisher.emit_substitution_made(
                team_id,
                plan.outgoing_id,
                plan.incoming_id,
                clock_inst,
                plan.reason,
            );
            executed += 1;
        }
    }
    Ok(executed)
}