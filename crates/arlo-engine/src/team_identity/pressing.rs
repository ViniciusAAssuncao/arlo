use arlo_tactics::{EngagementBias, PressingIntensity};

pub fn contest_radius_multiplier(pressing_intensity: PressingIntensity) -> f64 {
    1.0 + pressing_intensity.value()
}

pub fn individual_contest_radius_multiplier(
    team_pressing_multiplier: f64,
    engagement_bias: EngagementBias,
) -> f64 {
    team_pressing_multiplier * (1.0 + engagement_bias.value())
}