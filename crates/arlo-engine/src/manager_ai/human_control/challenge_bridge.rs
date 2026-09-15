use crate::manager_ai::challenges::apply_challenge;
use crate::rng::RngStream;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_events::EventSink;
use arlo_manager_control::ManagerDecisionInbox;
use uuid::Uuid;

pub fn try_apply_human_challenge(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    team_id: Uuid,
    inbox: &ManagerDecisionInbox,
) -> bool {
    if inbox.take_challenge(team_id).is_none() {
        return false;
    }
    let Some((call_team_id, call)) = publisher.state().last_reviewable_call().cloned() else {
        return false;
    };
    if call_team_id != team_id {
        return false;
    }
    let seq = publisher.state().event_sequence();
    let mut rng = publisher
        .state()
        .rng_provider()
        .indexed_rng_for(RngStream::ChallengeResolution, seq);
    apply_challenge(publisher, team_id, &call, &mut rng)
}
