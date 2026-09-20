use arlo_domain::sport_constants::PITCH_LENGTH_MIRIM_MAX;
use arlo_domain::ArtrineDecisionKind;
use rand::Rng;
use rand_distr::{Distribution, Gamma};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionProgressionKind {
    Carry,
    ShortPass,
    LongLaunch,
    Cross,
}

impl ActionProgressionKind {
    pub fn from_decision_kind(kind: ArtrineDecisionKind) -> Option<Self> {
        match kind {
            ArtrineDecisionKind::SelfCarry => Some(Self::Carry),
            ArtrineDecisionKind::ShortPass => Some(Self::ShortPass),
            ArtrineDecisionKind::LongLaunch => Some(Self::LongLaunch),
            ArtrineDecisionKind::Cross => Some(Self::Cross),
            ArtrineDecisionKind::SelfFinish => None,
        }
    }

    pub fn distribution_params(self, multiplier: f64) -> (f64, f64, f64, f64, f64) {
        let mult = multiplier.max(0.1);
        match self {
            Self::Carry => (2.5, 9.5 * mult, 0.65, 1.0, 45.0),
            Self::ShortPass => (2.8, 12.0 * mult, 0.55, 2.0, 35.0),
            Self::LongLaunch => (2.2, 24.0 * mult, 0.95, 4.0, 65.0),
            Self::Cross => (2.5, 11.5 * mult, 0.50, 2.0, 30.0),
        }
    }
}

pub fn sample_action_progression<R: Rng + ?Sized>(
    kind: ActionProgressionKind,
    margin: f64,
    multiplier: f64,
    rng: &mut R,
) -> f64 {
    let (shape, base_mean, advantage_factor, min_mean, max_mean) =
        kind.distribution_params(multiplier);
    let mean = (base_mean + margin * advantage_factor).clamp(min_mean, max_mean);
    let scale = mean / shape;

    if let Ok(gamma) = Gamma::new(shape, scale) {
        gamma.sample(rng).clamp(0.0, PITCH_LENGTH_MIRIM_MAX)
    } else {
        mean.clamp(0.0, PITCH_LENGTH_MIRIM_MAX)
    }
}

pub fn finish_distance_multiplier(normalized_proximity: f64) -> f64 {
    let p = normalized_proximity.clamp(0.0, 1.0);
    0.30 + 1.70 * p.powf(1.6)
}

pub fn scoring_distance_adjustment(territory_advance_mirim: f64, is_valid: bool) -> f64 {
    if !is_valid {
        return -5.0;
    }
    let normalized_advance = (territory_advance_mirim / 20.0).clamp(0.0, 1.5);
    (normalized_advance - 0.5) * 2.0
}
