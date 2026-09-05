use arlo_domain::sport_constants::{
    CHALLENGE_CALLS_PER_MATCH, OVERTIME_PERIODS_COUNT, OVERTIME_PERIOD_DURATION_MINUTES,
    PERIOD_DURATION_MINUTES, REGULAR_PERIODS_COUNT, TIME_CALLS_PER_PERIOD,
};
use arlo_events::MatchClockInstant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchClock {
    period: u32,
    seconds_in_period: f64,
    home_time_calls: u32,
    away_time_calls: u32,
    home_challenges: u32,
    away_challenges: u32,
    is_finished: bool,
}

impl MatchClock {
    pub fn new() -> Self {
        Self {
            period: 1,
            seconds_in_period: 0.0,
            home_time_calls: TIME_CALLS_PER_PERIOD,
            away_time_calls: TIME_CALLS_PER_PERIOD,
            home_challenges: CHALLENGE_CALLS_PER_MATCH,
            away_challenges: CHALLENGE_CALLS_PER_MATCH,
            is_finished: false,
        }
    }

    pub fn period(&self) -> u32 {
        self.period
    }

    pub fn seconds_in_period(&self) -> f64 {
        self.seconds_in_period
    }

    pub fn home_time_calls(&self) -> u32 {
        self.home_time_calls
    }

    pub fn away_time_calls(&self) -> u32 {
        self.away_time_calls
    }

    pub fn home_challenges(&self) -> u32 {
        self.home_challenges
    }

    pub fn away_challenges(&self) -> u32 {
        self.away_challenges
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn is_overtime(&self) -> bool {
        self.period > REGULAR_PERIODS_COUNT
    }

    pub fn period_duration_seconds(&self) -> f64 {
        if self.is_overtime() {
            (OVERTIME_PERIOD_DURATION_MINUTES * 60) as f64
        } else {
            (PERIOD_DURATION_MINUTES * 60) as f64
        }
    }

    pub fn remaining_seconds_in_period(&self) -> f64 {
        (self.period_duration_seconds() - self.seconds_in_period).max(0.0)
    }

    pub fn advance_seconds(&mut self, delta: f64) -> bool {
        if self.is_finished {
            return true;
        }
        self.seconds_in_period += delta;
        let limit = self.period_duration_seconds();
        if self.seconds_in_period >= limit {
            self.seconds_in_period = limit;
            true
        } else {
            false
        }
    }

    pub fn next_period(&mut self) -> bool {
        let max_periods = REGULAR_PERIODS_COUNT + OVERTIME_PERIODS_COUNT;
        if self.period >= max_periods {
            self.is_finished = true;
            return false;
        }
        self.period += 1;
        self.seconds_in_period = 0.0;
        self.home_time_calls = TIME_CALLS_PER_PERIOD;
        self.away_time_calls = TIME_CALLS_PER_PERIOD;
        true
    }

    pub fn finish_match(&mut self) {
        self.is_finished = true;
    }

    pub fn use_time_call(&mut self, is_home: bool) -> bool {
        let calls = if is_home {
            &mut self.home_time_calls
        } else {
            &mut self.away_time_calls
        };
        if *calls > 0 {
            *calls -= 1;
            true
        } else {
            false
        }
    }

    pub fn use_challenge(&mut self, is_home: bool, success: bool) -> bool {
        let challenges = if is_home {
            &mut self.home_challenges
        } else {
            &mut self.away_challenges
        };
        if *challenges > 0 {
            if !success {
                *challenges -= 1;
            }
            true
        } else {
            false
        }
    }

    pub fn to_instant(&self) -> MatchClockInstant {
        MatchClockInstant::new(self.period, self.seconds_in_period)
    }
}

impl Default for MatchClock {
    fn default() -> Self {
        Self::new()
    }
}
