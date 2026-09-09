pub mod bounded;
pub mod categorical;
pub mod contrast;
pub mod noise;
pub mod probability;
pub mod similarity;

pub use bounded::{BipolarScalar, UnipolarScalar};
pub use categorical::sample_categorical;
pub use contrast::{
    bradley_terry, bradley_terry_probability, bradley_terry_with_offset, logistic, logistic_scaled,
    softmax_weights,
};
pub use noise::{gaussian_noise, sample_gaussian, sample_gaussian_noise, GaussianNoise};
pub use probability::Probability;
pub use similarity::cosine_similarity;
