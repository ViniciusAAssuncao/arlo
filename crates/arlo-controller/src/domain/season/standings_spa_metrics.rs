use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct SpaMetrics {
    pb: f64,
    feo: f64,
    ispa: f64,
}

impl SpaMetrics {
    pub fn new(pb: f64, feo: f64, ispa: f64) -> Self {
        Self { pb, feo, ispa }
    }

    pub fn pb(&self) -> f64 {
        self.pb
    }

    pub fn feo(&self) -> f64 {
        self.feo
    }

    pub fn ispa(&self) -> f64 {
        self.ispa
    }
}
