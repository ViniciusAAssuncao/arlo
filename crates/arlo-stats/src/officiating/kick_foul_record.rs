use crate::aggregator::StatAggregator;
use crate::officiating::keyed_registry::{KeyedStat, KeyedStatRegistry};
use crate::snapshot::{IntoSnapshot, PlayerKickFoulSnapshot};
use arlo_domain::KickFoulDecisionKind;
use arlo_events::MatchEvent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PlayerKickFoulStats {
    pub player_id: Uuid,
    pub kick_foul_takes: u32,
    pub decisions_by_kind: HashMap<KickFoulDecisionKind, u32>,
}

impl PlayerKickFoulStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            kick_foul_takes: 0,
            decisions_by_kind: HashMap::new(),
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn kick_foul_takes(&self) -> u32 {
        self.kick_foul_takes
    }

    pub fn decisions_by_kind(&self) -> &HashMap<KickFoulDecisionKind, u32> {
        &self.decisions_by_kind
    }

    pub fn count_for_kind(&self, kind: KickFoulDecisionKind) -> u32 {
        self.decisions_by_kind.get(&kind).copied().unwrap_or(0)
    }

    pub fn record_decision(&mut self, decision: KickFoulDecisionKind) {
        self.kick_foul_takes += 1;
        *self.decisions_by_kind.entry(decision).or_insert(0) += 1;
    }
}

impl KeyedStat for PlayerKickFoulStats {
    fn new_for(player_id: Uuid) -> Self {
        Self::new(player_id)
    }
}

impl IntoSnapshot for PlayerKickFoulStats {
    type Snapshot = PlayerKickFoulSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerKickFoulSnapshot {
            player_id: self.player_id,
            kick_foul_takes: self.kick_foul_takes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerKickFoulAggregator {
    registry: KeyedStatRegistry<PlayerKickFoulStats>,
}

impl PlayerKickFoulAggregator {
    pub fn new() -> Self {
        Self {
            registry: KeyedStatRegistry::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerKickFoulStats> {
        self.registry.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerKickFoulStats {
        self.registry.get_or_default(player_id)
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerKickFoulStats> {
        self.registry.all_stats()
    }

    pub fn record_decision(&mut self, player_id: Uuid, decision: KickFoulDecisionKind) {
        let stats = self.registry.entry_or_default(player_id);
        stats.record_decision(decision);
    }
}

impl IntoSnapshot for PlayerKickFoulAggregator {
    type Snapshot = HashMap<Uuid, PlayerKickFoulSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.registry
            .all_stats()
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerKickFoulAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::KickFoulDecisionMade(e) = event {
            self.record_decision(e.taker_id(), e.decision());
        }
    }

    fn reset(&mut self) {
        self.registry.clear();
    }
}
