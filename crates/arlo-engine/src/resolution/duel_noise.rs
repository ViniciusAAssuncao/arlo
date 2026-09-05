use crate::spatial::decision_vector::extract_attribute_value;
use arlo_domain::sport_constants::{ATTRIBUTE_SATURATION_THRESHOLD, BASE_NOISE_SCALE};
use arlo_domain::{AttributeKey, Player};
use rand::Rng;
use rand_distr::{Distribution, SkewNormal};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SkewNormalParams {
    pub location: f64,
    pub scale: f64,
    pub shape: f64,
}

impl SkewNormalParams {
    pub fn new(location: f64, scale: f64, shape: f64) -> Self {
        Self {
            location,
            scale,
            shape,
        }
    }

    pub fn location(&self) -> f64 {
        self.location
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn shape(&self) -> f64 {
        self.shape
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let scale = if self.scale > 0.0 { self.scale } else { 1e-6 };
        match SkewNormal::new(self.location, scale, self.shape) {
            Ok(dist) => dist.sample(rng),
            Err(_) => self.location,
        }
    }
}

pub fn player_noise_distribution(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> SkewNormalParams {
    let consistency = extract_attribute_value(player, attribute_keys, AttributeKey::Consistency);
    let technique = extract_attribute_value(player, attribute_keys, AttributeKey::Technique);
    let flair = extract_attribute_value(player, attribute_keys, AttributeKey::Flair);
    let composure = extract_attribute_value(player, attribute_keys, AttributeKey::Composure);

    let scale = BASE_NOISE_SCALE
        * (1.0 + (20.0 - consistency).max(0.0) / ATTRIBUTE_SATURATION_THRESHOLD);
    let shape = ((technique + flair) / 2.0 - composure) / ATTRIBUTE_SATURATION_THRESHOLD;
    let location = 0.0;

    SkewNormalParams::new(location, scale, shape)
}

pub fn sample_player_noise<R: Rng + ?Sized>(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    rng: &mut R,
) -> f64 {
    let params = player_noise_distribution(player, attribute_keys);
    params.sample(rng)
}