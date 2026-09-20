use crate::scoring_model::tuning::ScoringDifficultyProfile;

pub fn scale_rating_gap(rating_gap: f64, saturation_point: f64, slope: f64) -> f64 {
    if saturation_point <= 0.0 {
        return rating_gap * slope;
    }
    saturation_point * ((slope * rating_gap) / saturation_point).tanh()
}

pub fn scale_rating_gap_with_profile(rating_gap: f64, profile: &ScoringDifficultyProfile) -> f64 {
    scale_rating_gap(rating_gap, profile.rating_gap_saturation_point(), profile.rating_gap_slope())
}

pub fn smooth_rating_gap_saturation(rating_gap: f64, saturation_point: f64, slope: f64) -> f64 {
    scale_rating_gap(rating_gap, saturation_point, slope)
}

pub fn calculate_scaled_rating_gap(rating_gap: f64, saturation_point: f64, slope: f64) -> f64 {
    scale_rating_gap(rating_gap, saturation_point, slope)
}
