use crate::artrine::ArtrineExecutionOutcome;
use crate::lineup_runtime::find_goalguard;
use crate::match_decision::play_outcome::DetailedPlayOutcome;
use crate::match_decision::scoring::ScoringDecision;
use crate::possession::TransitionResult;
use crate::psychology::systems::events::{ImpulseEvent, ImpulseEventKind};
use crate::resolution::AttributedDuelOutcome;
use crate::world_state::play_transition::possession_resolver::resolve_possession_transition;
use crate::world_state::play_transition::publisher::EventPublisher;
use arlo_domain::sport_constants::FOUL_IMPULSE_EPV_DELTA;
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
        let outcome = duel.outcome();
        let win_p = outcome.win_probability().value().clamp(0.0001, 0.9999);
        let epv_delta = (outcome.net_advantage() * 0.1).abs();

        let (att_kind, att_p) = if outcome.attacker_won() {
            (ImpulseEventKind::DuelWon, win_p)
        } else {
            (ImpulseEventKind::DuelLost, 1.0 - win_p)
        };

        let (def_kind, def_p) = if outcome.attacker_won() {
            (ImpulseEventKind::DuelLost, win_p)
        } else {
            (ImpulseEventKind::DuelWon, 1.0 - win_p)
        };

        let att_surprisal = -att_p.ln();
        let def_surprisal = -def_p.ln();

        for player in &offense_players {
            let pid = player.id();
            let involved = duel.is_active_attacker(&pid);
            let event = ImpulseEvent::new(att_kind, att_surprisal, epv_delta, involved);
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &event) {
                publisher.emit_impulse_shift(pid, &shift, &event);
            }
        }

        for player in &defense_players {
            let pid = player.id();
            let involved = duel.is_active_defender(&pid);
            let event = ImpulseEvent::new(def_kind, def_surprisal, epv_delta, involved);
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &event) {
                publisher.emit_impulse_shift(pid, &shift, &event);
            }
        }
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
        if let Some(shift) = publisher.state_mut().apply_impulse_event(foul.offending_player_id(), &committed_event) {
            publisher.emit_impulse_shift(foul.offending_player_id(), &shift, &committed_event);
        }
        if let Some(shift) = publisher.state_mut().apply_impulse_event(foul.opposing_player_id(), &drawn_event) {
            publisher.emit_impulse_shift(foul.opposing_player_id(), &shift, &drawn_event);
        }
    }

    let finisher_id = execution_outcome.receiver_id.unwrap_or(artrine_id);
    let goalguard_id = find_goalguard(&defense_players).map(|g| g.id()).ok();

    if execution_outcome.scoring_decision.is_scored() {
        let points = execution_outcome.scoring_decision.points() as f64;
        let p = 0.5_f64;
        let att_surprisal = -p.ln();
        let def_surprisal = -(1.0 - p).ln();

        for player in &offense_players {
            let pid = player.id();
            let involved = pid == finisher_id;
            let event = ImpulseEvent::new(ImpulseEventKind::ScoreFor, att_surprisal, points, involved);
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &event) {
                publisher.emit_impulse_shift(pid, &shift, &event);
            }
        }

        for player in &defense_players {
            let pid = player.id();
            let involved = Some(pid) == goalguard_id;
            let event = ImpulseEvent::new(ImpulseEventKind::ScoreAgainst, def_surprisal, points, involved);
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &event) {
                publisher.emit_impulse_shift(pid, &shift, &event);
            }
        }
    } else if matches!(execution_outcome.scoring_decision, ScoringDecision::Missed { .. }) {
        let points = 1.0;
        let p = 0.5_f64;
        let att_surprisal = -(1.0 - p).ln();
        let def_surprisal = -p.ln();

        for player in &offense_players {
            let pid = player.id();
            let involved = pid == finisher_id;
            let event = ImpulseEvent::new(ImpulseEventKind::DuelLost, att_surprisal, points, involved);
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &event) {
                publisher.emit_impulse_shift(pid, &shift, &event);
            }
        }

        for player in &defense_players {
            let pid = player.id();
            let involved = Some(pid) == goalguard_id;
            let event = ImpulseEvent::new(ImpulseEventKind::DuelWon, def_surprisal, points, involved);
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &event) {
                publisher.emit_impulse_shift(pid, &shift, &event);
            }
        }
    }

    let previous_down = publisher.state().possession().down();
    let transition_result = resolve_possession_transition(publisher.state(), detailed_outcome);

    let is_turnover_on_downs = detailed_outcome.turnover.is_none()
        && !detailed_outcome.scoring_decision.is_scored()
        && transition_result.snapshot.role().offense() != detailed_outcome.offense_team_id;

    if detailed_outcome.turnover.is_some() {
        let is_defense_home = detailed_outcome.defense_team_id == publisher.state().home_team_id();
        for player in &offense_players {
            let pid = player.id();
            let involved = Some(pid) == detailed_outcome.lost_by_player_id;
            let event = ImpulseEvent::new(ImpulseEventKind::TurnoverCommitted, 1.20, 2.5, involved);
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &event) {
                publisher.emit_impulse_shift(pid, &shift, &event);
            }
        }
        for player in &defense_players {
            let pid = player.id();
            let involved = Some(pid) == detailed_outcome.recovering_player_id;
            let (surprisal, epv_delta) = if is_defense_home {
                (2.40, 5.0)
            } else {
                (1.20, 2.5)
            };
            let event = ImpulseEvent::new(ImpulseEventKind::TurnoverWon, surprisal, epv_delta, involved);
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &event) {
                publisher.emit_impulse_shift(pid, &shift, &event);
            }
        }
    } else if is_turnover_on_downs || publisher.state().possession().series_state().should_turnover_on_downs() {
        let is_defense_home = detailed_outcome.defense_team_id == publisher.state().home_team_id();
        let failure_event = ImpulseEvent::new(ImpulseEventKind::SeriesFailure, 1.00, 2.5, true);
        let (surprisal, epv_delta) = if is_defense_home {
            (2.80, 6.0)
        } else {
            (1.20, 2.5)
        };
        let success_event = ImpulseEvent::new(ImpulseEventKind::TurnoverWon, surprisal, epv_delta, true);

        for player in &offense_players {
            let pid = player.id();
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &failure_event) {
                publisher.emit_impulse_shift(pid, &shift, &failure_event);
            }
        }
        for player in &defense_players {
            let pid = player.id();
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &success_event) {
                publisher.emit_impulse_shift(pid, &shift, &success_event);
            }
        }
    } else if transition_result.snapshot.down() == 1
        && previous_down > 1
        && !detailed_outcome.scoring_decision.is_scored()
    {
        let success_event = ImpulseEvent::new(ImpulseEventKind::SeriesSuccess, 0.70, 1.5, false);
        let failure_event = ImpulseEvent::new(ImpulseEventKind::SeriesFailure, 0.70, 1.5, false);

        for player in &offense_players {
            let pid = player.id();
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &success_event) {
                publisher.emit_impulse_shift(pid, &shift, &success_event);
            }
        }
        for player in &defense_players {
            let pid = player.id();
            if let Some(shift) = publisher.state_mut().apply_impulse_event(pid, &failure_event) {
                publisher.emit_impulse_shift(pid, &shift, &failure_event);
            }
        }
    }

    transition_result
}