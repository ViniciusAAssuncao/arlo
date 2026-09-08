use super::runs::TeamImpulseRun;
use crate::snapshot::{IntoSnapshot, TeamImpulseSnapshot};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
