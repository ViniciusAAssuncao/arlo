use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FaultOffenderRole {
    Offense,
    Defense,
    Either,
}

impl FaultOffenderRole {
    pub fn from_catalog_value(value: &str) -> Option<Self> {
        match value {
            "Offense" => Some(Self::Offense),
            "Defense" => Some(Self::Defense),
            "Either" => Some(Self::Either),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FaultActivation {
    fault_code: String,
    context: String,
    offender_role: FaultOffenderRole,
    weight: f64,
}

impl FaultActivation {
    pub fn new(
        fault_code: String,
        context: String,
        offender_role: FaultOffenderRole,
        weight: f64,
    ) -> Option<Self> {
        if fault_code.is_empty() || context.is_empty() || !weight.is_finite() || weight <= 0.0 {
            return None;
        }
        Some(Self {
            fault_code,
            context,
            offender_role,
            weight,
        })
    }

    pub fn fault_code(&self) -> &str {
        &self.fault_code
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn offender_role(&self) -> FaultOffenderRole {
        self.offender_role
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}
