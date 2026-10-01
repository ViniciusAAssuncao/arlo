use super::super::context::validate_match_state;
use super::super::down::emit_down_advanced;
use super::super::kicker::select_kicker;
use super::super::ratings::RatingIndex;
use super::model::{default_decision, sample_attempt, sample_post};
use crate::error::{EngineError, EngineResult};
use crate::input::MatchInput;
use crate::state::{MatchPhase, MatchState, ScoreKind};
use crate::step::StepResult;
use arlo_domain::sport_constants::IMMEDIATE_POSSESSION_CONTROL_SECONDS;
use arlo_domain::KickFoulDecisionKind;
use arlo_events::{
    AddedTimeAwarded, FieldPointScored, GoalPointScored, KickFoulDecisionMade, MatchEvent,
    MatchEventEnvelope, OutOfBounds, PassCompleted, PossessionTimeRecorded, ReceptionResolved,
    ScoringAttemptMissed, ScoringPost, Turnover,
};
use arlo_manager_control::RequiredManagerDecision;
use uuid::Uuid;

pub fn resolve_kick_foul_segment(
    input: &MatchInput,
    state: &mut MatchState,
    selected_decision: Option<KickFoulDecisionKind>,
    selected_taker_id: Option<Uuid>,
) -> EngineResult<StepResult> {
    state.begin_play_checkpoint();
    let prior = state.clone();
    let result = resolve_kick_foul_segment_inner(input, state, selected_decision, selected_taker_id)?;
    let result = super::super::officiating::resolve_officiating(input, state, result, None)?;
    let (mut events, outcome) = result.into_parts();
    if !events.iter().any(|event| matches!(event.event(), MatchEvent::PlayInvalidated(_))) {
        state.record_segment_energy(input, &prior, &mut events)?;
        state.record_segment_morale(&prior, &mut events)?;
    }
    if matches!(outcome, crate::step::StepOutcome::Finished) {
        state.record_final_energy(&mut events)?;
        state.record_final_morale(&mut events)?;
    }
    let result = match outcome {
        crate::step::StepOutcome::Resolved => StepResult::resolved(events),
        crate::step::StepOutcome::Finished => StepResult::finished(events),
        crate::step::StepOutcome::AwaitingDecision(decisions) => StepResult::awaiting_decision(decisions),
    };
    super::super::injury::resolve_injuries(input, state, result)
}

