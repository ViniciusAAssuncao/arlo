use super::player_stats::PlayerImpulseStats;
use super::team_stats::TeamImpulseStats;
use crate::aggregator::StatAggregator;
use crate::snapshot::{ IntoSnapshot, PlayerImpulseSnapshot };
use arlo_events::{ ImpulseEventKind, MatchEvent, MatchEventEnvelope };
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PlayerImpulseAggregator {
    player_stats: HashMap<Uuid, PlayerImpulseStats>,
    team_stats: HashMap<Uuid, TeamImpulseStats>,
    player_team_map: HashMap<Uuid, Uuid>,
    last_timestamp_seconds: f64,
}

pub type ImpulseAggregator = PlayerImpulseAggregator;

impl PlayerImpulseAggregator {
    pub fn new() -> Self {
        Self {
            player_stats: HashMap::new(),
            team_stats: HashMap::new(),
            player_team_map: HashMap::new(),
            last_timestamp_seconds: 0.0,
        }
    }

    pub fn register_player_baseline(&mut self, player_id: Uuid, baseline: f64) {
        if let Some(stats) = self.player_stats.get_mut(&player_id) {
            stats.baseline = baseline;
        } else {
            self.player_stats.insert(player_id, PlayerImpulseStats::new(player_id, baseline));
        }
    }

    pub fn register_player_team(&mut self, player_id: Uuid, team_id: Uuid) {
        self.player_team_map.insert(player_id, team_id);
        if let Some(pstats) = self.player_stats.get_mut(&player_id) {
            pstats.team_id = Some(team_id);
        }
        let team_entry = self.team_stats
            .entry(team_id)
            .or_insert_with(|| TeamImpulseStats::new(team_id, Vec::new(), 50.0));
        if !team_entry.player_ids.contains(&player_id) {
            team_entry.player_ids.push(player_id);
        }
        self.recompute_team_baseline(team_id);
    }

    pub fn register_team(&mut self, team_id: Uuid, player_ids: Vec<Uuid>, average_baseline: f64) {
        for &pid in &player_ids {
            self.player_team_map.insert(pid, team_id);
            if let Some(pstats) = self.player_stats.get_mut(&pid) {
                pstats.team_id = Some(team_id);
            }
        }
        self.team_stats.insert(
            team_id,
            TeamImpulseStats::new(team_id, player_ids, average_baseline)
        );
    }

    fn recompute_team_baseline(&mut self, team_id: Uuid) {
        if let Some(team_entry) = self.team_stats.get_mut(&team_id) {
            if team_entry.player_ids.is_empty() {
                return;
            }
            let mut total_base = 0.0;
            let mut count = 0;
            for pid in &team_entry.player_ids {
                if let Some(pstats) = self.player_stats.get(pid) {
                    total_base += pstats.baseline;
                    count += 1;
                }
            }
            if count > 0 {
                team_entry.average_baseline = total_base / (count as f64);
            }
        }
    }

    fn get_mut_or_create(&mut self, player_id: Uuid) -> &mut PlayerImpulseStats {
        let team_id = self.player_team_map.get(&player_id).copied();
        self.player_stats.entry(player_id).or_insert_with(|| {
            let mut s = PlayerImpulseStats::new(player_id, 50.0);
            s.team_id = team_id;
            s
        })
    }

    pub fn get(&self, player_id: &Uuid) -> Option<&PlayerImpulseStats> {
        self.player_stats.get(player_id)
    }

    pub fn get_or_default(&self, player_id: &Uuid) -> PlayerImpulseStats {
        self.player_stats
            .get(player_id)
            .cloned()
            .unwrap_or_else(|| PlayerImpulseStats::new(*player_id, 50.0))
    }

    pub fn get_team(&self, team_id: &Uuid) -> Option<&TeamImpulseStats> {
        self.team_stats.get(team_id)
    }

    pub fn all_player_stats(&self) -> &HashMap<Uuid, PlayerImpulseStats> {
        &self.player_stats
    }

    pub fn all_team_stats(&self) -> &HashMap<Uuid, TeamImpulseStats> {
        &self.team_stats
    }

    pub fn update_time(&mut self, current_time: f64) {
        if current_time > self.last_timestamp_seconds {
            let dt = current_time - self.last_timestamp_seconds;
            for stats in self.player_stats.values_mut() {
                stats.advance_time(dt, current_time);
            }
            for team_entry in self.team_stats.values_mut() {
                team_entry.advance_time(dt, current_time);
            }
            self.last_timestamp_seconds = current_time;
        }
    }

