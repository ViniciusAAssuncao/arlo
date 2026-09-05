use arlo_math::units::{Length, MIRIM_TO_METERS};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PitchCoordinates {
    x: Length,
    y: Length,
}

impl PitchCoordinates {
    pub fn new(x: Length, y: Length) -> Self {
        Self { x, y }
    }

    pub fn from_meters(x_m: f64, y_m: f64) -> Self {
        Self {
            x: Length::new(x_m),
            y: Length::new(y_m),
        }
    }

    pub fn from_mirim(x_mirim: f64, y_mirim: f64) -> Self {
        Self {
            x: Length::new(x_mirim * MIRIM_TO_METERS),
            y: Length::new(y_mirim * MIRIM_TO_METERS),
        }
    }

    pub fn x(&self) -> Length {
        self.x
    }

    pub fn y(&self) -> Length {
        self.y
    }

    pub fn x_mirim(&self) -> f64 {
        self.x.value() / MIRIM_TO_METERS
    }

    pub fn y_mirim(&self) -> f64 {
        self.y.value() / MIRIM_TO_METERS
    }

    pub fn x_meters(&self) -> f64 {
        self.x.value()
    }

    pub fn y_meters(&self) -> f64 {
        self.y.value()
    }
}
