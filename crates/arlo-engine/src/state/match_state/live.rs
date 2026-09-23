use super::{MatchState, SuspendedRestart};
use crate::error::{EngineError, EngineResult};
use crate::state::{MatchPhase, SeriesAdvance, SeriesOut, SeriesState};
use uuid::Uuid;

impl MatchState {
    pub fn begin_call_to_action(&mut self) -> EngineResult<()> {
        if !matches!(self.phase, MatchPhase::Ready | MatchPhase::Stopped)
            || self.possession.possessor_team_id() != self.possession.next_call_team_id()
            || self.series.team_id() != self.possession.next_call_team_id()
        {
            return Err(EngineError::InvalidTransition(
                "no Call-to-Action is ready".into(),
            ));
        }
        self.clock = self.clock.start()?;
        self.pending_call_outcome = None;
        self.phase = MatchPhase::Live;
        Ok(())
    }

    pub fn record_valid_advance(
        &mut self,
        gain_mirim: f64,
        end_mirim: f64,
    ) -> EngineResult<SeriesAdvance> {
        if self.phase != MatchPhase::Live
            || self.possession.possessor_team_id() != self.series.team_id()
        {
            return Err(EngineError::InvalidTransition(
                "no active series can gain mirins".into(),
            ));
        }
        let (series, result) =
            self.series
                .record_valid_advance(gain_mirim, end_mirim, self.pitch_length_mirim)?;
        let possession = self
            .possession
            .with_ball_position(end_mirim, self.pitch_length_mirim)?;
        self.series = series;
        self.possession = possession;
        if matches!(result, SeriesAdvance::FirstDown) {
            self.team_mut(self.series.team_id())?.reset_series_drives();
        }
        Ok(result)
    }

    pub fn record_artro(
        &mut self,
        team_id: Uuid,
        player_id: Uuid,
        control_seconds: f64,
    ) -> EngineResult<Option<u32>> {
        if self.phase != MatchPhase::Live || self.possession.possessor_team_id() != team_id {
            return Err(EngineError::InvalidTransition(
                "Artro requires live team possession".into(),
            ));
        }
        self.team_mut(team_id)?
            .record_artro(player_id, control_seconds)
    }

    pub fn turnover(&mut self, recipient_team_id: Uuid) -> EngineResult<()> {
        if self.phase != MatchPhase::Live {
            return Err(EngineError::InvalidTransition(
                "turnover requires live play".into(),
            ));
        }
        self.team(recipient_team_id)?;
        let previous_possessor = self.possession.possessor_team_id();
        let possession = self.possession.turnover(recipient_team_id)?;
        if previous_possessor == self.series.team_id() {
            self.suspended_restart = Some(SuspendedRestart {
                team_id: previous_possessor,
                series: self.series,
                position_mirim: self.possession.ball_position_mirim(),
            });
            self.series = self.series.suspend_after_turnover();
        } else if recipient_team_id == self.series.team_id() {
            self.series = self.series.resume_after_recovery();
        }
        self.possession = possession;
        Ok(())
    }

    pub fn recover_missed_shot(
        &mut self,
        shooting_team_id: Uuid,
        recovering_team_id: Uuid,
        recovery_position_mirim: f64,
    ) -> EngineResult<()> {
        if self.phase != MatchPhase::Live || self.possession.possessor_team_id() != shooting_team_id
        {
            return Err(EngineError::InvalidTransition(
                "missed shot requires live possession by the shooting team".into(),
            ));
        }
        self.team(recovering_team_id)?;
        let possession = self
            .possession
            .with_ball_position(recovery_position_mirim, self.pitch_length_mirim)?;
        if recovering_team_id != shooting_team_id {
            self.turnover(recovering_team_id)?;
        }
        self.possession = possession.with_possessor(recovering_team_id);
        Ok(())
    }

    pub fn move_live_ball(&mut self, position_mirim: f64) -> EngineResult<()> {
        if self.phase != MatchPhase::Live {
            return Err(EngineError::InvalidTransition(
                "ball movement requires live play".into(),
            ));
        }
        self.possession = self
            .possession
            .with_ball_position(position_mirim, self.pitch_length_mirim)?;
        Ok(())
    }

    pub fn resolve_time_call(
        &mut self,
        requesting_team_id: Uuid,
        time_calls_per_period: u32,
    ) -> EngineResult<(SeriesOut, u32)> {
        if self.phase != MatchPhase::Live
            || self.possession.possessor_team_id() != requesting_team_id
        {
            return Err(EngineError::InvalidTransition(
                "Time Call requires live possession by the requesting team".into(),
            ));
        }
        let used = self.team(requesting_team_id)?.time_calls_used_in_period();
        if used >= time_calls_per_period {
            return Err(EngineError::InvalidTransition(
                "no Time Calls remain in this period".into(),
            ));
        }
        let position = self.possession.ball_position_mirim();
        let result = self.resolve_out(requesting_team_id, position)?;
        self.team_mut(requesting_team_id)?.record_time_call();
        Ok((result, time_calls_per_period - used - 1))
    }

    pub fn resolve_out(
        &mut self,
        requested_next_team_id: Uuid,
        end_mirim: f64,
    ) -> EngineResult<SeriesOut> {
        if self.phase != MatchPhase::Live {
            return Err(EngineError::InvalidTransition(
                "Out requires live play".into(),
            ));
        }
        self.team(requested_next_team_id)?;
        let (next_team_id, series, result) = if requested_next_team_id == self.series.team_id()
            && requested_next_team_id == self.possession.possessor_team_id()
        {
            let (continued, outcome) = self.series.advance_after_out();
            if outcome == SeriesOut::TurnoverOnDowns {
                let opponent = self.opponent_id(requested_next_team_id)?;
                (
                    opponent,
                    SeriesState::new(opponent, end_mirim, self.pitch_length_mirim)?,
                    outcome,
                )
            } else {
                (requested_next_team_id, continued, outcome)
            }
        } else {
            (
                requested_next_team_id,
                SeriesState::new(requested_next_team_id, end_mirim, self.pitch_length_mirim)?,
                SeriesOut::NewSeries,
            )
        };
        let possession =
            self.possession
                .award_next_call(next_team_id, end_mirim, self.pitch_length_mirim)?;
        self.series = series;
        self.possession = possession;
        if matches!(result, SeriesOut::NewSeries | SeriesOut::TurnoverOnDowns) {
            self.team_mut(next_team_id)?.reset_series_drives();
        }
        self.suspended_restart = None;
        self.pending_call_outcome = None;
        self.clock = self.clock.stop();
        self.phase = MatchPhase::Stopped;
        Ok(result)
    }
}
