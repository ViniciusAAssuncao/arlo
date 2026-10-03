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
    calculate_confidence_from_evidence(
        effective_opportunities as f64,
        seconds_played,
        config,
    )
}

pub fn calculate_confidence_from_evidence(
    effective_evidence: f64,
    seconds_played: f64,
    config: &LiveRatingConfig,
) -> PerformanceConfidence {
    let evidence_cap = config.opportunities_to_full_confidence().max(1.0);
    let sec_cap = config.seconds_to_full_confidence().max(1.0);
    let evidence_ratio = (effective_evidence.max(0.0) / evidence_cap).min(1.0);
    let sec_ratio = (seconds_played.max(0.0) / sec_cap).min(1.0);
    let raw = (evidence_ratio * 0.60 + sec_ratio * 0.40).clamp(0.0, 1.0);
    PerformanceConfidence::new_clamped(raw)
}

pub fn calculate_quality_latent(
    quality_signal: f64,
    config: &LiveRatingConfig,
) -> f64 {
    (quality_signal - config.quality_center()) * config.quality_latent_scale()
}

pub fn calculate_impact_signal(
    high_impact_total: f64,
    opportunity_weight: f64,
    config: &LiveRatingConfig,
) -> f64 {
    let denominator =
        (opportunity_weight.max(0.0) + config.impact_weight_offset().max(1e-4)).sqrt();
    if denominator <= 0.0 {
        0.0
    } else {
        high_impact_total / denominator
    }
}

pub fn calculate_impact_adjustment(
    impact_signal: f64,
    confidence: PerformanceConfidence,
    config: &LiveRatingConfig,
) -> f64 {
    let raw_adjustment = if impact_signal > config.impact_positive_threshold() {
        let excess = impact_signal - config.impact_positive_threshold();
        config.impact_positive_max()
            * (excess / config.impact_scale().max(1e-4)).tanh()
    } else if impact_signal < -config.impact_negative_threshold() {
        let excess = -impact_signal - config.impact_negative_threshold();
        -config.impact_negative_max()
            * (excess / config.impact_scale().max(1e-4)).tanh()
    } else {
        0.0
    };

    raw_adjustment * confidence_weight(confidence, config)
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

    let adjusted_delta = delta * confidence_weight(confidence, config);

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

fn confidence_weight(
    confidence: PerformanceConfidence,
    config: &LiveRatingConfig,
) -> f64 {
    let shrinkage = config.confidence_shrinkage_weight().clamp(0.0, 1.0);
    1.0 - shrinkage * (1.0 - confidence.value())
}
