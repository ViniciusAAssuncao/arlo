use crate::playcall::axes::ReadPriority;
use arlo_domain::pitch::ArtroPlacement;
use arlo_domain::sport_constants::{NORMALIZED_RATIO_MAX, NORMALIZED_RATIO_MIN};
use arlo_domain::validation::validate_float_range;
use arlo_domain::DomainResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RouteAssignment {
    slot_index: usize,
    target_channel: ArtroPlacement,
    depth_ratio: f64,
    break_ratio: f64,
    read_priority: ReadPriority,
}

impl RouteAssignment {
    pub fn new(
        slot_index: usize,
        target_channel: ArtroPlacement,
        depth_ratio: f64,
        break_ratio: f64,
        read_priority: ReadPriority,
    ) -> DomainResult<Self> {
        validate_float_range(
            depth_ratio,
            NORMALIZED_RATIO_MIN,
            NORMALIZED_RATIO_MAX,
            "depth_ratio",
        )?;
        validate_float_range(
            break_ratio,
            NORMALIZED_RATIO_MIN,
            NORMALIZED_RATIO_MAX,
            "break_ratio",
        )?;

        Ok(Self {
            slot_index,
            target_channel,
            depth_ratio,
            break_ratio,
            read_priority,
        })
    }

    pub fn slot_index(&self) -> usize {
        self.slot_index
    }

    pub fn target_channel(&self) -> ArtroPlacement {
        self.target_channel
    }

    pub fn depth_ratio(&self) -> f64 {
        self.depth_ratio
    }

    pub fn break_ratio(&self) -> f64 {
        self.break_ratio
    }

    pub fn read_priority(&self) -> ReadPriority {
        self.read_priority
    }
}
