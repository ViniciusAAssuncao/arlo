use crate::domain::sport_constants::{
    AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM, FIRST_ZONE_DEPTH_MIRIM, SECOND_ZONE_DEPTH_MIRIM_MAX,
    SECOND_ZONE_DEPTH_MIRIM_MIN,
};
use crate::domain::validation::validate_float_range;
use crate::domain::InvariantViolation;
use crate::error::{DomainError, DomainResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PitchZone {
    FirstZone,
    SecondZone,
    #[default]
    OpenField,
}

impl PitchZone {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FirstZone => "FirstZone",
            Self::SecondZone => "SecondZone",
            Self::OpenField => "OpenField",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct FirstZone {
    depth_mirim: f64,
}

impl FirstZone {
    pub fn new() -> Self {
        Self {
            depth_mirim: FIRST_ZONE_DEPTH_MIRIM,
        }
    }

    pub fn depth_mirim(&self) -> f64 {
        self.depth_mirim
    }
}

impl Default for FirstZone {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SecondZone {
    depth_mirim: f64,
}

impl SecondZone {
    pub fn from_mirim(depth_mirim: f64) -> DomainResult<Self> {
        validate_float_range(
            depth_mirim,
            SECOND_ZONE_DEPTH_MIRIM_MIN,
            SECOND_ZONE_DEPTH_MIRIM_MAX,
            "second_zone_depth_mirim",
        )?;
        Ok(Self { depth_mirim })
    }

    pub fn default_awc() -> Self {
        Self {
            depth_mirim: AWC_DEFAULT_SECOND_ZONE_DEPTH_MIRIM,
        }
    }

    pub fn from_rule_value_or_default(rule_value: Option<&str>) -> DomainResult<Self> {
        match rule_value {
            Some(val) if !val.trim().is_empty() => {
                let depth_mirim =
                    val.trim()
                        .parse::<f64>()
                        .map_err(|_| DomainError::InvalidInvariant {
                            field: "second_zone_depth_mirim".to_string(),
                            violation: InvariantViolation::OutOfFloatRange {
                                min: SECOND_ZONE_DEPTH_MIRIM_MIN,
                                max: SECOND_ZONE_DEPTH_MIRIM_MAX,
                            },
                        })?;
                Self::from_mirim(depth_mirim)
            }
            _ => Ok(Self::default_awc()),
        }
    }

    pub fn depth_mirim(&self) -> f64 {
        self.depth_mirim
    }
}

impl Default for SecondZone {
    fn default() -> Self {
        Self::default_awc()
    }
}