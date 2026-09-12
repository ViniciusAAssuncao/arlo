use crate::aggregator::StatAggregator;
use crate::officiating::keyed_registry::{KeyedStat, KeyedStatRegistry};
use crate::snapshot::{IntoSnapshot, PlayerFoulSnapshot};
use arlo_events::{FoulOrigin, MatchEvent};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerFoulStats {
    pub player_id: Uuid,
    pub fouls_committed: u32,
    pub fouls_drawn: u32,
    pub correct_calls_committed: u32,
    pub incorrect_calls_committed: u32,
    pub by_origin: HashMap<FoulOrigin, u32>,
}

impl PlayerFoulStats {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            fouls_committed: 0,
            fouls_drawn: 0,
            correct_calls_committed: 0,
            incorrect_calls_committed: 0,
            by_origin: HashMap::new(),
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn fouls_committed(&self) -> u32 {
        self.fouls_committed
    }

    pub fn fouls_drawn(&self) -> u32 {
        self.fouls_drawn
    }

    pub fn correct_calls_committed(&self) -> u32 {
        self.correct_calls_committed
    }

    pub fn incorrect_calls_committed(&self) -> u32 {
        self.incorrect_calls_committed
    }

    pub fn by_origin(&self) -> &HashMap<FoulOrigin, u32> {
        &self.by_origin
    }

    pub fn fouls_for_origin(&self, origin: FoulOrigin) -> u32 {
        self.by_origin.get(&origin).copied().unwrap_or(0)
    }
}

impl KeyedStat for PlayerFoulStats {
    fn new_for(player_id: Uuid) -> Self {
        Self::new(player_id)
    }
}

impl IntoSnapshot for PlayerFoulStats {
    type Snapshot = PlayerFoulSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        PlayerFoulSnapshot {
            player_id: self.player_id,
            fouls_committed: self.fouls_committed,
            fouls_drawn: self.fouls_drawn,
            correct_calls_committed: self.correct_calls_committed,
            incorrect_calls_committed: self.incorrect_calls_committed,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerFoulAggregator {
    registry: KeyedStatRegistry<PlayerFoulStats>,
}

impl PlayerFoulAggregator {
    pub fn new() -> Self {
        Self {
            registry: KeyedStatRegistry::new(),
        }
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerFoulStats> {
        self.registry.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerFoulStats {
        self.registry.get_or_default(player_id)
    }

    pub fn all_stats(&self) -> &HashMap<Uuid, PlayerFoulStats> {
        self.registry.all_stats()
    }

    pub fn record_foul_committed(
        &mut self,
        player_id: Uuid,
        origin: FoulOrigin,
        final_call_correct: bool,
    ) {
        let stats = self.registry.entry_or_default(player_id);
        stats.fouls_committed += 1;
        if final_call_correct {
            stats.correct_calls_committed += 1;
        } else {
            stats.incorrect_calls_committed += 1;
        }
        *stats.by_origin.entry(origin).or_insert(0) += 1;
    }

    pub fn record_foul_drawn(&mut self, player_id: Uuid) {
        let stats = self.registry.entry_or_default(player_id);
        stats.fouls_drawn += 1;
    }
}

impl IntoSnapshot for PlayerFoulAggregator {
    type Snapshot = HashMap<Uuid, PlayerFoulSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.registry
            .all_stats()
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerFoulAggregator {
    fn handle_event(&mut self, event: &MatchEvent) {
        if let MatchEvent::FoulRaised(e) = event {
            self.record_foul_committed(
                e.offending_player_id(),
                e.origin(),
                e.final_call_correct(),
            );
            self.record_foul_drawn(e.opposing_player_id());
        }
    }

    fn reset(&mut self) {
        self.registry.clear();
    }
}
