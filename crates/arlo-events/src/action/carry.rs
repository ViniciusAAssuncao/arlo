use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarryResolved {
    carrier_id: Uuid,
    gain_mirim: f64,
}

impl CarryResolved {
    pub fn new(carrier_id: Uuid, gain_mirim: f64) -> Self {
        Self {
            carrier_id,
            gain_mirim,
        }
    }

    pub fn carrier_id(&self) -> Uuid {
        self.carrier_id
    }

    pub fn gain_mirim(&self) -> f64 {
        self.gain_mirim
    }
}
