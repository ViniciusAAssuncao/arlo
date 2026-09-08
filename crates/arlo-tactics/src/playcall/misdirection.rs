use crate::playcall::route::RouteAssignment;
use arlo_math::stats::UnipolarScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MisdirectionLink {
    decoy_slot_index: usize,
    true_carrier_slot_index: usize,
    deception_intensity: UnipolarScalar,
}

impl MisdirectionLink {
    pub fn new(
        decoy_slot_index: usize,
        true_carrier_slot_index: usize,
        deception_intensity: UnipolarScalar,
    ) -> Self {
        Self {
            decoy_slot_index,
            true_carrier_slot_index,
            deception_intensity,
        }
    }

    pub fn new_clamped(
        decoy_slot_index: usize,
        true_carrier_slot_index: usize,
        deception_intensity: f64,
    ) -> Self {
        Self {
            decoy_slot_index,
            true_carrier_slot_index,
            deception_intensity: UnipolarScalar::new_clamped(deception_intensity),
        }
    }

    pub fn decoy_slot_index(&self) -> usize {
        self.decoy_slot_index
    }

    pub fn true_carrier_slot_index(&self) -> usize {
        self.true_carrier_slot_index
    }

    pub fn deception_intensity(&self) -> UnipolarScalar {
        self.deception_intensity
    }

    pub fn geometric_similarity(
        &self,
        decoy_route: &RouteAssignment,
        true_route: &RouteAssignment,
    ) -> f64 {
        let channel_term = if decoy_route.target_channel() == true_route.target_channel() {
            1.0
        } else {
            0.0
        };
        let depth_term = 1.0 - (decoy_route.depth_ratio() - true_route.depth_ratio()).abs();
        let break_term = 1.0 - (decoy_route.break_ratio() - true_route.break_ratio()).abs();
        let mean = (channel_term + depth_term + break_term) / 3.0;
        mean * self.deception_intensity.value()
    }
}