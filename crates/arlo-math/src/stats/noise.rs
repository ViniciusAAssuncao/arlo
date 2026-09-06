use rand::Rng;
use rand_distr::{Distribution, Normal};

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
