use super::runs::ImpulseRun;
use crate::snapshot::{ IntoSnapshot, PlayerImpulseSnapshot };
use arlo_events::ImpulseEventKind;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use uuid::Uuid;

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
        let completed_max = self.runs
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
        let completed_max = self.runs
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
        let sum: f64 = self.runs
            .iter()
            .map(|r| r.integrated_intensity)
            .sum();
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
            let total_dur: f64 =
                self.runs
                    .iter()
                    .map(|r| r.duration_seconds)
                    .sum::<f64>() + self.active_run.map(|r| r.duration_seconds).unwrap_or(0.0);
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
        timestamp_seconds: f64
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
