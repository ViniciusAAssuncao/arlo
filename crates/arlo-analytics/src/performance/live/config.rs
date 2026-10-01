use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LiveRatingConfig {
    baseline_rating: f64,
    positive_scale: f64,
    negative_scale: f64,
    opportunities_to_full_confidence: f64,
    seconds_to_full_confidence: f64,
    confidence_shrinkage_weight: f64,
    shrinkage_prior_opportunities: f64,
    record_snapshots_automatically: bool,
}

impl Default for LiveRatingConfig {
    fn default() -> Self {
        Self {
            baseline_rating: 5.5,
            positive_scale: 12.0,
            negative_scale: 10.0,
            opportunities_to_full_confidence: 20.0,
            seconds_to_full_confidence: 1800.0,
            confidence_shrinkage_weight: 0.0,
            shrinkage_prior_opportunities: 4.0,
            record_snapshots_automatically: true,
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

    pub fn record_snapshots_automatically(&self) -> bool {
        self.record_snapshots_automatically
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

    pub fn with_record_snapshots_automatically(mut self, record: bool) -> Self {
        self.record_snapshots_automatically = record;
        self
    }
}