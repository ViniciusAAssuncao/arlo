use crate::artrine::ArtrineExecutionOutcome;
use crate::lineup_runtime::find_goalguard;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::possession::TransitionResult;
use crate::psychology::systems::events::{ImpulseEvent, ImpulseEventKind};
use crate::psychology::systems::instrumentation::{duel_impulse_events, scoring_impulse_events};
use crate::resolution::AttributedDuelOutcome;
use crate::world_state::play_transition::possession_resolver::resolve_possession_transition;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::sport_constants::FOUL_IMPULSE_EPV_DELTA;
use arlo_domain::Player;
use arlo_events::EventSink;
use uuid::Uuid;

fn apply_team_impulse<S: EventSink>(
    publisher: &mut EventPublisher<'_, S>,
    players: &[&Player],
    event: &ImpulseEvent,
    is_involved: impl Fn(Uuid) -> bool,
) {
    for player in players {
        let pid = player.id();
        let ev = ImpulseEvent::new(event.kind(), event.surprisal(), event.epv_delta(), is_involved(pid));
        if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &ev) {
            publisher.emit_impulse_shift(pid, &shift, &ev);
        }
    }
}

fn apply_player_impulse<S: EventSink>(
    publisher: &mut EventPublisher<'_, S>,
    player_id: Uuid,
    event: &ImpulseEvent,
) {
    if let Some(shift) = publisher.state_mut().apply_impulse_event(player_id, event) {
        publisher.emit_impulse_shift(player_id, &shift, event);
    }
}

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
        let (att_event, def_event) = duel_impulse_events(duel.outcome());
        apply_team_impulse(publisher, &offense_players, &att_event, |id| duel.is_active_attacker(&id));
        apply_team_impulse(publisher, &defense_players, &def_event, |id| duel.is_active_defender(&id));
    }

    for foul in &execution_outcome.fouls {
        let p = foul.trigger_probability().clamp(0.0001, 0.9999);
        let surprisal = -p.ln();
        let committed_event = ImpulseEvent::new(
            ImpulseEventKind::FoulCommitted,
            surprisal,
            FOUL_IMPULSE_EPV_DELTA,
            true,
        );
        let drawn_event = ImpulseEvent::new(
            ImpulseEventKind::FoulDrawn,
            surprisal,
            FOUL_IMPULSE_EPV_DELTA,
            true,
        );
        apply_player_impulse(publisher, foul.offending_player_id(), &committed_event);
        apply_player_impulse(publisher, foul.opposing_player_id(), &drawn_event);
    }

    let finisher_id = execution_outcome.receiver_id.unwrap_or(artrine_id);
    let goalguard_id = find_goalguard(&defense_players).map(|g| g.id()).ok();

    if let Some((score_for, score_against)) = scoring_impulse_events(&execution_outcome.scoring_decision, 0.5) {
        apply_team_impulse(publisher, &offense_players, &score_for, |id| id == finisher_id);
        apply_team_impulse(publisher, &defense_players, &score_against, |id| Some(id) == goalguard_id);
    }

    let previous_down = publisher.state().possession().down();
    let transition_result = resolve_possession_transition(publisher.state(), detailed_outcome);

    if detailed_outcome.turnover.is_some() {
        let committed = ImpulseEvent::new(ImpulseEventKind::TurnoverCommitted, 1.20, 2.5, true);
        let won = ImpulseEvent::new(ImpulseEventKind::TurnoverWon, 1.20, 2.5, true);
        apply_team_impulse(publisher, &offense_players, &committed, |id| Some(id) == detailed_outcome.lost_by_player_id);
        apply_team_impulse(publisher, &defense_players, &won, |id| Some(id) == detailed_outcome.recovering_player_id);
    } else if publisher.state().possession().series_state().should_turnover_on_downs() {
        let failure_event = ImpulseEvent::new(ImpulseEventKind::SeriesFailure, 0.90, 2.0, true);
        let success_event = ImpulseEvent::new(ImpulseEventKind::SeriesSuccess, 0.90, 2.0, true);
        apply_team_impulse(publisher, &offense_players, &failure_event, |_| true);
        apply_team_impulse(publisher, &defense_players, &success_event, |_| true);
    } else if transition_result.snapshot.down() == 1
        && previous_down > 1
        && !detailed_outcome.scoring_decision.is_scored()
    {
        let success_event = ImpulseEvent::new(ImpulseEventKind::SeriesSuccess, 0.70, 1.5, false);
        let failure_event = ImpulseEvent::new(ImpulseEventKind::SeriesFailure, 0.70, 1.5, false);
        apply_team_impulse(publisher, &offense_players, &success_event, |_| false);
        apply_team_impulse(publisher, &defense_players, &failure_event, |_| false);
    }

    transition_result
}