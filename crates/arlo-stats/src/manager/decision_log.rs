use arlo_events::{MatchEvent, PlayCallCategory, SubstitutionReason};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ManagerDecisionLog {
    pub substitutions_made: u32,
    pub substitutions_by_reason: HashMap<SubstitutionReason, u32>,
    pub time_calls_used: u32,
    pub challenges_won: u32,
    pub challenges_lost: u32,
    pub tactical_profile_switches: u32,
    #[serde(default)]
    pub tactical_realignments: u32,
    #[serde(default)]
    pub prepared_plan_activations: u32,
    pub play_calls_by_category: HashMap<PlayCallCategory, u32>,
}

impl ManagerDecisionLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::TacticalPlanActivated(_) => {
                self.prepared_plan_activations += 1;
            }
            MatchEvent::SubstitutionMade(e) => {
                self.substitutions_made += 1;
                *self.substitutions_by_reason.entry(e.reason()).or_insert(0) += 1;
            }
            MatchEvent::TimeCallUsed(_) => {
                self.time_calls_used += 1;
            }
            MatchEvent::ChallengeResolved(e) => {
                if e.success() {
                    self.challenges_won += 1;
                } else {
                    self.challenges_lost += 1;
                }
            }
            MatchEvent::TacticalRealignmentMade(_) => {
                self.tactical_realignments += 1;
            }
            MatchEvent::TacticalProfileActivated(_) => {
                self.tactical_profile_switches += 1;
            }
            MatchEvent::PlayCallSelected(e) => {
                *self.play_calls_by_category.entry(e.category()).or_insert(0) += 1;
            }
            _ => {}
        }
    }

    pub fn substitutions_made(&self) -> u32 {
        self.substitutions_made
    }

    pub fn substitutions_by_reason(&self) -> &HashMap<SubstitutionReason, u32> {
        &self.substitutions_by_reason
    }

    pub fn time_calls_used(&self) -> u32 {
        self.time_calls_used
    }

    pub fn challenges_won(&self) -> u32 {
        self.challenges_won
    }

    pub fn challenges_lost(&self) -> u32 {
        self.challenges_lost
    }

    pub fn tactical_realignments(&self) -> u32 {
        self.tactical_realignments
    }

    pub fn tactical_profile_switches(&self) -> u32 {
        self.tactical_profile_switches
    }

    pub fn play_calls_by_category(&self) -> &HashMap<PlayCallCategory, u32> {
        &self.play_calls_by_category
    }
}
