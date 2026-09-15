use arlo_domain::MatchFormatRules;
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
    regulation_periods: u32,
    regulation_period_duration_seconds: f64,
    allows_overtime: bool,
    overtime_periods: u32,
    overtime_period_duration_seconds: f64,
    time_calls_per_period: u32,
    added_time_seconds: f64,
    added_time_decided: bool,
}

impl MatchClock {
    pub fn new(format_rules: &MatchFormatRules) -> Self {
        Self {
            period: 1,
            seconds_in_period: 0.0,
            home_time_calls: format_rules.time_calls_per_period(),
            away_time_calls: format_rules.time_calls_per_period(),
            home_challenges: format_rules.challenges_per_match(),
            away_challenges: format_rules.challenges_per_match(),
            is_finished: false,
            regulation_periods: format_rules.regulation_periods(),
            regulation_period_duration_seconds: (format_rules.regulation_period_duration_minutes()
                * 60) as f64,
            allows_overtime: format_rules.allows_overtime(),
            overtime_periods: format_rules.overtime_periods(),
            overtime_period_duration_seconds: (format_rules.overtime_period_duration_minutes() * 60)
                as f64,
            time_calls_per_period: format_rules.time_calls_per_period(),
            added_time_seconds: 0.0,
            added_time_decided: false,
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
        self.period > self.regulation_periods
    }

    pub fn period_duration_seconds(&self) -> f64 {
        let base = if self.is_overtime() {
            self.overtime_period_duration_seconds
        } else {
            self.regulation_period_duration_seconds
        };
        base + self.added_time_seconds
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
        let max_periods = self.regulation_periods
            + if self.allows_overtime {
                self.overtime_periods
            } else {
                0
            };
        if self.period >= max_periods {
            self.is_finished = true;
            return false;
        }
        self.period += 1;
        self.seconds_in_period = 0.0;
        self.home_time_calls = self.time_calls_per_period;
        self.away_time_calls = self.time_calls_per_period;
        self.added_time_seconds = 0.0;
        self.added_time_decided = false;
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

    pub fn added_time_seconds(&self) -> f64 {
        self.added_time_seconds
    }

    pub fn is_added_time_decided(&self) -> bool {
        self.added_time_decided
    }

    pub fn apply_added_time(&mut self, seconds: f64) {
        self.added_time_seconds += seconds;
    }

    pub fn mark_added_time_decided(&mut self) {
        self.added_time_decided = true;
    }
}

impl Default for MatchClock {
    fn default() -> Self {
        Self::new(&MatchFormatRules::default_ruleset())
    }
}