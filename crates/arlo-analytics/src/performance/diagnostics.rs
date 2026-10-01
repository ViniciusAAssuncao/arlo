use crate::performance::observation::ObservationCategory;
use crate::performance::rating::PerformanceBreakdown;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceCategoryContribution {
    category: ObservationCategory,
    latent_contribution: f64,
    observations: u32,
    opportunity_weight: f64,
    breakdown: PerformanceBreakdown,
}

impl PerformanceCategoryContribution {
    pub fn new(
        category: ObservationCategory,
        latent_contribution: f64,
        observations: u32,
        opportunity_weight: f64,
        breakdown: PerformanceBreakdown,
    ) -> Self {
        Self {
            category,
            latent_contribution,
            observations,
            opportunity_weight,
            breakdown,
        }
    }

    pub fn category(&self) -> ObservationCategory {
        self.category
    }

    pub fn latent_contribution(&self) -> f64 {
        self.latent_contribution
    }

    pub fn observations(&self) -> u32 {
        self.observations
    }

    pub fn opportunity_weight(&self) -> f64 {
        self.opportunity_weight
    }

    pub fn breakdown(&self) -> &PerformanceBreakdown {
        &self.breakdown
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceDiagnostics {
    effective_opportunity_weight: f64,
    offensive_latent: f64,
    defensive_latent: f64,
    raw_latent: f64,
    category_contributions: Vec<PerformanceCategoryContribution>,
}

impl PerformanceDiagnostics {
    pub fn new(
        effective_opportunity_weight: f64,
        offensive_latent: f64,
        defensive_latent: f64,
        category_contributions: Vec<PerformanceCategoryContribution>,
    ) -> Self {
        Self {
            effective_opportunity_weight,
            offensive_latent,
            defensive_latent,
            raw_latent: offensive_latent + defensive_latent,
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

    pub fn category_contributions(&self) -> &[PerformanceCategoryContribution] {
        &self.category_contributions
    }
}
