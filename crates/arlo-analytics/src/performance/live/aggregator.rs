mod finalization;
mod processing;
mod replay;
mod roster;
mod snapshots;

use crate::context::MatchAnalysisContext;
use crate::performance::live::config::LiveRatingConfig;
use crate::performance::live::finalizer::MatchFinalizationState;
use crate::performance::live::player_state::LivePlayerState;
use crate::performance::live::seed::InitialParticipantSeed;
use crate::performance::live::snapshot::LivePerformanceSnapshotRecord;
use crate::performance::translator::EventPerformanceTranslator;
use arlo_events::{MatchClockInstant, MatchEvent, MatchEventEnvelope};
use arlo_stats::StatAggregator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerPerformanceAggregator {
    initial_context: Option<MatchAnalysisContext>,
    initial_seeds: Vec<InitialParticipantSeed>,
    translator: EventPerformanceTranslator,
    config: LiveRatingConfig,
    players: HashMap<Uuid, LivePlayerState>,
    history: Vec<LivePerformanceSnapshotRecord>,
    last_clock: MatchClockInstant,
    last_clock_seconds: f64,
    sequence_counter: u64,
    finalization: MatchFinalizationState,
}

impl Default for PlayerPerformanceAggregator {
    fn default() -> Self {
        Self {
            initial_context: None,
            initial_seeds: Vec::new(),
            translator: EventPerformanceTranslator::new(),
            config: LiveRatingConfig::default(),
            players: HashMap::new(),
            history: Vec::new(),
            last_clock: MatchClockInstant::zero(),
            last_clock_seconds: 0.0,
            sequence_counter: 0,
            finalization: MatchFinalizationState::new(),
        }
    }
}

impl PlayerPerformanceAggregator {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: LiveRatingConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    pub fn with_context(context: MatchAnalysisContext) -> Self {
        let mut aggregator = Self {
            initial_context: Some(context.clone()),
            translator: EventPerformanceTranslator::with_context(context.clone()),
            ..Default::default()
        };
        aggregator.seed_from_context(&context);
        aggregator
    }

    pub fn set_context(&mut self, context: MatchAnalysisContext) {
        self.initial_context = Some(context.clone());
        self.initial_seeds.clear();
        self.players.clear();
        self.history.clear();
        self.last_clock = MatchClockInstant::zero();
        self.last_clock_seconds = 0.0;
        self.sequence_counter = 0;
        self.finalization.clear();
        self.translator.set_context(context.clone());
        self.seed_from_context(&context);
    }

    pub fn context(&self) -> Option<&MatchAnalysisContext> {
        self.translator.context()
    }

    pub fn initial_context(&self) -> Option<&MatchAnalysisContext> {
        self.initial_context.as_ref()
    }

    pub fn initial_seeds(&self) -> &[InitialParticipantSeed] {
        &self.initial_seeds
    }

    pub fn config(&self) -> &LiveRatingConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut LiveRatingConfig {
        &mut self.config
    }

    pub fn players(&self) -> &HashMap<Uuid, LivePlayerState> {
        &self.players
    }

    pub fn players_mut(&mut self) -> &mut HashMap<Uuid, LivePlayerState> {
        &mut self.players
    }

    pub fn player_ids(&self) -> Vec<Uuid> {
        let mut ids: Vec<Uuid> = self.players.keys().copied().collect();
        ids.sort();
        ids
    }
}

impl StatAggregator for PlayerPerformanceAggregator {
    fn handle_envelope(&mut self, envelope: &MatchEventEnvelope) {
        self.process_envelope(envelope);
    }

    fn handle_event(&mut self, event: &MatchEvent) {
        self.process_event(event);
    }

    fn reset(&mut self) {
        self.reset_state();
    }
}
