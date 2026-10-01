use crate::performance::diagnostics::{
    PerformanceCategoryContribution, PerformanceDiagnostics,
};
use crate::performance::observation::{
    ObservationCategory, PerformanceObservation, PossessionPhase,
};
use crate::performance::profile::PerformanceProfile;
use crate::performance::rating::PerformanceBreakdown;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
struct CategoryAccumulator {
    latent_contribution: f64,
    observations: u32,
    opportunity_weight: f64,
    breakdown: PerformanceBreakdown,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LivePerformanceDiagnosticsState {
    effective_opportunity_weight: f64,
    categories: HashMap<ObservationCategory, CategoryAccumulator>,
}

impl LivePerformanceDiagnosticsState {
    pub fn effective_opportunity_weight(&self) -> f64 {
        self.effective_opportunity_weight
    }

    pub fn record(
        &mut self,
        observation: &PerformanceObservation,
        scaled_breakdown: PerformanceBreakdown,
        offensive_profile: &PerformanceProfile,
        defensive_profile: &PerformanceProfile,
    ) {
        self.effective_opportunity_weight += observation.opportunity_value();

        let latent = match observation.phase() {
            PossessionPhase::Offense => {
                offensive_profile.calculate_latent_score(&scaled_breakdown)
            }
            PossessionPhase::Defense => {
                defensive_profile.calculate_latent_score(&scaled_breakdown)
            }
            PossessionPhase::Neutral => {
                let half = PerformanceBreakdown::new_unchecked(
                    scaled_breakdown.execution() * 0.5,
                    scaled_breakdown.production() * 0.5,
                    scaled_breakdown.defense() * 0.5,
                    scaled_breakdown.ball_security() * 0.5,
                    scaled_breakdown.discipline() * 0.5,
                    scaled_breakdown.high_impact() * 0.5,
                );
                offensive_profile.calculate_latent_score(&half)
                    + defensive_profile.calculate_latent_score(&half)
            }
        };

        let entry = self.categories.entry(observation.category()).or_default();
        entry.latent_contribution += latent;
        entry.observations = entry.observations.saturating_add(1);
        entry.opportunity_weight += observation.opportunity_value();
        entry.breakdown = entry.breakdown + scaled_breakdown;
    }

    pub fn snapshot(
        &self,
        offensive_latent: f64,
        defensive_latent: f64,
    ) -> PerformanceDiagnostics {
        let mut categories: Vec<_> = self
            .categories
            .iter()
            .map(|(&category, data)| {
                PerformanceCategoryContribution::new(
                    category,
                    data.latent_contribution,
                    data.observations,
                    data.opportunity_weight,
                    data.breakdown,
                )
            })
            .collect();

        categories.sort_by_key(|entry| entry.category().as_str());

        PerformanceDiagnostics::new(
            self.effective_opportunity_weight,
            offensive_latent,
            defensive_latent,
            categories,
        )
    }
}