pub(in crate::resolution) fn resolve_kick_foul_segment_inner(
    input: &MatchInput,
    state: &mut MatchState,
    selected_decision: Option<KickFoulDecisionKind>,
    selected_taker_id: Option<Uuid>,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    if state.phase() != MatchPhase::KickFoul {
        return Err(EngineError::InvalidTransition(
            "no Kick Foul is ready".into(),
        ));
    }
    let team_id = state.possessor_team_id();
    let is_home = team_id == input.home().team_id();
    let (offense, defense) = if is_home {
        (input.home(), input.away())
    } else {
        (input.away(), input.home())
    };
    if offense.manager().is_human_controlled() && selected_decision.is_none() {
        return Ok(StepResult::awaiting_decision(vec![
            RequiredManagerDecision::KickFoulDecision { team_id },
        ]));
    }
    let ratings = RatingIndex::new(input, state);
    let active_players = if is_home {
        state.home().active_player_ids()
    } else {
        state.away().active_player_ids()
    };
    let taker_id = match selected_taker_id {
        Some(player_id) if active_players.contains(&player_id) => player_id,
        Some(_) => {
            return Err(EngineError::InvalidInput(
                "Kick Foul taker must be active for the awarded team".into(),
            ));
        }
        None => select_kicker(&ratings, offense, None)?,
    };
    let artrine_id = if is_home {
        state.home().artrine_id()
    } else {
        state.away().artrine_id()
    };
    let receiver_id = if artrine_id != taker_id {
        artrine_id
    } else {
        offense
            .lineup()
            .assignments()
            .iter()
            .find(|assignment| {
                let player_id = ratings.slot_player_id(offense, assignment.player_id());
                player_id != taker_id && active_players.contains(&player_id)
            })
            .map(|assignment| ratings.slot_player_id(offense, assignment.player_id()))
            .ok_or_else(|| EngineError::InvalidInput("Kick Foul has no receiver".into()))?
    };
    let mut next = state.clone();
    let pending = next.pending_call_outcome();
    let decision = selected_decision.unwrap_or_else(|| default_decision(next.rng_mut()));
    let position = next.possession().ball_position_mirim();
    let pitch_length = input.pitch().length_mirim();
    let distance_to_goal = if is_home {
        pitch_length - position
    } else {
        position
    };
    let post = (decision == KickFoulDecisionKind::Shoot)
        .then(|| sample_post(distance_to_goal / pitch_length, next.rng_mut()));
    let sample = sample_attempt(
        &ratings,
        offense,
        defense,
        taker_id,
        receiver_id,
        decision,
        post,
        distance_to_goal / pitch_length,
        next.rng_mut(),
    )?;
    let remaining = next.clock().maximum_period_seconds() - next.clock().seconds_in_period();
    if remaining <= 0.0 {
        return Err(EngineError::InvalidTransition(
            "Kick Foul cannot begin at the maximum quarter limit".into(),
        ));
    }
    let duration = sample.duration_seconds.min(remaining);
    let mut events = Vec::with_capacity(6);
    let extension =
        next.clock().seconds_in_period() + duration - next.clock().period_limit_seconds();
    if extension > 0.0 {
        next.grant_added_time(next.clock().added_seconds() + extension)?;
        if next.clock().period() % 2 == 0 {
            events.push(
                next.emit(MatchEvent::AddedTimeAwarded(AddedTimeAwarded::new(
                    next.clock().period(),
                    extension,
                    0,
                    0,
                    0,
                    0,
                    0,
                    1,
                    0.0,
                )))?,
            );
        }
    }
    events.push(
        next.emit(MatchEvent::KickFoulDecisionMade(KickFoulDecisionMade::new(
            taker_id, decision,
        )))?,
    );
    next.begin_kick_foul_attempt()?;
    next.advance_kick_foul_time(duration)?;
    events.push(next.emit(MatchEvent::PossessionTimeRecorded(
        PossessionTimeRecorded::new(team_id, duration),
    ))?);
    match post {
        Some(target) => resolve_shot(
            &mut next,
            &mut events,
            team_id,
            defense,
            &ratings,
            input.pitch(),
            taker_id,
            artrine_id,
            target,
            sample.converted,
            sample.out_of_bounds,
            sample.defense_recovers,
            position,
            if is_home { pitch_length } else { 0.0 },
        )?,
        None => resolve_launch(
            &mut next,
            &mut events,
            team_id,
            defense.team_id(),
            taker_id,
            receiver_id,
            sample.converted && duration >= IMMEDIATE_POSSESSION_CONTROL_SECONDS,
            sample.out_of_bounds,
            sample.defense_recovers,
            position,
            if is_home { 1.0 } else { -1.0 },
            sample.distance_mirim,
            pitch_length,
            matches!(
                decision,
                KickFoulDecisionKind::Cross | KickFoulDecisionKind::LongLaunch
            ),
        )?,
    }
    if next.phase() == MatchPhase::Stopped {
        if let Some(outcome) = pending {
            emit_down_advanced(&mut next, &mut events, outcome, position)?;
        }
    }
    *state = next;
    Ok(StepResult::resolved(events))
}

