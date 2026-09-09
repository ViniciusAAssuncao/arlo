use rand::Rng;
use rand_distr::{Distribution, Normal, SkewNormal};
use serde::{Deserialize, Serialize};

pub fn sample_gaussian_noise<R: Rng + ?Sized>(std_dev: f64, rng: &mut R) -> f64 {
    if std_dev <= 0.0 {
        return 0.0;
    }
    match Normal::new(0.0, std_dev) {
        Ok(normal) => normal.sample(rng),
        Err(_) => 0.0,
    }
}

pub fn sample_gaussian<R: Rng + ?Sized>(mean: f64, std_dev: f64, rng: &mut R) -> f64 {
    if std_dev <= 0.0 {
        return mean;
    }
    match Normal::new(mean, std_dev) {
        Ok(normal) => normal.sample(rng),
        Err(_) => mean,
    }
}

pub fn gaussian_noise<R: Rng + ?Sized>(std_dev: f64, rng: &mut R) -> f64 {
    sample_gaussian_noise(std_dev, rng)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GaussianNoise {
    std_dev: f64,
}

impl GaussianNoise {
    pub fn new(std_dev: f64) -> Self {
        Self { std_dev }
    }

    pub fn std_dev(&self) -> f64 {
        self.std_dev
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        sample_gaussian_noise(self.std_dev, rng)
    }
}

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