    pub fn record_shift(
        &mut self,
        player_id: Uuid,
        _previous_value: u8,
        new_value: u8,
        event_kind: ImpulseEventKind,
        _surprisal: f64,
        timestamp_seconds: f64
    ) {
        self.update_time(timestamp_seconds);

        let team_id_opt = {
            let stats = self.get_mut_or_create(player_id);
            stats.apply_value_shift(new_value, event_kind, timestamp_seconds);
            stats.team_id
        };

        if let Some(team_id) = team_id_opt {
            let team_avg = self.calculate_team_current_average(team_id);
            if let Some(tstats) = self.team_stats.get_mut(&team_id) {
                tstats.update_team_average(team_avg, timestamp_seconds);
            }
        }
    }

    pub fn record_critical(
        &mut self,
        player_id: Uuid,
        _value: u8,
        _duration_seconds: f64,
        timestamp_seconds: f64
    ) {
        self.update_time(timestamp_seconds);
        let stats = self.get_mut_or_create(player_id);
        stats.critical_reached_count += 1;
    }

    fn calculate_team_current_average(&self, team_id: Uuid) -> f64 {
        if let Some(tstats) = self.team_stats.get(&team_id) {
            if tstats.player_ids.is_empty() {
                return 50.0;
            }
            let mut sum = 0.0;
            let mut count = 0;
            for pid in &tstats.player_ids {
                if let Some(pstats) = self.player_stats.get(pid) {
                    sum += pstats.current_value as f64;
                    count += 1;
                }
            }
            if count > 0 {
                sum / (count as f64)
            } else {
                50.0
            }
        } else {
            50.0
        }
    }

    pub fn finalize(&mut self, final_timestamp: f64) {
        self.update_time(final_timestamp);
        for stats in self.player_stats.values_mut() {
            stats.finalize_active_run(final_timestamp);
        }
        for team_entry in self.team_stats.values_mut() {
            team_entry.finalize_active_run(final_timestamp);
        }
    }

    fn process_event(&mut self, event: &MatchEvent, timestamp_seconds: f64) {
        self.update_time(timestamp_seconds);
        match event {
            MatchEvent::ImpulseShiftRecorded(e) => {
                self.record_shift(
                    e.player_id(),
                    e.previous_value(),
                    e.new_value(),
                    e.event_kind(),
                    e.surprisal(),
                    timestamp_seconds
                );
            }
            MatchEvent::ImpulseCriticalReached(e) => {
                self.record_critical(
                    e.player_id(),
                    e.value(),
                    e.duration_seconds(),
                    timestamp_seconds
                );
            }
            MatchEvent::CallToActionStarted(e) => {
                self.register_player_team(e.passer_id(), e.offense_team_id());
                self.register_player_team(e.artrine_id(), e.offense_team_id());
            }
            MatchEvent::GoalPoint(e) => {
                self.register_player_team(e.scorer_id(), e.team_id());
                self.register_player_team(e.artrine_id(), e.team_id());
            }
            MatchEvent::FieldPoint(e) => {
                self.register_player_team(e.scorer_id(), e.team_id());
            }
            MatchEvent::FieldGoal(e) => {
                self.register_player_team(e.scorer_id(), e.team_id());
            }
            MatchEvent::ScoringAttemptMissed(e) => {
                self.register_player_team(e.scorer_id(), e.team_id());
            }
            _ => {}
        }
    }
}

impl IntoSnapshot for PlayerImpulseAggregator {
    type Snapshot = HashMap<Uuid, PlayerImpulseSnapshot>;

    fn into_snapshot(&self) -> Self::Snapshot {
        self.player_stats
            .iter()
            .map(|(&id, stats)| (id, stats.into_snapshot()))
            .collect()
    }
}

impl StatAggregator for PlayerImpulseAggregator {
    fn handle_envelope(&mut self, envelope: &MatchEventEnvelope) {
        let timestamp_seconds = envelope.clock().total_elapsed_seconds();
        self.process_event(envelope.event(), timestamp_seconds);
    }

    fn handle_event(&mut self, event: &MatchEvent) {
        let current_time = self.last_timestamp_seconds;
        self.process_event(event, current_time);
    }

    fn reset(&mut self) {
        self.player_stats.clear();
        self.team_stats.clear();
        self.player_team_map.clear();
        self.last_timestamp_seconds = 0.0;
    }
}
