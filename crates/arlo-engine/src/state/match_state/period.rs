use super::MatchState;
use crate::error::{EngineError, EngineResult};
use crate::state::MatchPhase;
use arlo_domain::sport_constants::REGULAR_PERIODS_COUNT;

impl MatchState {
    pub fn advance_playing_time(&mut self, seconds: f64) -> EngineResult<()> {
        if self.phase != MatchPhase::Live {
            return Err(EngineError::InvalidTransition(
                "playing time requires live play".into(),
            ));
        }
        self.clock = self.clock.advance(seconds)?;
        Ok(())
    }

    pub fn advance_bonus_playing_time(&mut self, seconds: f64) -> EngineResult<()> {
        if self.phase != MatchPhase::BonusPhase {
            return Err(EngineError::InvalidTransition(
                "bonus playing time requires a Bonus Phase".into(),
            ));
        }
        self.clock = self.clock.advance(seconds)?;
        Ok(())
    }

    pub fn grant_added_time(&mut self, seconds: f64) -> EngineResult<()> {
        self.clock = self.clock.grant_added_time(seconds)?;
        Ok(())
    }

    pub fn finish_quarter(&mut self) -> EngineResult<()> {
        if !matches!(self.phase, MatchPhase::Ready | MatchPhase::Stopped)
            || self.clock.seconds_in_period() < self.clock.period_limit_seconds()
        {
            return Err(EngineError::InvalidTransition(
                "quarter cannot end during open play".into(),
            ));
        }
        self.home.reset_drives();
        self.away.reset_drives();
        self.clock = self.clock.stop();
        self.phase = if self.clock.period() == REGULAR_PERIODS_COUNT {
            MatchPhase::Finished
        } else {
            MatchPhase::PeriodBreak
        };
        Ok(())
    }

    pub fn start_next_quarter(&mut self) -> EngineResult<()> {
        if self.phase != MatchPhase::PeriodBreak {
            return Err(EngineError::InvalidTransition(
                "no quarter break is active".into(),
            ));
        }
        self.clock = self.clock.next_quarter()?;
        self.home.reset_time_calls();
        self.away.reset_time_calls();
        self.phase = MatchPhase::Ready;
        Ok(())
    }
}
