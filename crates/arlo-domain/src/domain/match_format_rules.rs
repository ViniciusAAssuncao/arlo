use crate::domain::invariant_violation::InvariantViolation;
use crate::domain::rule::{Rule, RuleCategory};
use crate::domain::sport_constants::{
    CHALLENGE_CALLS_PER_MATCH, OVERTIME_PERIODS_COUNT, OVERTIME_PERIOD_DURATION_MINUTES,
    PERIOD_DURATION_MINUTES, REGULAR_PERIODS_COUNT, TIME_CALLS_PER_PERIOD,
    TIME_CALL_DURATION_MINUTES,
};
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchFormatRules {
    regulation_periods: u32,
    regulation_period_duration_minutes: u32,
    allows_overtime: bool,
    overtime_periods: u32,
    overtime_period_duration_minutes: u32,
    time_calls_per_period: u32,
    time_call_duration_minutes: u32,
    challenges_per_match: u32,
}

impl MatchFormatRules {
    pub fn new(
        regulation_periods: u32,
        regulation_period_duration_minutes: u32,
        allows_overtime: bool,
        overtime_periods: u32,
        overtime_period_duration_minutes: u32,
        time_calls_per_period: u32,
        time_call_duration_minutes: u32,
        challenges_per_match: u32,
    ) -> Self {
        Self {
            regulation_periods,
            regulation_period_duration_minutes,
            allows_overtime,
            overtime_periods,
            overtime_period_duration_minutes,
            time_calls_per_period,
            time_call_duration_minutes,
            challenges_per_match,
        }
    }

    pub fn default_ruleset() -> Self {
        Self {
            regulation_periods: REGULAR_PERIODS_COUNT,
            regulation_period_duration_minutes: PERIOD_DURATION_MINUTES,
            allows_overtime: false,
            overtime_periods: OVERTIME_PERIODS_COUNT,
            overtime_period_duration_minutes: OVERTIME_PERIOD_DURATION_MINUTES,
            time_calls_per_period: TIME_CALLS_PER_PERIOD,
            time_call_duration_minutes: TIME_CALL_DURATION_MINUTES,
            challenges_per_match: CHALLENGE_CALLS_PER_MATCH,
        }
    }

    pub fn from_rules(rules: &[Rule]) -> DomainResult<Self> {
        let mut format = Self::default_ruleset();

        for rule in rules {
            match rule.category() {
                RuleCategory::TieBreaker => match rule.rule_key() {
                    "allows_overtime" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<bool>().map_err(|_| DomainError::InvalidInvariant {
                            field: "allows_overtime".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        format.allows_overtime = parsed;
                    }
                    "overtime_periods" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<u32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "overtime_periods".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        format.overtime_periods = parsed;
                    }
                    "overtime_period_duration_minutes" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<u32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "overtime_period_duration_minutes".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        format.overtime_period_duration_minutes = parsed;
                    }
                    _ => {}
                },
                RuleCategory::Phases => match rule.rule_key() {
                    "regulation_periods" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<u32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "regulation_periods".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        format.regulation_periods = parsed;
                    }
                    "regulation_period_duration_minutes" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<u32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "regulation_period_duration_minutes".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        format.regulation_period_duration_minutes = parsed;
                    }
                    "time_calls_per_period" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<u32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "time_calls_per_period".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        format.time_calls_per_period = parsed;
                    }
                    "time_call_duration_minutes" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<u32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "time_call_duration_minutes".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        format.time_call_duration_minutes = parsed;
                    }
                    "challenges_per_match" => {
                        let val = rule.value().trim();
                        let parsed = val.parse::<u32>().map_err(|_| DomainError::InvalidInvariant {
                            field: "challenges_per_match".to_string(),
                            violation: InvariantViolation::UnexpectedValue,
                        })?;
                        format.challenges_per_match = parsed;
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(format)
    }

    pub fn total_periods(&self) -> u32 {
        self.regulation_periods + if self.allows_overtime { self.overtime_periods } else { 0 }
    }

    pub fn regulation_periods(&self) -> u32 {
        self.regulation_periods
    }

    pub fn regulation_period_duration_minutes(&self) -> u32 {
        self.regulation_period_duration_minutes
    }

    pub fn allows_overtime(&self) -> bool {
        self.allows_overtime
    }

    pub fn overtime_periods(&self) -> u32 {
        self.overtime_periods
    }

    pub fn overtime_period_duration_minutes(&self) -> u32 {
        self.overtime_period_duration_minutes
    }

    pub fn time_calls_per_period(&self) -> u32 {
        self.time_calls_per_period
    }

    pub fn time_call_duration_minutes(&self) -> u32 {
        self.time_call_duration_minutes
    }

    pub fn challenges_per_match(&self) -> u32 {
        self.challenges_per_match
    }
}

impl Default for MatchFormatRules {
    fn default() -> Self {
        Self::default_ruleset()
    }
}