use crate::context::MatchAnalysisContext;
use crate::error::AnalyticsResult;
use crate::performance::live::config::LiveRatingConfig;
use crate::performance::live::player_state::LivePlayerState;
use crate::performance::live::snapshot::LivePerformanceSnapshotRecord;
use crate::performance::observation::PerformanceObservation;
use crate::performance::rating::{
    MatchOutcome, OutcomeAdjustmentPolicy, PlayerPerformanceSnapshot,
};
use crate::performance::translator::EventPerformanceTranslator;
use arlo_domain::tactics::Formation;
use arlo_domain::{Position, SlotRole};
use arlo_events::{AvailabilityStatus, MatchClockInstant, MatchEvent, MatchEventEnvelope};
use arlo_stats::StatAggregator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerPerformanceAggregator {
    translator: EventPerformanceTranslator,
    config: LiveRatingConfig,
    players: HashMap<Uuid, LivePlayerState>,
    history: Vec<LivePerformanceSnapshotRecord>,
    last_clock: MatchClockInstant,
    last_clock_seconds: f64,
    sequence_counter: u64,
}

impl Default for PlayerPerformanceAggregator {
    fn default() -> Self {
        Self {
            translator: EventPerformanceTranslator::new(),
            config: LiveRatingConfig::default(),
            players: HashMap::new(),
            history: Vec::new(),
            last_clock: MatchClockInstant::zero(),
            last_clock_seconds: 0.0,
            sequence_counter: 0,
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
        let mut agg = Self {
            translator: EventPerformanceTranslator::with_context(context.clone()),
            ..Default::default()
        };
        agg.seed_from_context(&context);
        agg
    }

    pub fn set_context(&mut self, context: MatchAnalysisContext) {
        self.seed_from_context(&context);
        self.translator.set_context(context);
    }

    pub fn context(&self) -> Option<&MatchAnalysisContext> {
        self.translator.context()
    }

    pub fn config(&self) -> &LiveRatingConfig {
        &self.config
    }

    pub fn config_mut(&mut self) -> &mut LiveRatingConfig {
        &mut self.config
    }

    pub fn seed_from_context(&mut self, context: &MatchAnalysisContext) {
        for assignment in context.all_assignments().values() {
            self.seed_participant(
                assignment.player_id(),
                assignment.team_id(),
                assignment.offensive_position(),
                assignment.defensive_position(),
                assignment.slot_role(),
                assignment.is_starter(),
            );
        }
    }

    pub fn seed_participant(
        &mut self,
        player_id: Uuid,
        team_id: Uuid,
        offensive_position: Position,
        defensive_position: Position,
        slot_role: SlotRole,
        is_active: bool,
    ) {
        let state = LivePlayerState::new(
            player_id,
            team_id,
            offensive_position,
            defensive_position,
            slot_role,
            is_active,
            &self.config,
        );
        self.players.insert(player_id, state);
    }

    pub fn seed_participants<I>(&mut self, participants: I)
    where
        I: IntoIterator<Item = (Uuid, Uuid, Position, Position, SlotRole, bool)>,
    {
        for (pid, tid, off_pos, def_pos, role, active) in participants {
            self.seed_participant(pid, tid, off_pos, def_pos, role, active);
        }
    }

    pub fn seed_team_starters(
        &mut self,
        team_id: Uuid,
        formation: &Formation,
        player_ids: &[Uuid],
    ) -> AnalyticsResult<()> {
        let slots = formation.slots();
        if player_ids.len() != slots.len() {
            return Err(crate::error::AnalyticsError::InvalidData(format!(
                "Formation expects {} players, but {} were provided",
                slots.len(),
                player_ids.len()
            )));
        }

        for (&pid, slot) in player_ids.iter().zip(slots.iter()) {
            self.seed_participant(
                pid,
                team_id,
                slot.offensive_position(),
                slot.defensive_position(),
                slot.role(),
                true,
            );
        }

        Ok(())
    }

    pub fn process_envelope(&mut self, envelope: &MatchEventEnvelope) {
        let current_seconds = envelope.clock().total_elapsed_seconds();
        self.advance_time(current_seconds);
        self.last_clock = envelope.clock();
        self.sequence_counter = envelope.sequence_number();

        let observations = self.translator.translate_envelope(envelope);
        self.apply_observations(&observations);
        self.inspect_event_for_roster_updates(envelope.event());

        if self.config.record_snapshots_automatically() {
            let snap = self.create_snapshot(envelope.sequence_number(), envelope.clock());
            self.history.push(snap);
        }
    }

    pub fn process_event(&mut self, event: &MatchEvent) {
        self.sequence_counter = self.sequence_counter.saturating_add(1);
        let observations = self.translator.translate_event(event, self.last_clock);
        self.apply_observations(&observations);
        self.inspect_event_for_roster_updates(event);

        if self.config.record_snapshots_automatically() {
            let snap = self.create_snapshot(self.sequence_counter, self.last_clock);
            self.history.push(snap);
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
            let dt = clock_seconds - self.last_clock_seconds;
            for state in self.players.values_mut() {
                state.advance_time(dt, &self.config);
            }
            self.last_clock_seconds = clock_seconds;
        }
    }

    pub fn apply_observations(&mut self, observations: &[PerformanceObservation]) {
        for obs in observations {
            self.ensure_player_exists(obs.player_id(), obs.team_id());
            if let Some(state) = self.players.get_mut(&obs.player_id()) {
                state.apply_observation(obs, &self.config);
            }
        }
    }

    pub fn handle_substitution(&mut self, player_out: Uuid, player_in: Uuid, team_id: Uuid) {
        if let Some(out_state) = self.players.get_mut(&player_out) {
            out_state.set_active(false);
        }
        if let Some(in_state) = self.players.get_mut(&player_in) {
            in_state.set_active(true);
        } else {
            let (off_pos, def_pos, role) = if let Some(out_state) = self.players.get(&player_out) {
                (
                    out_state.offensive_position(),
                    out_state.defensive_position(),
                    out_state.slot_role(),
                )
            } else {
                (Position::CenterOffense, Position::Centerback, SlotRole::Standard)
            };
            self.players.insert(
                player_in,
                LivePlayerState::new(
                    player_in,
                    team_id,
                    off_pos,
                    def_pos,
                    role,
                    true,
                    &self.config,
                ),
            );
        }
    }

    pub fn handle_availability(&mut self, player_id: Uuid, new_status: AvailabilityStatus) {
        if matches!(new_status, AvailabilityStatus::Expelled | AvailabilityStatus::Injured) {
            if let Some(state) = self.players.get_mut(&player_id) {
                state.set_active(false);
            }
        }
    }

    pub fn capture_snapshot(
        &mut self,
        sequence_number: u64,
        clock: MatchClockInstant,
    ) -> LivePerformanceSnapshotRecord {
        let snap = self.create_snapshot(sequence_number, clock);
        self.history.push(snap.clone());
        snap
    }

    pub fn create_snapshot(
        &self,
        sequence_number: u64,
        clock: MatchClockInstant,
    ) -> LivePerformanceSnapshotRecord {
        let player_snapshots = self.player_snapshots_map();
        let team_average_ratings = self.all_team_average_ratings();
        LivePerformanceSnapshotRecord::new(
            sequence_number,
            clock,
            player_snapshots,
            team_average_ratings,
        )
    }

    pub fn current_player_snapshot(&self, player_id: &Uuid) -> Option<PlayerPerformanceSnapshot> {
        self.players.get(player_id).map(|s| s.to_snapshot())
    }

    pub fn all_player_snapshots(&self) -> Vec<PlayerPerformanceSnapshot> {
        let mut list: Vec<PlayerPerformanceSnapshot> =
            self.players.values().map(|s| s.to_snapshot()).collect();
        list.sort_by_key(|s| s.player_id());
        list
    }

    pub fn player_snapshots_map(&self) -> HashMap<Uuid, PlayerPerformanceSnapshot> {
        self.players
            .iter()
            .map(|(&id, state)| (id, state.to_snapshot()))
            .collect()
    }

    pub fn team_average_rating(&self, team_id: &Uuid) -> Option<f64> {
        let team_players: Vec<&LivePlayerState> = self
            .players
            .values()
            .filter(|p| p.team_id() == *team_id)
            .collect();

        if team_players.is_empty() {
            None
        } else {
            let sum: f64 = team_players
                .iter()
                .map(|p| p.performance_rating().value())
                .sum();
            Some(sum / team_players.len() as f64)
        }
    }

    pub fn all_team_average_ratings(&self) -> HashMap<Uuid, f64> {
        let mut team_sums: HashMap<Uuid, (f64, usize)> = HashMap::new();
        for state in self.players.values() {
            let entry = team_sums.entry(state.team_id()).or_insert((0.0, 0));
            entry.0 += state.performance_rating().value();
            entry.1 += 1;
        }

        team_sums
            .into_iter()
            .map(|(tid, (sum, count))| (tid, sum / count.max(1) as f64))
            .collect()
    }

    pub fn snapshot_history(&self) -> &[LivePerformanceSnapshotRecord] {
        &self.history
    }

    pub fn snapshot_at_sequence(&self, sequence_number: u64) -> Option<&LivePerformanceSnapshotRecord> {
        self.history.iter().find(|s| s.sequence_number() == sequence_number)
    }

    pub fn latest_snapshot(&self) -> Option<&LivePerformanceSnapshotRecord> {
        self.history.last()
    }

    pub fn apply_match_outcome(
        &mut self,
        team_id: Uuid,
        outcome: MatchOutcome,
        policy: &OutcomeAdjustmentPolicy,
    ) {
        let adjustment = policy.adjustment_for(outcome);
        for state in self.players.values_mut() {
            if state.team_id() == team_id {
                state.set_outcome_adjustment(adjustment);
            }
        }
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

    pub fn reset_state(&mut self) {
        self.translator.reset();
        self.players.clear();
        self.history.clear();
        self.last_clock = MatchClockInstant::zero();
        self.last_clock_seconds = 0.0;
        self.sequence_counter = 0;
    }

    fn inspect_event_for_roster_updates(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::SubstitutionMade(e) => {
                self.handle_substitution(e.player_out(), e.player_in(), e.team_id());
            }
            MatchEvent::PlayerAvailabilityChanged(e) => {
                self.handle_availability(e.player_id(), e.new_status());
            }
            _ => {}
        }
    }

    fn ensure_player_exists(&mut self, player_id: Uuid, team_id: Uuid) {
        if !self.players.contains_key(&player_id) {
            let (off_pos, def_pos, role) = if let Some(ctx) = self.translator.context() {
                let off = ctx.offensive_position(&player_id).unwrap_or(Position::CenterOffense);
                let def = ctx.defensive_position(&player_id).unwrap_or(Position::Centerback);
                let r = ctx.slot_role(&player_id).unwrap_or(SlotRole::Standard);
                (off, def, r)
            } else {
                (Position::CenterOffense, Position::Centerback, SlotRole::Standard)
            };
            self.players.insert(
                player_id,
                LivePlayerState::new(
                    player_id,
                    team_id,
                    off_pos,
                    def_pos,
                    role,
                    true,
                    &self.config,
                ),
            );
        }
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