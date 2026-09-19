use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PossessionOrigin {
    origin_x_mirim: f64,
    total_advanced_mirins: f64,
}

impl PossessionOrigin {
    pub fn new(origin_x_mirim: f64) -> Self {
        Self {
            origin_x_mirim,
            total_advanced_mirins: 0.0,
        }
    }

    pub fn origin_x_mirim(&self) -> f64 {
        self.origin_x_mirim
    }

    pub fn total_advanced_mirins(&self) -> f64 {
        self.total_advanced_mirins
    }

    pub fn record_advance(&mut self, mirins: f64) {
        self.total_advanced_mirins += mirins;
    }

    pub fn reset(&mut self, new_origin_x_mirim: f64) {
        self.origin_x_mirim = new_origin_x_mirim;
        self.total_advanced_mirins = 0.0;
    }
}