use crate::context::{MatchAnalysisContext, PlayerAssignment};
use crate::error::AnalyticsResult;
use crate::performance::live::config::LiveRatingConfig;
use crate::performance::live::finalizer::{
    apply_outcome_to_players, clear_players_outcome, extract_player_match_ratings,
    MatchFinalizationState,
};
use crate::performance::live::player_state::LivePlayerState;
use crate::performance::live::replay;
use crate::performance::live::seed::InitialParticipantSeed;
use crate::performance::live::snapshot::LivePerformanceSnapshotRecord;
use crate::performance::observation::PerformanceObservation;
use crate::performance::rating::{
    MatchOutcome, OutcomeAdjustmentPolicy, PlayerPerformanceSnapshot,
};
use crate::performance::translator::EventPerformanceTranslator;
use crate::performance::PlayerMatchRating;
use arlo_domain::tactics::Formation;
use arlo_domain::{Position, SlotRole};
use arlo_events::{AvailabilityStatus, MatchClockInstant, MatchEvent, MatchEventEnvelope};
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
        let mut agg = Self {
            initial_context: Some(context.clone()),
            translator: EventPerformanceTranslator::with_context(context.clone()),
            ..Default::default()
        };
        agg.seed_from_context(&context);
        agg
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

    pub fn seed_from_context(&mut self, context: &MatchAnalysisContext) {
        let mut assignments: Vec<&PlayerAssignment> = context.all_assignments().values().collect();
        assignments.sort_by_key(|a| (a.slot_index(), a.player_id()));
        for assignment in assignments {
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
        let seed = InitialParticipantSeed::new(
            player_id,
            team_id,
            offensive_position,
            defensive_position,
            slot_role,
            is_active,
        );

        if let Some(pos) = self
            .initial_seeds
            .iter()
            .position(|s| s.player_id() == player_id)
        {
            self.initial_seeds[pos] = seed;
        } else {
            self.initial_seeds.push(seed);
        }

        self.apply_seed(&seed);
    }

    fn apply_seed(&mut self, seed: &InitialParticipantSeed) {
        let mut state = LivePlayerState::new(
            seed.player_id(),
            seed.team_id(),
            seed.offensive_position(),
            seed.defensive_position(),
            seed.slot_role(),
            seed.is_active(),
            &self.config,
        );
        state.set_starter(seed.is_active());
        if seed.is_active() {
            state.set_entry_time_seconds(0.0);
        }
        self.players.insert(seed.player_id(), state);
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

        let clock_seconds = self.last_clock_seconds;

        let (off_pos, def_pos, role) = if let Some(ctx) = self.translator.context() {
            let off = ctx
                .offensive_position(&player_in)
                .or_else(|| ctx.offensive_position(&player_out))
                .unwrap_or(Position::CenterOffense);
            let def = ctx
                .defensive_position(&player_in)
                .or_else(|| ctx.defensive_position(&player_out))
                .unwrap_or(Position::Centerback);
            let r = ctx
                .slot_role(&player_in)
                .or_else(|| ctx.slot_role(&player_out))
                .unwrap_or(SlotRole::Standard);
            (off, def, r)
        } else if let Some(out_state) = self.players.get(&player_out) {
            (
                out_state.offensive_position(),
                out_state.defensive_position(),
                out_state.slot_role(),
            )
        } else {
            (Position::CenterOffense, Position::Centerback, SlotRole::Standard)
        };

        if let Some(in_state) = self.players.get_mut(&player_in) {
            in_state.set_active(true);
            in_state.set_entry_time_seconds(clock_seconds);
            in_state.update_assignment(off_pos, def_pos, role, &self.config);
        } else {
            let mut new_state = LivePlayerState::new(
                player_in,
                team_id,
                off_pos,
                def_pos,
                role,
                true,
                &self.config,
            );
            new_state.set_starter(false);
            new_state.set_entry_time_seconds(clock_seconds);
            self.players.insert(player_in, new_state);
        }
    }

    pub fn handle_availability(
        &mut self,
        player_id: Uuid,
        previous_status: AvailabilityStatus,
        new_status: AvailabilityStatus,
    ) {
        match new_status {
            AvailabilityStatus::Expelled
            | AvailabilityStatus::Injured
            | AvailabilityStatus::Suspended => {
                if let Some(state) = self.players.get_mut(&player_id) {
                    state.set_active(false);
                }
            }
            AvailabilityStatus::Active if previous_status == AvailabilityStatus::Suspended => {
                if let Some(state) = self.players.get_mut(&player_id) {
                    state.set_active(true);
                }
            }
            _ => {}
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
        let mut team_players: Vec<&LivePlayerState> = self
            .players
            .values()
            .filter(|p| p.team_id() == *team_id)
            .collect();

        if team_players.is_empty() {
            None
        } else {
            team_players.sort_by_key(|p| p.player_id());
            let sum: f64 = team_players
                .iter()
                .map(|p| p.performance_rating().value())
                .sum();
            Some(sum / team_players.len() as f64)
        }
    }

    pub fn all_team_average_ratings(&self) -> HashMap<Uuid, f64> {
        let mut sorted_players: Vec<&LivePlayerState> = self.players.values().collect();
        sorted_players.sort_by_key(|p| p.player_id());

        let mut team_sums: HashMap<Uuid, (f64, usize)> = HashMap::new();
        for state in sorted_players {
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
        apply_outcome_to_players(&mut self.players, team_id, outcome, policy);
        self.finalization.record_outcome(team_id, outcome);
    }

    pub fn apply_outcome(&mut self, team_id: Uuid, outcome: MatchOutcome) {
        let policy = *self.config.outcome_policy();
        self.apply_match_outcome(team_id, outcome, &policy);
    }

    pub fn finalize_match(
        &mut self,
        home_team_id: Uuid,
        home_outcome: MatchOutcome,
        away_team_id: Uuid,
        away_outcome: MatchOutcome,
        policy: &OutcomeAdjustmentPolicy,
    ) -> LivePerformanceSnapshotRecord {
        self.apply_match_outcome(home_team_id, home_outcome, policy);
        self.apply_match_outcome(away_team_id, away_outcome, policy);
        self.finalization.mark_finalized();

        let snap = self.create_snapshot(self.sequence_counter, self.last_clock);
        if let Some(last) = self.history.last_mut() {
            if last.sequence_number() == self.sequence_counter && last.clock() == self.last_clock {
                *last = snap.clone();
                return snap;
            }
        }

        self.history.push(snap.clone());
        snap
    }

    pub fn finalize_match_with_scores(
        &mut self,
        home_team_id: Uuid,
        home_score: u32,
        away_team_id: Uuid,
        away_score: u32,
        policy: &OutcomeAdjustmentPolicy,
    ) -> LivePerformanceSnapshotRecord {
        let home_outcome = MatchOutcome::from_scores(home_score, away_score);
        let away_outcome = MatchOutcome::from_scores(away_score, home_score);
        self.finalize_match(home_team_id, home_outcome, away_team_id, away_outcome, policy)
    }

    pub fn finalize_with_config(
        &mut self,
        home_team_id: Uuid,
        home_outcome: MatchOutcome,
        away_team_id: Uuid,
        away_outcome: MatchOutcome,
    ) -> LivePerformanceSnapshotRecord {
        let policy = *self.config.outcome_policy();
        self.finalize_match(home_team_id, home_outcome, away_team_id, away_outcome, &policy)
    }

    pub fn finalize_with_scores_and_config(
        &mut self,
        home_team_id: Uuid,
        home_score: u32,
        away_team_id: Uuid,
        away_score: u32,
    ) -> LivePerformanceSnapshotRecord {
        let policy = *self.config.outcome_policy();
        self.finalize_match_with_scores(home_team_id, home_score, away_team_id, away_score, &policy)
    }

    pub fn is_finalized(&self) -> bool {
        self.finalization.is_finalized()
    }

    pub fn finalized_outcome_for_team(&self, team_id: &Uuid) -> Option<MatchOutcome> {
        self.finalization.outcome_for_team(team_id)
    }

    pub fn final_player_ratings(&self) -> Vec<PlayerMatchRating> {
        extract_player_match_ratings(&self.players)
    }

    pub fn final_player_rating(&self, player_id: &Uuid) -> Option<f64> {
        self.players.get(player_id).map(|p| p.final_rating().value())
    }

    pub fn clear_match_outcomes(&mut self) {
        clear_players_outcome(&mut self.players);
        self.finalization.clear();
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
                .all(|p| p.effective_opportunities() == 0 && p.seconds_played() == 0.0)
    }

    pub fn replay<'a>(
        &mut self,
        envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
    ) {
        self.reset_state();
        self.handle_envelopes(envelopes);
    }

    pub fn replay_from_sink(&mut self, sink: &arlo_events::InMemorySink) {
        self.replay(sink.events());
    }

    pub fn invalidate_and_replay<'a>(
        &mut self,
        first_sequence: u64,
        last_sequence: u64,
        envelopes: impl IntoIterator<Item = &'a MatchEventEnvelope>,
    ) {
        let surviving = replay::filter_surviving_envelopes(first_sequence, last_sequence, envelopes);
        self.replay(surviving);
    }

    fn inspect_event_for_roster_updates(&mut self, event: &MatchEvent) {
        match event {
            MatchEvent::SubstitutionMade(e) => {
                self.handle_substitution(e.player_out(), e.player_in(), e.team_id());
            }
            MatchEvent::PlayerAvailabilityChanged(e) => {
                self.handle_availability(e.player_id(), e.previous_status(), e.new_status());
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
            let mut state = LivePlayerState::new(
                player_id,
                team_id,
                off_pos,
                def_pos,
                role,
                true,
                &self.config,
            );
            state.set_starter(false);
            state.set_entry_time_seconds(self.last_clock_seconds);
            self.players.insert(player_id, state);
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