fn resolve_shot(
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
    team_id: Uuid,
    defense: &crate::input::TeamInput,
    ratings: &RatingIndex,
    pitch: arlo_domain::Pitch,
    taker_id: Uuid,
    artrine_id: Uuid,
    post: ScoringPost,
    converted: bool,
    out_of_bounds: bool,
    defense_recovers: bool,
    position: f64,
    goal_line: f64,
) -> EngineResult<()> {
    let defense_id = defense.team_id();
    if converted {
        let kind = match post {
            ScoringPost::Goalpost => ScoreKind::KickFoulGoalPoint,
            ScoringPost::Fieldpost => ScoreKind::KickFoulFieldPoint,
        };
        state.apply_score(team_id, kind)?;
        let event = match post {
            ScoringPost::Goalpost => {
                MatchEvent::GoalPoint(GoalPointScored::new(team_id, taker_id, artrine_id, None, 0))
            }
            ScoringPost::Fieldpost => {
                MatchEvent::FieldPoint(FieldPointScored::new(team_id, taker_id, 0.0, 0))
            }
        };
        events.push(state.emit(event)?);
        return Ok(());
    }
    state.continue_after_kick_foul()?;
    events.push(
        state.emit(MatchEvent::ScoringAttemptMissed(ScoringAttemptMissed::new(
            team_id, taker_id, post,
        )))?,
    );
    if out_of_bounds {
        state.resolve_out(defense_id, position)?;
        events.push(state.emit(MatchEvent::OutOfBounds(OutOfBounds::new(
            team_id,
            Some(taker_id),
            false,
        )))?);
        events.push(state.emit(MatchEvent::Turnover(Turnover::new(
            team_id, defense_id, None, None, false,
        )))?);
    } else {
        let recovery_team_id = if defense_recovers {
            defense_id
        } else {
            team_id
        };
        if defense_recovers {
            let recovery = super::super::goalguard::resolve_recovery(
                ratings, defense, pitch, goal_line, state, events,
            )?;
            state.recover_missed_shot(team_id, recovery_team_id, recovery.position_mirim)?;
            state.set_carrier(recovery.player_id)?;
            events.push(state.emit(MatchEvent::Turnover(Turnover::new(
                team_id, defense_id, Some(recovery.player_id), Some(taker_id), true,
            )))?);
        } else {
            let position = if goal_line > pitch.length_mirim() / 2.0 {
                (goal_line - 2.0).max(0.0)
            } else {
                (goal_line + 2.0).min(pitch.length_mirim())
            };
            state.recover_missed_shot(team_id, recovery_team_id, position)?;
        }
    }
    Ok(())
}

fn resolve_launch(
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
    team_id: Uuid,
    defense_id: Uuid,
    taker_id: Uuid,
    receiver_id: Uuid,
    caught: bool,
    out_of_bounds: bool,
    defense_recovers: bool,
    position: f64,
    direction: f64,
    distance_mirim: f64,
    pitch_length: f64,
    is_aerial: bool,
) -> EngineResult<()> {
    state.continue_after_kick_foul()?;
    events.push(
        state.emit(MatchEvent::ReceptionResolved(ReceptionResolved::new(
            receiver_id,
            taker_id,
            caught && !out_of_bounds,
            is_aerial,
        )))?,
    );
    if out_of_bounds {
        state.resolve_out(defense_id, position)?;
        events.push(state.emit(MatchEvent::OutOfBounds(OutOfBounds::new(
            team_id,
            Some(taker_id),
            false,
        )))?);
        events.push(state.emit(MatchEvent::Turnover(Turnover::new(
            team_id, defense_id, None, None, false,
        )))?);
        return Ok(());
    }
    let end = (position + direction * distance_mirim).clamp(0.0, pitch_length);
    state.move_live_ball(end)?;
    if caught {
        state.set_carrier(receiver_id)?;
        events.push(state.emit(MatchEvent::PassCompleted(PassCompleted::new(
            taker_id,
            receiver_id,
            is_aerial,
            distance_mirim,
        )))?);
    } else if defense_recovers {
        state.turnover(defense_id)?;
        events.push(state.emit(MatchEvent::Turnover(Turnover::new(
            team_id,
            defense_id,
            None,
            Some(taker_id),
            true,
        )))?);
    }
    Ok(())
}
