use arlo_domain::sport_constants::{FIELD_POINT_REQUIRED_DRIVES, GOAL_POINT_REQUIRED_DRIVES};
use arlo_domain::{ArtroPlacement, PitchZone};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PitchState {
    down: u8,
    remaining_advance_mirim: f64,
    zone: PitchZone,
    channel: ArtroPlacement,
    normalized_proximity: f64,
    drives_in_series: u32,
    is_bonus_phase: bool,
}

impl PitchState {
    pub fn new(
        down: u8,
        remaining_advance_mirim: f64,
        zone: PitchZone,
        channel: ArtroPlacement,
        normalized_proximity: f64,
        drives_in_series: u32,
        is_bonus_phase: bool,
    ) -> Self {
        Self {
            down: down.clamp(1, 4),
            remaining_advance_mirim: remaining_advance_mirim.max(0.0),
            zone,
            channel,
            normalized_proximity: normalized_proximity.clamp(0.0, 1.0),
            drives_in_series,
            is_bonus_phase,
        }
    }

    pub fn initial() -> Self {
        Self {
            down: 1,
            remaining_advance_mirim: 10.0,
            zone: PitchZone::OpenField,
            channel: ArtroPlacement::Central,
            normalized_proximity: 0.5,
            drives_in_series: 0,
            is_bonus_phase: false,
        }
    }

    pub fn down(&self) -> u8 {
        self.down
    }

    pub fn remaining_advance_mirim(&self) -> f64 {
        self.remaining_advance_mirim
    }

    pub fn zone(&self) -> PitchZone {
        self.zone
    }

    pub fn channel(&self) -> ArtroPlacement {
        self.channel
    }

    pub fn normalized_proximity(&self) -> f64 {
        self.normalized_proximity
    }

    pub fn drives_in_series(&self) -> u32 {
        self.drives_in_series
    }

    pub fn is_bonus_phase(&self) -> bool {
        self.is_bonus_phase
    }

    pub fn is_last_down(&self) -> bool {
        self.down >= 4
    }

    pub fn should_turnover_on_downs(&self) -> bool {
        self.is_last_down() && self.remaining_advance_mirim > 0.0
    }

    pub fn can_attempt_goal_point(&self) -> bool {
        self.drives_in_series >= GOAL_POINT_REQUIRED_DRIVES && !self.is_bonus_phase
    }

    pub fn can_attempt_field_point(&self) -> bool {
        self.drives_in_series >= FIELD_POINT_REQUIRED_DRIVES
            && (self.normalized_proximity >= 0.6
                || self.zone == PitchZone::SecondZone
                || self.zone == PitchZone::FirstZone)
    }

    pub fn determine_zone_from_proximity(normalized_proximity: f64) -> PitchZone {
        let p = normalized_proximity.clamp(0.0, 1.0);
        if p >= 0.88 {
            PitchZone::FirstZone
        } else if p >= 0.72 {
            PitchZone::SecondZone
        } else {
            PitchZone::OpenField
        }
    }

    pub fn with_advance(&self, mirins: f64, pitch_length_mirim: f64) -> Self {
        let new_norm_prox =
            (self.normalized_proximity + mirins / pitch_length_mirim.max(1.0)).clamp(0.0, 1.0);
        let new_zone = Self::determine_zone_from_proximity(new_norm_prox);
        let (new_down, new_rem) = if mirins >= self.remaining_advance_mirim {
            (1, 10.0)
        } else {
            (
                self.down.saturating_add(1).min(4),
                (self.remaining_advance_mirim - mirins).max(0.0),
            )
        };
        Self {
            down: new_down,
            remaining_advance_mirim: new_rem,
            zone: new_zone,
            channel: self.channel,
            normalized_proximity: new_norm_prox,
            drives_in_series: self.drives_in_series,
            is_bonus_phase: self.is_bonus_phase,
        }
    }

    pub fn with_drive_increment(&self, additional_drives: u32) -> Self {
        Self {
            drives_in_series: self.drives_in_series + additional_drives,
            ..*self
        }
    }

    pub fn with_channel(&self, channel: ArtroPlacement) -> Self {
        Self { channel, ..*self }
    }

    pub fn with_bonus_phase(&self, is_bonus_phase: bool) -> Self {
        Self {
            is_bonus_phase,
            ..*self
        }
    }

    pub fn reset_for_new_series(&self, normalized_proximity: f64, is_bonus_phase: bool) -> Self {
        let zone = Self::determine_zone_from_proximity(normalized_proximity);
        Self {
            down: 1,
            remaining_advance_mirim: 10.0,
            zone,
            channel: self.channel,
            normalized_proximity,
            drives_in_series: 0,
            is_bonus_phase,
        }
    }
}

impl Default for PitchState {
    fn default() -> Self {
        Self::initial()
    }
}