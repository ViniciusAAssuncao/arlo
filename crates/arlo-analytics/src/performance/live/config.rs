use crate::performance::rating::OutcomeAdjustmentPolicy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct LiveRatingConfig {
    baseline_rating: f64,
    positive_scale: f64,
    negative_scale: f64,
    opportunities_to_full_confidence: f64,
    seconds_to_full_confidence: f64,
    confidence_shrinkage_weight: f64,
    shrinkage_prior_opportunities: f64,
    quality_center: f64,
    quality_latent_scale: f64,
    impact_positive_threshold: f64,
    impact_negative_threshold: f64,
    impact_positive_max: f64,
    impact_negative_max: f64,
    impact_scale: f64,
    impact_weight_offset: f64,
    record_snapshots_automatically: bool,
    outcome_policy: OutcomeAdjustmentPolicy,
}

impl Default for LiveRatingConfig {
    fn default() -> Self {
        Self {
            baseline_rating: 6.2,
            positive_scale: 4.0,
            negative_scale: 4.0,
            opportunities_to_full_confidence: 55.0,
            seconds_to_full_confidence: 3600.0,
            confidence_shrinkage_weight: 0.85,
            shrinkage_prior_opportunities: 4.0,
            quality_center: 0.10,
            quality_latent_scale: 0.22,
            impact_positive_threshold: 0.30,
            impact_negative_threshold: 0.55,
            impact_positive_max: 1.35,
            impact_negative_max: 0.65,
            impact_scale: 0.55,
            impact_weight_offset: 16.0,
            record_snapshots_automatically: false,
            outcome_policy: OutcomeAdjustmentPolicy::zero(),
        }
    }
}

impl LiveRatingConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn baseline_rating(&self) -> f64 {
        self.baseline_rating
    }

    pub fn positive_scale(&self) -> f64 {
        self.positive_scale
    }

    pub fn negative_scale(&self) -> f64 {
        self.negative_scale
    }

    pub fn opportunities_to_full_confidence(&self) -> f64 {
        self.opportunities_to_full_confidence
    }

    pub fn seconds_to_full_confidence(&self) -> f64 {
        self.seconds_to_full_confidence
    }

    pub fn confidence_shrinkage_weight(&self) -> f64 {
        self.confidence_shrinkage_weight
    }

    pub fn shrinkage_prior_opportunities(&self) -> f64 {
        self.shrinkage_prior_opportunities
    }

    pub fn quality_center(&self) -> f64 {
        self.quality_center
    }

    pub fn quality_latent_scale(&self) -> f64 {
        self.quality_latent_scale
    }

    pub fn impact_positive_threshold(&self) -> f64 {
        self.impact_positive_threshold
    }

    pub fn impact_negative_threshold(&self) -> f64 {
        self.impact_negative_threshold
    }

    pub fn impact_positive_max(&self) -> f64 {
        self.impact_positive_max
    }

    pub fn impact_negative_max(&self) -> f64 {
        self.impact_negative_max
    }

    pub fn impact_scale(&self) -> f64 {
        self.impact_scale
    }

    pub fn impact_weight_offset(&self) -> f64 {
        self.impact_weight_offset
    }

    pub fn record_snapshots_automatically(&self) -> bool {
        self.record_snapshots_automatically
    }

    pub fn outcome_policy(&self) -> &OutcomeAdjustmentPolicy {
        &self.outcome_policy
    }

    pub fn with_baseline_rating(mut self, baseline_rating: f64) -> Self {
        self.baseline_rating = baseline_rating;
        self
    }

    pub fn with_positive_scale(mut self, positive_scale: f64) -> Self {
        self.positive_scale = positive_scale;
        self
    }

    pub fn with_negative_scale(mut self, negative_scale: f64) -> Self {
        self.negative_scale = negative_scale;
        self
    }

    pub fn with_opportunities_to_full_confidence(mut self, count: f64) -> Self {
        self.opportunities_to_full_confidence = count;
        self
    }

    pub fn with_seconds_to_full_confidence(mut self, seconds: f64) -> Self {
        self.seconds_to_full_confidence = seconds;
        self
    }

    pub fn with_confidence_shrinkage_weight(mut self, weight: f64) -> Self {
        self.confidence_shrinkage_weight = weight;
        self
    }

    pub fn with_shrinkage_prior_opportunities(mut self, count: f64) -> Self {
        self.shrinkage_prior_opportunities = count;
        self
    }

    pub fn with_quality_center(mut self, quality_center: f64) -> Self {
        self.quality_center = quality_center;
        self
    }

    pub fn with_quality_latent_scale(mut self, scale: f64) -> Self {
        self.quality_latent_scale = scale;
        self
    }

    pub fn with_impact_positive_threshold(mut self, threshold: f64) -> Self {
        self.impact_positive_threshold = threshold;
        self
    }

    pub fn with_impact_negative_threshold(mut self, threshold: f64) -> Self {
        self.impact_negative_threshold = threshold;
        self
    }

    pub fn with_impact_positive_max(mut self, value: f64) -> Self {
        self.impact_positive_max = value;
        self
    }

    pub fn with_impact_negative_max(mut self, value: f64) -> Self {
        self.impact_negative_max = value;
        self
    }

    pub fn with_impact_scale(mut self, scale: f64) -> Self {
        self.impact_scale = scale;
        self
    }

    pub fn with_impact_weight_offset(mut self, offset: f64) -> Self {
        self.impact_weight_offset = offset;
        self
    }

    pub fn with_record_snapshots_automatically(mut self, record: bool) -> Self {
        self.record_snapshots_automatically = record;
        self
    }

    pub fn with_outcome_policy(mut self, policy: OutcomeAdjustmentPolicy) -> Self {
        self.outcome_policy = policy;
        self
    }
}
