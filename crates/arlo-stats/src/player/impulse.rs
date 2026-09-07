use crate::aggregator::StatAggregator;
use crate::snapshot::{
    ImpulseRunSnapshot, IntoSnapshot, PlayerImpulseSnapshot, TeamImpulseRunSnapshot,
    TeamImpulseSnapshot,
};
use arlo_events::{ImpulseEventKind, MatchEvent, MatchEventEnvelope};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseRun {
    pub start_time_seconds: f64,
    pub end_time_seconds: f64,
    pub duration_seconds: f64,
    pub peak_value: u8,
    pub integrated_intensity: f64,
    pub shifts_count: u32,
}

impl ImpulseRun {
    pub fn new(start_time_seconds: f64, initial_value: u8) -> Self {
        Self {
            start_time_seconds,
            end_time_seconds: start_time_seconds,
            duration_seconds: 0.0,
            peak_value: initial_value,
            integrated_intensity: 0.0,
            shifts_count: 0,
        }
    }

    pub fn start_time_seconds(&self) -> f64 {
        self.start_time_seconds
    }

    pub fn end_time_seconds(&self) -> f64 {
        self.end_time_seconds
    }

    pub fn duration_seconds(&self) -> f64 {
        self.duration_seconds
    }

    pub fn peak_value(&self) -> u8 {
        self.peak_value
    }

    pub fn integrated_intensity(&self) -> f64 {
        self.integrated_intensity
    }

    pub fn shifts_count(&self) -> u32 {
        self.shifts_count
    }

    pub fn average_intensity(&self) -> f64 {
        if self.duration_seconds <= 0.0 {
            0.0
        } else {
            self.integrated_intensity / self.duration_seconds
        }
    }

