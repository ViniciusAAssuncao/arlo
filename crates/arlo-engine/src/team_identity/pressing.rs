use arlo_tactics::PressingIntensity;

pub fn contest_radius_multiplier(pressing_intensity: PressingIntensity) -> f64 {
    1.0 + pressing_intensity.value()
}