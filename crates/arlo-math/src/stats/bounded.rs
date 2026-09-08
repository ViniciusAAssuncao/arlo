use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct BipolarScalar(f64);

impl BipolarScalar {
    pub fn new_clamped(value: f64) -> Self {
        Self(value.clamp(-1.0, 1.0))
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn lerp(&self, target: f64, t: f64) -> f64 {
        self.0 + (target - self.0) * t
    }

    pub fn abs(&self) -> f64 {
        self.0.abs()
    }
}

impl std::ops::Neg for BipolarScalar {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Default)]
pub struct UnipolarScalar(f64);

impl UnipolarScalar {
    pub fn new_clamped(value: f64) -> Self {
        Self(value.clamp(0.0, 1.0))
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn lerp(&self, target: f64, t: f64) -> f64 {
        self.0 + (target - self.0) * t
    }

    pub fn complement(&self) -> Self {
        Self(1.0 - self.0)
    }
}