use rand::Rng;
use rand_distr::{Beta, Distribution};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BetaBelief {
    alpha: f64,
    beta: f64,
    observations: u32,
}

impl BetaBelief {
    pub fn new(alpha: f64, beta: f64) -> Self {
        Self {
            alpha: alpha.max(1e-4),
            beta: beta.max(1e-4),
            observations: 0,
        }
    }

    pub fn from_prior_mean_and_strength(prior_mean: f64, strength: f64) -> Self {
        let mean = prior_mean.clamp(0.0001, 0.9999);
        let s = strength.max(1e-4);
        let alpha = mean * s;
        let beta = (1.0 - mean) * s;
        Self {
            alpha: alpha.max(1e-4),
            beta: beta.max(1e-4),
            observations: 0,
        }
    }

    pub fn alpha(&self) -> f64 {
        self.alpha
    }

    pub fn beta(&self) -> f64 {
        self.beta
    }

    pub fn observations(&self) -> u32 {
        self.observations
    }

    pub fn mean(&self) -> f64 {
        let total = self.alpha + self.beta;
        if total > 0.0 {
            self.alpha / total
        } else {
            0.5
        }
    }

    pub fn update(&mut self, success: bool) {
        if success {
            self.alpha += 1.0;
        } else {
            self.beta += 1.0;
        }
        self.observations = self.observations.saturating_add(1);
    }

    pub fn update_with_decay(&mut self, success: bool, decay_factor: f64) {
        let decay = decay_factor.clamp(0.0, 1.0);
        self.alpha = (self.alpha * decay).max(1e-4);
        self.beta = (self.beta * decay).max(1e-4);
        if success {
            self.alpha += 1.0;
        } else {
            self.beta += 1.0;
        }
        self.observations = self.observations.saturating_add(1);
    }

    pub fn sample_thompson<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        match Beta::new(self.alpha, self.beta) {
            Ok(dist) => dist.sample(rng),
            Err(_) => self.mean(),
        }
    }
}

impl Default for BetaBelief {
    fn default() -> Self {
        Self::new(1.0, 1.0)
    }
}
