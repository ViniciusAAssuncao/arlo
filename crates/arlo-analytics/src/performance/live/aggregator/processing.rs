use super::PlayerPerformanceAggregator;
use crate::performance::observation::PerformanceObservation;
use arlo_events::{MatchEvent, MatchEventEnvelope};

impl PlayerPerformanceAggregator {
    pub fn process_envelope(&mut self, envelope: &MatchEventEnvelope) {
        let current_seconds = envelope.clock().total_elapsed_seconds();
        self.advance_time(current_seconds);
        self.last_clock = envelope.clock();
        self.sequence_counter = envelope.sequence_number();

        self.record_position_time(envelope.event());

        let observations = self.translator.translate_envelope(envelope);
        self.apply_observations(&observations);
        self.inspect_event_for_roster_updates(envelope.event());

        if self.config.record_snapshots_automatically() {
            let snapshot = self.create_snapshot(envelope.sequence_number(), envelope.clock());
            self.history.push(snapshot);
        }
    }

    pub fn process_event(&mut self, event: &MatchEvent) {
        self.sequence_counter = self.sequence_counter.saturating_add(1);
        self.record_position_time(event);
        let observations = self.translator.translate_event(event, self.last_clock);
        self.apply_observations(&observations);
        self.inspect_event_for_roster_updates(event);

        if self.config.record_snapshots_automatically() {
            let snapshot = self.create_snapshot(self.sequence_counter, self.last_clock);
            self.history.push(snapshot);
        }
    }

    pub fn handle_envelopes<'a>(
        &mut self,
        envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
    ) {
        for envelope in envelopes {
            self.process_envelope(envelope);
        }
    }

    pub fn advance_time(&mut self, clock_seconds: f64) {
        if clock_seconds > self.last_clock_seconds {
            let delta = clock_seconds - self.last_clock_seconds;
            for state in self.players.values_mut() {
                state.advance_time(delta, &self.config);
            }
            self.last_clock_seconds = clock_seconds;
        }
    }

    fn record_position_time(&mut self, event: &MatchEvent) {
        if let MatchEvent::PossessionTimeRecorded(possession) = event {
            for state in self.players.values_mut() {
                state.record_possession_time(
                    possession.team_id(),
                    possession.live_duration_seconds(),
                );
            }
        }
    }

    pub fn apply_observations(&mut self, observations: &[PerformanceObservation]) {
        for observation in observations {
            self.ensure_player_exists(observation.player_id(), observation.team_id());
            if let Some(state) = self.players.get_mut(&observation.player_id()) {
                state.apply_observation(observation, &self.config);
            }
        }
    }
}
