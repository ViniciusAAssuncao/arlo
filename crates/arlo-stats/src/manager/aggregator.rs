use super::decision_log::ManagerDecisionLog;
use crate::aggregator::StatAggregator;
use arlo_events::{MatchEvent, MatchEventEnvelope};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

pub fn aggregate_match(events: &[MatchEventEnvelope]) -> HashMap<Uuid, ManagerDecisionLog> {
    let mut logs: HashMap<Uuid, ManagerDecisionLog> = HashMap::new();

    for envelope in events {
        let event = envelope.event();
        let team_id = match event {
            MatchEvent::SubstitutionMade(e) => Some(e.team_id()),
            MatchEvent::TimeCallUsed(e) => Some(e.team_id()),
            MatchEvent::ChallengeResolved(e) => Some(e.team_id()),
            MatchEvent::TacticalProfileActivated(e) => Some(e.team_id()),
            MatchEvent::PlayCallSelected(e) => Some(e.team_id()),
            _ => None,
        };

        if let Some(id) = team_id {
            logs.entry(id).or_default().apply(event);
        }
    }

    logs
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ManagerDecisionAggregator {
    logs: HashMap<Uuid, ManagerDecisionLog>,
}

impl ManagerDecisionAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, team_id: &Uuid) -> Option<&ManagerDecisionLog> {
        self.logs.get(team_id)
    }

    pub fn get_or_default(&self, team_id: &Uuid) -> ManagerDecisionLog {
        self.logs.get(team_id).cloned().unwrap_or_default()
    }

    pub fn all_logs(&self) -> &HashMap<Uuid, ManagerDecisionLog> {
        &self.logs
    }
}

impl StatAggregator for ManagerDecisionAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        let team_id = match event {
            MatchEvent::SubstitutionMade(e) => Some(e.team_id()),
            MatchEvent::TimeCallUsed(e) => Some(e.team_id()),
            MatchEvent::ChallengeResolved(e) => Some(e.team_id()),
            MatchEvent::TacticalProfileActivated(e) => Some(e.team_id()),
            MatchEvent::PlayCallSelected(e) => Some(e.team_id()),
            _ => None,
        };

        if let Some(id) = team_id {
            self.logs.entry(id).or_default().apply(event);
        }
    }

    fn reset(&mut self) {
        self.logs.clear();
    }
}
