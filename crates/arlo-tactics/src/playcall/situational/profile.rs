use crate::playcall::situational::constants::{SITUATIONAL_SIGMA_MAX, SITUATIONAL_SIGMA_MIN};
use crate::playcall::situational::context::SituationalContext;
use arlo_math::stats::UnipolarScalar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SituationalProfile {
    down_pressure: UnipolarScalar,
    distance_urgency: UnipolarScalar,
    scoring_proximity: UnipolarScalar,
    drive_scarcity: UnipolarScalar,
    targeting_flexibility: UnipolarScalar,
}

impl SituationalProfile {
    pub fn new_clamped(
        down_pressure: f64,
        distance_urgency: f64,
        scoring_proximity: f64,
        drive_scarcity: f64,
        targeting_flexibility: f64,
    ) -> Self {
        Self {
            down_pressure: UnipolarScalar::new_clamped(down_pressure),
            distance_urgency: UnipolarScalar::new_clamped(distance_urgency),
            scoring_proximity: UnipolarScalar::new_clamped(scoring_proximity),
            drive_scarcity: UnipolarScalar::new_clamped(drive_scarcity),
            targeting_flexibility: UnipolarScalar::new_clamped(targeting_flexibility),
        }
    }

    pub fn down_pressure(&self) -> UnipolarScalar {
        self.down_pressure
    }

    pub fn distance_urgency(&self) -> UnipolarScalar {
        self.distance_urgency
    }

    pub fn scoring_proximity(&self) -> UnipolarScalar {
        self.scoring_proximity
    }

    pub fn drive_scarcity(&self) -> UnipolarScalar {
        self.drive_scarcity
    }

    pub fn targeting_flexibility(&self) -> UnipolarScalar {
        self.targeting_flexibility
    }

    pub fn fit_score(&self, context: &SituationalContext) -> f64 {
        let sigma = SITUATIONAL_SIGMA_MIN
            + self.targeting_flexibility.value() * (SITUATIONAL_SIGMA_MAX - SITUATIONAL_SIGMA_MIN);
        let two_sigma_sq = 2.0 * sigma * sigma;

        let k_down = (-((self.down_pressure.value() - context.down_pressure()).powi(2))
            / two_sigma_sq)
            .exp();
        let k_dist = (-((self.distance_urgency.value() - context.distance_urgency()).powi(2))
            / two_sigma_sq)
            .exp();
        let k_score = (-((self.scoring_proximity.value() - context.scoring_proximity()).powi(2))
            / two_sigma_sq)
            .exp();
        let k_drive = (-((self.drive_scarcity.value() - context.drive_scarcity()).powi(2))
            / two_sigma_sq)
            .exp();

        k_down * k_dist * k_score * k_drive
    }
}

impl Default for SituationalProfile {
    fn default() -> Self {
        Self::new_clamped(0.0, 0.0, 0.0, 0.0, 0.0)
    }
}
