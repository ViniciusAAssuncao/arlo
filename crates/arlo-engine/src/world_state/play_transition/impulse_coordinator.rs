use crate::artrine::ArtrineExecutionOutcome;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::possession::TransitionResult;
use crate::resolution::AttributedDuelOutcome;
use crate::world_state::play_transition::possession_resolver::resolve_possession_transition;
use crate::world_state::play_transition::publisher::EventPublisher;
use crate::world_state::play_transition::scoring_handler::publish_scoring_impulse;
use arlo_domain::Player;
use arlo_events::EventSink;
use uuid::Uuid;

pub fn coordinate_play_impulse(
    publisher: &mut EventPublisher<'_, impl EventSink>,
    play_duels: &[AttributedDuelOutcome],
    artrine_id: Uuid,
    execution_outcome: &ArtrineExecutionOutcome,
    detailed_outcome: &DetailedPlayOutcome,
) -> TransitionResult {
    let offense_lineup = if detailed_outcome.offense_team_id == publisher.state().home_team_id() {
        publisher.state().home_lineup_arc()
    } else {
        publisher.state().away_lineup_arc()
    };
    let defense_lineup = if detailed_outcome.defense_team_id == publisher.state().home_team_id() {
        publisher.state().home_lineup_arc()
    } else {
        publisher.state().away_lineup_arc()
    };

    let offense_players: Vec<&Player> = offense_lineup
        .assignments()
        .iter()
        .map(|a| a.player())
        .collect();
    let defense_players: Vec<&Player> = defense_lineup
        .assignments()
        .iter()
        .map(|a| a.player())
        .collect();

    for duel in play_duels {
        publisher
            .state_mut()
            .impulse_bus_mut()
            .publish_attributed_duel(duel, &offense_players, &defense_players);
    }

    let finisher_id = execution_outcome.receiver_id.unwrap_or(artrine_id);

    publish_scoring_impulse(
        publisher.state_mut(),
        &execution_outcome.scoring_decision,
        finisher_id,
        &defense_players,
        &offense_players,
    );

    let transition_result = resolve_possession_transition(publisher.state(), detailed_outcome);

    publisher
        .state_mut()
        .impulse_bus_mut()
        .publish_events(transition_result.impulse_events.clone());

    let current_period_seconds = publisher.state().clock().seconds_in_period();
    let shifts = publisher
        .state_mut()
        .process_impulse_bus(current_period_seconds);
    for (pid, shift, ev) in shifts {
        publisher.emit_impulse_shift(pid, &shift, &ev);
    }

    transition_result
}