    pub fn into_snapshot(&self) -> ImpulseRunSnapshot {
        ImpulseRunSnapshot {
            start_time_seconds: self.start_time_seconds,
            end_time_seconds: self.end_time_seconds,
            duration_seconds: self.duration_seconds,
            peak_value: self.peak_value,
            integrated_intensity: self.integrated_intensity,
            shifts_count: self.shifts_count,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TeamImpulseRun {
    pub team_id: Uuid,
    pub start_time_seconds: f64,
    pub end_time_seconds: f64,
    pub duration_seconds: f64,
    pub peak_average_value: f64,
    pub integrated_intensity: f64,
}

impl TeamImpulseRun {
    pub fn new(team_id: Uuid, start_time_seconds: f64, initial_average: f64) -> Self {
        Self {
            team_id,
            start_time_seconds,
            end_time_seconds: start_time_seconds,
            duration_seconds: 0.0,
            peak_average_value: initial_average,
            integrated_intensity: 0.0,
        }
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn start_time_seconds(&self) -> f64 {
        self.start_time_seconds
    }

    pub fn end_time_seconds(&self) -> f64 {
        self.end_time_seconds
    }

    pub fn duration_seconds(&self) -> f64 {
        self.duration_seconds
    }

    pub fn peak_average_value(&self) -> f64 {
        self.peak_average_value
    }

    pub fn integrated_intensity(&self) -> f64 {
        self.integrated_intensity
    }

    pub fn average_intensity(&self) -> f64 {
        if self.duration_seconds <= 0.0 {
            0.0
        } else {
            self.integrated_intensity / self.duration_seconds
        }
    }

    pub fn into_snapshot(&self) -> TeamImpulseRunSnapshot {
        TeamImpulseRunSnapshot {
            team_id: self.team_id,
            start_time_seconds: self.start_time_seconds,
            end_time_seconds: self.end_time_seconds,
            duration_seconds: self.duration_seconds,
            peak_average_value: self.peak_average_value,
            integrated_intensity: self.integrated_intensity,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerImpulseStats {
    pub player_id: Uuid,
    pub team_id: Option<Uuid>,
    pub baseline: f64,
    pub current_value: u8,
    pub initial_value: u8,
    pub min_value: u8,
    pub max_value: u8,
    pub shifts_count: u32,
    pub positive_shifts: u32,
    pub negative_shifts: u32,
    pub shifts_by_kind: HashMap<ImpulseEventKind, u32>,
    pub time_below_baseline_seconds: f64,
    pub critical_reached_count: u32,
    pub total_tracked_time_seconds: f64,
    pub time_weighted_sum: f64,
    pub discrete_sample_sum: f64,
    pub discrete_sample_count: u32,
    pub runs: Vec<ImpulseRun>,
    pub active_run: Option<ImpulseRun>,
}

impl PlayerImpulseStats {
    pub fn new(player_id: Uuid, baseline: f64) -> Self {
        let initial_val = baseline.clamp(0.0, 100.0).round() as u8;
        let mut stats = Self {
            player_id,
            team_id: None,
            baseline,
            current_value: initial_val,
            initial_value: initial_val,
            min_value: initial_val,
            max_value: initial_val,
            shifts_count: 0,
            positive_shifts: 0,
            negative_shifts: 0,
            shifts_by_kind: HashMap::new(),
            time_below_baseline_seconds: 0.0,
            critical_reached_count: 0,
            total_tracked_time_seconds: 0.0,
            time_weighted_sum: 0.0,
            discrete_sample_sum: initial_val as f64,
            discrete_sample_count: 1,
            runs: Vec::new(),
            active_run: None,
        };

        if (initial_val as f64) >= baseline {
            stats.active_run = Some(ImpulseRun::new(0.0, initial_val));
        }

        stats
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn team_id(&self) -> Option<Uuid> {
        self.team_id
    }

    pub fn baseline(&self) -> f64 {
        self.baseline
    }

    pub fn current_value(&self) -> u8 {
        self.current_value
    }

    pub fn min_value(&self) -> u8 {
        self.min_value
    }

    pub fn max_value(&self) -> u8 {
        self.max_value
    }

    pub fn average_value(&self) -> f64 {
        if self.total_tracked_time_seconds > 0.0 {
            self.time_weighted_sum / self.total_tracked_time_seconds
        } else if self.discrete_sample_count > 0 {
            self.discrete_sample_sum / (self.discrete_sample_count as f64)
        } else {
            self.current_value as f64
        }
    }

    pub fn shifts_count(&self) -> u32 {
        self.shifts_count
    }

    pub fn positive_shifts(&self) -> u32 {
        self.positive_shifts
    }

    pub fn negative_shifts(&self) -> u32 {
        self.negative_shifts
    }

    pub fn shifts_by_kind(&self) -> &HashMap<ImpulseEventKind, u32> {
        &self.shifts_by_kind
    }

    pub fn count_for_kind(&self, kind: ImpulseEventKind) -> u32 {
        self.shifts_by_kind.get(&kind).copied().unwrap_or(0)
    }

    pub fn time_below_baseline_seconds(&self) -> f64 {
        self.time_below_baseline_seconds
    }

    pub fn critical_reached_count(&self) -> u32 {
        self.critical_reached_count
    }

    pub fn all_completed_and_active_runs(&self) -> Vec<ImpulseRun> {
        let mut all = self.runs.clone();
        if let Some(active) = self.active_run {
            all.push(active);
        }
        all
    }

    pub fn runs(&self) -> &[ImpulseRun] {
        &self.runs
    }

    pub fn runs_count(&self) -> u32 {
        let active_count = if self.active_run.is_some() { 1 } else { 0 };
        (self.runs.len() as u32) + active_count
    }

    pub fn longest_run_duration_seconds(&self) -> f64 {
        let completed_max = self
            .runs
            .iter()
            .map(|r| r.duration_seconds)
            .fold(0.0_f64, f64::max);
        if let Some(active) = self.active_run {
            completed_max.max(active.duration_seconds)
        } else {
            completed_max
        }
    }

    pub fn peak_run_value(&self) -> u8 {
        let completed_max = self
            .runs
            .iter()
            .map(|r| r.peak_value)
            .max()
            .unwrap_or(self.min_value);
        if let Some(active) = self.active_run {
            completed_max.max(active.peak_value)
        } else {
            completed_max
        }
    }

    pub fn total_integrated_run_intensity(&self) -> f64 {
        let sum: f64 = self.runs.iter().map(|r| r.integrated_intensity).sum();
        if let Some(active) = self.active_run {
            sum + active.integrated_intensity
        } else {
            sum
        }
    }

    pub fn average_run_duration_seconds(&self) -> f64 {
        let total_runs = self.runs_count();
        if total_runs == 0 {
            0.0
        } else {
            let total_dur: f64 = self.runs.iter().map(|r| r.duration_seconds).sum::<f64>()
                + self
                    .active_run
                    .map(|r| r.duration_seconds)
                    .unwrap_or(0.0);
            total_dur / (total_runs as f64)
        }
    }

    pub fn average_run_intensity(&self) -> f64 {
        let total_runs = self.runs_count();
        if total_runs == 0 {
            0.0
        } else {
            self.total_integrated_run_intensity() / (total_runs as f64)
        }
    }

    pub fn advance_time(&mut self, dt: f64, current_time: f64) {
        if dt <= 0.0 {
            return;
        }

        let val_f = self.current_value as f64;
        self.total_tracked_time_seconds += dt;
        self.time_weighted_sum += val_f * dt;

        if val_f < self.baseline {
            self.time_below_baseline_seconds += dt;
            if let Some(mut run) = self.active_run.take() {
                run.end_time_seconds = current_time;
                self.runs.push(run);
            }
        } else {
            let excess = (val_f - self.baseline).max(0.0);
            if let Some(run) = &mut self.active_run {
                run.duration_seconds += dt;
                run.end_time_seconds = current_time;
                run.integrated_intensity += excess * dt;
                run.peak_value = run.peak_value.max(self.current_value);
            } else {
                let mut run = ImpulseRun::new(current_time - dt, self.current_value);
                run.duration_seconds = dt;
                run.end_time_seconds = current_time;
                run.integrated_intensity = excess * dt;
                self.active_run = Some(run);
            }
        }
    }

    pub fn apply_value_shift(
        &mut self,
        new_value: u8,
        event_kind: ImpulseEventKind,
        timestamp_seconds: f64,
    ) {
        let prev_value = self.current_value;
        self.current_value = new_value;
        self.min_value = self.min_value.min(new_value);
        self.max_value = self.max_value.max(new_value);
        self.shifts_count += 1;
        *self.shifts_by_kind.entry(event_kind).or_insert(0) += 1;

        if new_value > prev_value {
            self.positive_shifts += 1;
        } else if new_value < prev_value {
            self.negative_shifts += 1;
        }

        self.discrete_sample_sum += new_value as f64;
        self.discrete_sample_count += 1;

        let val_f = new_value as f64;
        if val_f >= self.baseline {
            if let Some(run) = &mut self.active_run {
                run.peak_value = run.peak_value.max(new_value);
                run.shifts_count += 1;
            } else {
                let mut run = ImpulseRun::new(timestamp_seconds, new_value);
                run.shifts_count = 1;
                self.active_run = Some(run);
            }
        } else if let Some(mut run) = self.active_run.take() {
            run.end_time_seconds = timestamp_seconds;
            self.runs.push(run);
        }
    }

    pub fn finalize_active_run(&mut self, final_timestamp: f64) {
        if let Some(mut run) = self.active_run.take() {
            run.end_time_seconds = final_timestamp;
            self.runs.push(run);
        }
    }
}

impl IntoSnapshot for PlayerImpulseStats {
    type Snapshot = PlayerImpulseSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        let run_snapshots = self
            .all_completed_and_active_runs()
            .into_iter()
            .map(|r| r.into_snapshot())
            .collect();

        PlayerImpulseSnapshot {
            player_id: self.player_id,
            baseline: self.baseline,
            current_value: self.current_value,
            min_value: self.min_value,
            max_value: self.max_value,
            average_value: self.average_value(),
            total_shifts: self.shifts_count,
            positive_shifts: self.positive_shifts,
            negative_shifts: self.negative_shifts,
            time_below_baseline_seconds: self.time_below_baseline_seconds,
            critical_reached_count: self.critical_reached_count,
            runs_count: self.runs_count(),
            longest_run_duration_seconds: self.longest_run_duration_seconds(),
            peak_run_value: self.peak_run_value(),
            total_integrated_run_intensity: self.total_integrated_run_intensity(),
            runs: run_snapshots,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TeamImpulseStats {
    pub team_id: Uuid,
    pub player_ids: Vec<Uuid>,
    pub average_baseline: f64,
    pub current_average_value: f64,
    pub min_average_value: f64,
    pub max_average_value: f64,
    pub time_below_baseline_seconds: f64,
    pub total_tracked_time_seconds: f64,
    pub time_weighted_sum: f64,
    pub runs: Vec<TeamImpulseRun>,
    pub active_run: Option<TeamImpulseRun>,
}

impl TeamImpulseStats {
    pub fn new(team_id: Uuid, player_ids: Vec<Uuid>, average_baseline: f64) -> Self {
        let init_val = average_baseline;
        let mut stats = Self {
            team_id,
            player_ids,
            average_baseline,
            current_average_value: init_val,
            min_average_value: init_val,
            max_average_value: init_val,
            time_below_baseline_seconds: 0.0,
            total_tracked_time_seconds: 0.0,
            time_weighted_sum: 0.0,
            runs: Vec::new(),
            active_run: None,
        };

        if init_val >= average_baseline {
            stats.active_run = Some(TeamImpulseRun::new(team_id, 0.0, init_val));
        }

        stats
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn player_ids(&self) -> &[Uuid] {
        &self.player_ids
    }

    pub fn average_baseline(&self) -> f64 {
        self.average_baseline
    }

    pub fn current_average_value(&self) -> f64 {
        self.current_average_value
    }

    pub fn min_average_value(&self) -> f64 {
        self.min_average_value
    }

    pub fn max_average_value(&self) -> f64 {
        self.max_average_value
    }

    pub fn average_value(&self) -> f64 {
        if self.total_tracked_time_seconds > 0.0 {
            self.time_weighted_sum / self.total_tracked_time_seconds
        } else {
            self.current_average_value
        }
    }

    pub fn time_below_baseline_seconds(&self) -> f64 {
        self.time_below_baseline_seconds
    }

    pub fn all_completed_and_active_runs(&self) -> Vec<TeamImpulseRun> {
        let mut all = self.runs.clone();
        if let Some(active) = self.active_run {
            all.push(active);
        }
        all
    }

    pub fn runs(&self) -> &[TeamImpulseRun] {
        &self.runs
    }

    pub fn runs_count(&self) -> u32 {
        let active_count = if self.active_run.is_some() { 1 } else { 0 };
        (self.runs.len() as u32) + active_count
    }

    pub fn longest_run_duration_seconds(&self) -> f64 {
        let completed_max = self
            .runs
            .iter()
            .map(|r| r.duration_seconds)
            .fold(0.0_f64, f64::max);
        if let Some(active) = self.active_run {
            completed_max.max(active.duration_seconds)
        } else {
            completed_max
        }
    }

    pub fn peak_run_average_value(&self) -> f64 {
        let completed_max = self
            .runs
            .iter()
            .map(|r| r.peak_average_value)
            .fold(self.min_average_value, f64::max);
        if let Some(active) = self.active_run {
            completed_max.max(active.peak_average_value)
        } else {
            completed_max
        }
    }

    pub fn total_integrated_run_intensity(&self) -> f64 {
        let sum: f64 = self.runs.iter().map(|r| r.integrated_intensity).sum();
        if let Some(active) = self.active_run {
            sum + active.integrated_intensity
        } else {
            sum
        }
    }

    pub fn average_run_duration_seconds(&self) -> f64 {
        let total = self.runs_count();
        if total == 0 {
            0.0
        } else {
            let sum_dur: f64 = self.runs.iter().map(|r| r.duration_seconds).sum::<f64>()
                + self.active_run.map(|r| r.duration_seconds).unwrap_or(0.0);
            sum_dur / (total as f64)
        }
    }

    pub fn update_team_average(&mut self, new_average: f64, timestamp_seconds: f64) {
        self.current_average_value = new_average;
        self.min_average_value = self.min_average_value.min(new_average);
        self.max_average_value = self.max_average_value.max(new_average);

        if new_average >= self.average_baseline {
            if let Some(run) = &mut self.active_run {
                run.peak_average_value = run.peak_average_value.max(new_average);
            } else {
                self.active_run = Some(TeamImpulseRun::new(
                    self.team_id,
                    timestamp_seconds,
                    new_average,
                ));
            }
        } else if let Some(mut run) = self.active_run.take() {
            run.end_time_seconds = timestamp_seconds;
            self.runs.push(run);
        }
    }

    pub fn advance_time(&mut self, dt: f64, current_time: f64) {
        if dt <= 0.0 {
            return;
        }

        let avg = self.current_average_value;
        self.total_tracked_time_seconds += dt;
        self.time_weighted_sum += avg * dt;

        if avg < self.average_baseline {
            self.time_below_baseline_seconds += dt;
            if let Some(mut run) = self.active_run.take() {
                run.end_time_seconds = current_time;
                self.runs.push(run);
            }
        } else {
            let excess = (avg - self.average_baseline).max(0.0);
            if let Some(run) = &mut self.active_run {
                run.duration_seconds += dt;
                run.end_time_seconds = current_time;
                run.integrated_intensity += excess * dt;
                run.peak_average_value = run.peak_average_value.max(avg);
            } else {
                let mut run = TeamImpulseRun::new(self.team_id, current_time - dt, avg);
                run.duration_seconds = dt;
                run.end_time_seconds = current_time;
                run.integrated_intensity = excess * dt;
                self.active_run = Some(run);
            }
        }
    }

    pub fn finalize_active_run(&mut self, final_timestamp: f64) {
        if let Some(mut run) = self.active_run.take() {
            run.end_time_seconds = final_timestamp;
            self.runs.push(run);
        }
    }
}

impl IntoSnapshot for TeamImpulseStats {
    type Snapshot = TeamImpulseSnapshot;

    fn into_snapshot(&self) -> Self::Snapshot {
        let run_snapshots = self
            .all_completed_and_active_runs()
            .into_iter()
            .map(|r| r.into_snapshot())
            .collect();

        TeamImpulseSnapshot {
            team_id: self.team_id,
            average_baseline: self.average_baseline,
            current_average_value: self.current_average_value,
            min_average_value: self.min_average_value,
            max_average_value: self.max_average_value,
            average_value: self.average_value(),
            time_below_baseline_seconds: self.time_below_baseline_seconds,
            runs_count: self.runs_count(),
            longest_run_duration_seconds: self.longest_run_duration_seconds(),
            peak_run_average_value: self.peak_run_average_value(),
            total_integrated_run_intensity: self.total_integrated_run_intensity(),
            runs: run_snapshots,
        }
    }
}

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
            self.player_stats
                .insert(player_id, PlayerImpulseStats::new(player_id, baseline));
        }
    }

    pub fn register_player_team(&mut self, player_id: Uuid, team_id: Uuid) {
        self.player_team_map.insert(player_id, team_id);
        if let Some(pstats) = self.player_stats.get_mut(&player_id) {
            pstats.team_id = Some(team_id);
        }
        let team_entry = self
            .team_stats
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
            TeamImpulseStats::new(team_id, player_ids, average_baseline),
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
        self.player_stats
            .entry(player_id)
            .or_insert_with(|| {
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
        timestamp_seconds: f64,
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
        timestamp_seconds: f64,
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
        self.update_time(timestamp_seconds);

        match envelope.event() {
            MatchEvent::ImpulseShiftRecorded(e) => {
                self.record_shift(
                    e.player_id(),
                    e.previous_value(),
                    e.new_value(),
                    e.event_kind(),
                    e.surprisal(),
                    timestamp_seconds,
                );
            }
            MatchEvent::ImpulseCriticalReached(e) => {
                self.record_critical(
                    e.player_id(),
                    e.value(),
                    e.duration_seconds(),
                    timestamp_seconds,
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

    fn handle_event(&mut self, event: &MatchEvent) {
        let current_time = self.last_timestamp_seconds;
        match event {
            MatchEvent::ImpulseShiftRecorded(e) => {
                self.record_shift(
                    e.player_id(),
                    e.previous_value(),
                    e.new_value(),
                    e.event_kind(),
                    e.surprisal(),
                    current_time,
                );
            }
            MatchEvent::ImpulseCriticalReached(e) => {
                self.record_critical(
                    e.player_id(),
                    e.value(),
                    e.duration_seconds(),
                    current_time,
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

    fn reset(&mut self) {
        self.player_stats.clear();
        self.team_stats.clear();
        self.player_team_map.clear();
        self.last_timestamp_seconds = 0.0;
    }
}