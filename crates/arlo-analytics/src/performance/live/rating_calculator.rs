use crate::performance::live::config::LiveRatingConfig;
use crate::performance::profile::PerformanceProfile;
use crate::performance::rating::{
    PerformanceBreakdown, PerformanceConfidence, PerformanceRating,
};

pub fn calculate_confidence(
    effective_opportunities: u32,
    seconds_played: f64,
    config: &LiveRatingConfig,
) -> PerformanceConfidence {
    let opp_cap = config.opportunities_to_full_confidence().max(1.0);
    let sec_cap = config.seconds_to_full_confidence().max(1.0);
    let opp_ratio = (effective_opportunities as f64 / opp_cap).min(1.0);
    let sec_ratio = (seconds_played / sec_cap).min(1.0);
    let raw = (opp_ratio * 0.70 + sec_ratio * 0.30).clamp(0.0, 1.0);
    PerformanceConfidence::new_clamped(raw)
}

pub fn calculate_rating(
    profile: &PerformanceProfile,
    breakdown: &PerformanceBreakdown,
    confidence: PerformanceConfidence,
    config: &LiveRatingConfig,
) -> PerformanceRating {
    let latent = profile.calculate_latent_score(breakdown);
    let delta = if latent >= 0.0 {
        let scale = config.positive_scale().max(1e-4);
        (PerformanceRating::MAX - config.baseline_rating()) * latent
            / (scale * scale + latent * latent).sqrt()
    } else {
        let scale = config.negative_scale().max(1e-4);
        (config.baseline_rating() - PerformanceRating::MIN) * latent
            / (scale * scale + latent * latent).sqrt()
    };

    let shrinkage = config.confidence_shrinkage_weight().clamp(0.0, 1.0);
    let confidence_weight = 1.0 - shrinkage * (1.0 - confidence.value());
    let adjusted_delta = delta * confidence_weight;

    PerformanceRating::new_clamped(config.baseline_rating() + adjusted_delta)
}