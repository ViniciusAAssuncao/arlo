use crate::resolution::outcome::DuelOutcome;
use crate::resolution::progression_strategy::ProgressionResolutionStrategy;
use arlo_domain::sport_constants::PITCH_LENGTH_MIRIM_MAX;
use rand::Rng;
use rand_distr::{Distribution, Gamma};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AggregateProgressionStrategy {
    shape: f64,
    base_mean: f64,
    advantage_factor: f64,
    min_mean: f64,
}

impl AggregateProgressionStrategy {
    pub fn new(shape: f64, base_mean: f64, advantage_factor: f64, min_mean: f64) -> Self {
        Self {
            shape,
            base_mean,
            advantage_factor,
            min_mean,
        }
    }

    pub fn shape(&self) -> f64 {
        self.shape
    }

    pub fn base_mean(&self) -> f64 {
        self.base_mean
    }

    pub fn advantage_factor(&self) -> f64 {
        self.advantage_factor
    }

    pub fn min_mean(&self) -> f64 {
        self.min_mean
    }
}

impl Default for AggregateProgressionStrategy {
    fn default() -> Self {
        Self {
            shape: 2.0,
            base_mean: 3.0,
            advantage_factor: 0.35,
            min_mean: 0.2,
        }
    }
}

impl ProgressionResolutionStrategy for AggregateProgressionStrategy {
    fn resolve_progression<R: Rng + ?Sized>(&self, duel_outcome: &DuelOutcome, rng: &mut R) -> f64 {
        let advantage = duel_outcome.net_advantage();
        let mean = (self.base_mean + advantage * self.advantage_factor).max(self.min_mean);
        let scale = mean / self.shape;

        if let Ok(gamma) = Gamma::new(self.shape, scale) {
            let sampled = gamma.sample(rng);
            sampled.clamp(0.0, PITCH_LENGTH_MIRIM_MAX)
        } else {
            mean.clamp(0.0, PITCH_LENGTH_MIRIM_MAX)
        }
    }
}
