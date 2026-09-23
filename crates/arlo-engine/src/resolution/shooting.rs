use super::down::emit_down_advanced;
use super::ratings::RatingIndex;
use super::shooting_model::sample_regular_shot;
use crate::error::EngineResult;
use crate::input::{MatchInput, TeamInput};
use crate::state::{MatchState, ScoreKind};
use arlo_domain::Position;
use arlo_events::{
    FieldPointScored, GoalPointScored, MatchEvent, MatchEventEnvelope, OutOfBounds,
    ScoringAttemptMissed, ScoringPost, Turnover,
};
use arlo_tactics::PlayCall;

pub(super) fn resolve_regular_attempt(
    input: &MatchInput,
    ratings: &RatingIndex,
    offense: &TeamInput,
    defense: &TeamInput,
    is_home: bool,
    selected_play_call: Option<&PlayCall>,
    state: &mut MatchState,
    events: &mut Vec<MatchEventEnvelope>,
) -> EngineResult<bool> {
    let team_id = offense.team_id();
    let shooter_id = if is_home {
        state.home().artrine_id()
    } else {
        state.away().artrine_id()
    };
    let drives = if is_home {
        state.home().drive_progress().completed_drives()
    } else {
        state.away().drive_progress().completed_drives()
    };
    let position = state.possession().ball_position_mirim();
    let pitch_length = input.pitch().length_mirim();
    let distance = if is_home {
        pitch_length - position
    } else {
        position
    };
    let Some(sample) = sample_regular_shot(
        ratings,
        offense,
        defense,
        selected_play_call,
        distance,
        pitch_length,
        drives > 0,
        state.rng_mut(),
    )?
    else {
        return Ok(false);
    };
    if sample.converted {
        let pending = state.pending_call_outcome();
        let territory_advance = if state.series().team_id() == team_id {
            pending
                .map(|outcome| outcome.total_advance_mirim)
                .unwrap_or(state.series().valid_advance_mirim())
        } else {
            0.0
        };
        let kind = match sample.post {
            ScoringPost::Goalpost => ScoreKind::RegularGoalPoint,
            ScoringPost::Fieldpost => ScoreKind::RegularFieldPoint,
        };
        state.apply_score(team_id, kind)?;
        let event = match sample.post {
            ScoringPost::Goalpost => MatchEvent::GoalPoint(GoalPointScored::new(
                team_id, shooter_id, shooter_id, None, drives,
            )),
            ScoringPost::Fieldpost => MatchEvent::FieldPoint(FieldPointScored::new(
                team_id,
                shooter_id,
                territory_advance,
                drives,
            )),
        };
        events.push(state.emit(event)?);
        if let Some(outcome) = pending {
            emit_down_advanced(state, events, outcome, position)?;
        }
    } else if sample.out_of_bounds {
        let pending = state.pending_call_outcome();
        state.resolve_out(defense.team_id(), position)?;
        events.push(
            state.emit(MatchEvent::ScoringAttemptMissed(ScoringAttemptMissed::new(
                team_id,
                shooter_id,
                sample.post,
            )))?,
        );
        events.push(state.emit(MatchEvent::OutOfBounds(OutOfBounds::new(
            team_id,
            Some(shooter_id),
            false,
        )))?);
        events.push(state.emit(MatchEvent::Turnover(Turnover::new(
            team_id,
            defense.team_id(),
            None,
            None,
            false,
        )))?);
        if let Some(outcome) = pending {
            emit_down_advanced(state, events, outcome, position)?;
        }
    } else {
        let recovery_team_id = if sample.defense_recovers {
            defense.team_id()
        } else {
            team_id
        };
        let recovery_position = if is_home {
            (pitch_length - 2.0).max(0.0)
        } else {
            2.0_f64.min(pitch_length)
        };
        state.recover_missed_shot(team_id, recovery_team_id, recovery_position)?;
        events.push(
            state.emit(MatchEvent::ScoringAttemptMissed(ScoringAttemptMissed::new(
                team_id,
                shooter_id,
                sample.post,
            )))?,
        );
        if sample.defense_recovers {
            let recovering_goalguard_id = defense
                .lineup()
                .assignments()
                .iter()
                .find(|assignment| assignment.position() == Position::Goalguard)
                .map(|assignment| assignment.player_id());
            events.push(state.emit(MatchEvent::Turnover(Turnover::new(
                team_id,
                recovery_team_id,
                recovering_goalguard_id,
                None,
                true,
            )))?);
        }
    }
    Ok(true)
}
