use super::PlayerPerformanceAggregator;
use crate::performance::live::replay;
use arlo_events::{InMemorySink, MatchClockInstant, MatchEventEnvelope};

impl PlayerPerformanceAggregator {
    pub fn reset_state(&mut self) {
        self.translator.reset();
        self.players.clear();
        self.history.clear();
        self.last_clock = MatchClockInstant::zero();
        self.last_clock_seconds = 0.0;
        self.sequence_counter = 0;
        self.finalization.clear();

        let seeds = self.initial_seeds.clone();
        for seed in &seeds {
            self.apply_seed(seed);
        }
    }

    pub fn is_initial_state(&self) -> bool {
        self.sequence_counter == 0
            && self.last_clock_seconds == 0.0
            && self.history.is_empty()
            && !self.finalization.is_finalized()
            && self
                .players
                .values()
                .all(|player| player.effective_opportunities() == 0 && player.seconds_played() == 0.0)
    }

    pub fn replay<'a>(
        &mut self,
        envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
    ) {
        self.reset_state();
        self.handle_envelopes(envelopes);
    }

    pub fn replay_from_sink(&mut self, sink: &InMemorySink) {
        self.replay(sink.events());
    }

    pub fn invalidate_and_replay<'a>(
        &mut self,
        first_sequence: u64,
        last_sequence: u64,
        envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
    ) {
        let surviving =
            replay::filter_surviving_envelopes(first_sequence, last_sequence, envelopes);
        self.replay(surviving);
    }
}
