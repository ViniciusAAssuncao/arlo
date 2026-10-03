use crate::performance::observation::ObservationCategory;
use crate::performance::rating::PerformanceBreakdown;
use serde::{Deserialize, Serialize};

fn default_positional_relevance() -> f64 {
    1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceCategoryContribution {
    category: ObservationCategory,
    latent_contribution: f64,
    #[serde(default)]
    rating_latent_contribution: f64,
    observations: u32,
    opportunity_weight: f64,
    #[serde(default)]
    rating_opportunity_weight: f64,
    #[serde(default = "default_positional_relevance")]
    positional_relevance: f64,
    #[serde(default)]
    rating_high_impact: f64,
    breakdown: PerformanceBreakdown,
}

impl PerformanceCategoryContribution {
    pub fn new(
        category: ObservationCategory,
        latent_contribution: f64,
        rating_latent_contribution: f64,
        observations: u32,
        opportunity_weight: f64,
        rating_opportunity_weight: f64,
        positional_relevance: f64,
        rating_high_impact: f64,
        breakdown: PerformanceBreakdown,
    ) -> Self {
        Self {
            category,
            latent_contribution,
            rating_latent_contribution,
            observations,
            opportunity_weight,
            rating_opportunity_weight,
            positional_relevance,
            rating_high_impact,
            breakdown,
        }
    }

    pub fn category(&self) -> ObservationCategory {
        self.category
    }

    pub fn latent_contribution(&self) -> f64 {
        self.latent_contribution
    }

    pub fn rating_latent_contribution(&self) -> f64 {
        self.rating_latent_contribution
    }

    pub fn observations(&self) -> u32 {
        self.observations
    }

    pub fn opportunity_weight(&self) -> f64 {
        self.opportunity_weight
    }

    pub fn rating_opportunity_weight(&self) -> f64 {
        self.rating_opportunity_weight
    }

    pub fn positional_relevance(&self) -> f64 {
        self.positional_relevance
    }

    pub fn rating_high_impact(&self) -> f64 {
        self.rating_high_impact
    }

    pub fn breakdown(&self) -> &PerformanceBreakdown {
        &self.breakdown
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PerformanceDiagnostics {
    effective_opportunity_weight: f64,
    offensive_latent: f64,
    defensive_latent: f64,
    raw_latent: f64,
    quality_signal: f64,
    confidence_evidence: f64,
    rating_latent: f64,
    impact_signal: f64,
    impact_adjustment: f64,
    category_contributions: Vec<PerformanceCategoryContribution>,
}

impl PerformanceDiagnostics {
    pub fn new(
        effective_opportunity_weight: f64,
        offensive_latent: f64,
        defensive_latent: f64,
        quality_signal: f64,
        confidence_evidence: f64,
        rating_latent: f64,
        impact_signal: f64,
        impact_adjustment: f64,
        category_contributions: Vec<PerformanceCategoryContribution>,
    ) -> Self {
        Self {
            effective_opportunity_weight,
            offensive_latent,
            defensive_latent,
            raw_latent: offensive_latent + defensive_latent,
            quality_signal,
            confidence_evidence,
            rating_latent,
            impact_signal,
            impact_adjustment,
            category_contributions,
        }
    }

    pub fn effective_opportunity_weight(&self) -> f64 {
        self.effective_opportunity_weight
    }

    pub fn offensive_latent(&self) -> f64 {
        self.offensive_latent
    }

    pub fn defensive_latent(&self) -> f64 {
        self.defensive_latent
    }

    pub fn raw_latent(&self) -> f64 {
        self.raw_latent
    }

    pub fn quality_signal(&self) -> f64 {
        self.quality_signal
    }

    pub fn confidence_evidence(&self) -> f64 {
        self.confidence_evidence
    }

    pub fn rating_latent(&self) -> f64 {
        self.rating_latent
    }

    pub fn impact_signal(&self) -> f64 {
        self.impact_signal
    }

    pub fn impact_adjustment(&self) -> f64 {
        self.impact_adjustment
    }

    pub fn category_contributions(&self) -> &[PerformanceCategoryContribution] {
        &self.category_contributions
    }
}
