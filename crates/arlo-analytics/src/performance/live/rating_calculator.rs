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

pub fn calculate_shrunk_latent(
    raw_latent: f64,
    effective_opportunities: u32,
    opportunity_weight: f64,
    config: &LiveRatingConfig,
) -> f64 {
    let prior_opps = config.shrinkage_prior_opportunities().max(0.1);
    let n = if opportunity_weight > 0.0 {
        opportunity_weight
    } else {
        effective_opportunities as f64
    };

    raw_latent / (n + prior_opps)
}

pub fn calculate_rating_from_latent(
    latent: f64,
    confidence: PerformanceConfidence,
    config: &LiveRatingConfig,
) -> PerformanceRating {
    let gain = if latent >= 0.0 {
        config.positive_scale().max(1e-4)
    } else {
        config.negative_scale().max(1e-4)
    };
    let scaled = latent * gain;
    let normalized = scaled / (1.0 + scaled * scaled).sqrt();
    let delta = if normalized >= 0.0 {
        (PerformanceRating::MAX - config.baseline_rating()) * normalized
    } else {
        (config.baseline_rating() - PerformanceRating::MIN) * normalized
    };

    let shrinkage = config.confidence_shrinkage_weight().clamp(0.0, 1.0);
    let confidence_weight = 1.0 - shrinkage * (1.0 - confidence.value());
    let adjusted_delta = delta * confidence_weight;

    PerformanceRating::new_clamped(config.baseline_rating() + adjusted_delta)
}

pub fn calculate_rating_with_exposure(
    profile: &PerformanceProfile,
    breakdown: &PerformanceBreakdown,
    effective_opportunities: u32,
    opportunity_weight: f64,
    confidence: PerformanceConfidence,
    config: &LiveRatingConfig,
) -> PerformanceRating {
    let raw_latent = profile.calculate_latent_score(breakdown);
    let shrunk_latent = calculate_shrunk_latent(
        raw_latent,
        effective_opportunities,
        opportunity_weight,
        config,
    );
    calculate_rating_from_latent(shrunk_latent, confidence, config)
}

pub fn calculate_rating(
    profile: &PerformanceProfile,
    breakdown: &PerformanceBreakdown,
    confidence: PerformanceConfidence,
    config: &LiveRatingConfig,
) -> PerformanceRating {
    calculate_rating_with_exposure(profile, breakdown, 0, 0.0, confidence, config)
}

pub fn calculate_dual_rating_with_exposure(
    offensive_profile: &PerformanceProfile,
    offensive_breakdown: &PerformanceBreakdown,
    defensive_profile: &PerformanceProfile,
    defensive_breakdown: &PerformanceBreakdown,
    effective_opportunities: u32,
    opportunity_weight: f64,
    confidence: PerformanceConfidence,
    config: &LiveRatingConfig,
) -> PerformanceRating {
    let latent_off = offensive_profile.calculate_latent_score(offensive_breakdown);
    let latent_def = defensive_profile.calculate_latent_score(defensive_breakdown);
    let raw_latent = latent_off + latent_def;
    let shrunk_latent = calculate_shrunk_latent(
        raw_latent,
        effective_opportunities,
        opportunity_weight,
        config,
    );
    calculate_rating_from_latent(shrunk_latent, confidence, config)
}

pub fn calculate_dual_rating(
    offensive_profile: &PerformanceProfile,
    offensive_breakdown: &PerformanceBreakdown,
    defensive_profile: &PerformanceProfile,
    defensive_breakdown: &PerformanceBreakdown,
    confidence: PerformanceConfidence,
    config: &LiveRatingConfig,
) -> PerformanceRating {
    calculate_dual_rating_with_exposure(
        offensive_profile,
        offensive_breakdown,
        defensive_profile,
        defensive_breakdown,
        0,
        0.0,
        confidence,
        config,
    )
}