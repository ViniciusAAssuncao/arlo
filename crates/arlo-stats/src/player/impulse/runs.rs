use crate::snapshot::{ImpulseRunSnapshot, TeamImpulseRunSnapshot};
use serde::{Deserialize, Serialize};
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
