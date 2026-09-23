use super::{MatchState, SuspendedRestart};
use crate::error::{EngineError, EngineResult};
use crate::state::{MatchPhase, PossessionState, ScoreKind, SeriesState};
use arlo_domain::sport_constants::GOAL_POINT_REQUIRED_DRIVES;
use uuid::Uuid;

impl MatchState {
    pub fn award_kick_foul(&mut self, kicker_team_id: Uuid) -> EngineResult<()> {
        if self.phase != MatchPhase::Live {
            return Err(EngineError::InvalidTransition(
                "Kick Foul requires live play".into(),
            ));
        }
        self.team(kicker_team_id)?;
        if kicker_team_id != self.possession.possessor_team_id()
            && self.possession.possessor_team_id() == self.series.team_id()
        {
            self.suspended_restart = Some(SuspendedRestart {
                team_id: self.series.team_id(),
                series: self.series,
                position_mirim: self.possession.ball_position_mirim(),
            });
            self.series = self.series.suspend_after_turnover();
        }
        self.possession = self.possession.with_possessor(kicker_team_id);
        self.clock = self.clock.stop();
        self.phase = MatchPhase::KickFoul;
        Ok(())
    }

    pub fn apply_score(&mut self, team_id: Uuid, kind: ScoreKind) -> EngineResult<u32> {
        if self.possession.possessor_team_id() != team_id {
            return Err(EngineError::InvalidTransition(
                "scoring team lacks possession".into(),
            ));
        }
        let correct_phase = match kind {
            ScoreKind::RegularGoalPoint | ScoreKind::RegularFieldPoint => {
                self.phase == MatchPhase::Live
            }
            ScoreKind::KickFoulGoalPoint | ScoreKind::KickFoulFieldPoint => {
                self.phase == MatchPhase::KickFoul
            }
            ScoreKind::BonusFieldpost | ScoreKind::BonusGoalpost => {
                self.phase == MatchPhase::BonusPhase
            }
        };
        if !correct_phase {
            return Err(EngineError::InvalidTransition(
                "score is unavailable in this phase".into(),
            ));
        }
        if kind == ScoreKind::RegularGoalPoint
            && self.team(team_id)?.drive_progress().completed_drives() < GOAL_POINT_REQUIRED_DRIVES
        {
            return Err(EngineError::InvalidTransition(
                "regular Goal Point requires a Drive".into(),
            ));
        }
        let next_score = self.team(team_id)?.score().apply(kind)?;
        let bonus_phase = kind.opens_bonus_phase()
            && self.clock.seconds_in_period() < self.clock.maximum_period_seconds();
        let restart = if bonus_phase {
            None
        } else {
            let recipient = self.opponent_id(team_id)?;
            Some(self.restart_for(recipient)?)
        };

        self.team_mut(team_id)?.replace_score(next_score);
        self.home.reset_drives();
        self.away.reset_drives();
        self.pending_call_outcome = None;
        if !bonus_phase {
            self.clock = self.clock.stop();
        }
        if let Some((possession, series)) = restart {
            self.possession = possession;
            self.series = series;
            self.suspended_restart = None;
            self.phase = MatchPhase::Stopped;
        } else {
            self.phase = MatchPhase::BonusPhase;
        }
        Ok(kind.points())
    }

    pub fn finish_bonus_phase_without_score(&mut self) -> EngineResult<()> {
        if self.phase != MatchPhase::BonusPhase {
            return Err(EngineError::InvalidTransition(
                "no Bonus Phase is active".into(),
            ));
        }
        let recipient = self.opponent_id(self.possession.possessor_team_id())?;
        let (possession, series) = self.restart_for(recipient)?;
        self.possession = possession;
        self.series = series;
        self.suspended_restart = None;
        self.clock = self.clock.stop();
        self.phase = MatchPhase::Stopped;
        Ok(())
    }

    fn restart_for(&self, recipient: Uuid) -> EngineResult<(PossessionState, SeriesState)> {
        let (position, series) = match self.suspended_restart {
            Some(saved) if saved.team_id == recipient => {
                (saved.position_mirim, saved.series.resume_for_new_call())
            }
            _ => {
                let midfield = self.pitch_length_mirim / 2.0;
                (
                    midfield,
                    SeriesState::new(recipient, midfield, self.pitch_length_mirim)?,
                )
            }
        };
        let possession =
            self.possession
                .award_next_call(recipient, position, self.pitch_length_mirim)?;
        Ok((possession, series))
    }
}
