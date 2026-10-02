use crate::performance::diagnostics::{
    PerformanceCategoryContribution, PerformanceDiagnostics,
};
use crate::performance::live::category_signal::{policy_for, reliability};
use crate::performance::live::positional_relevance::relevance_for;
use crate::performance::observation::{
    ObservationCategory, PerformanceObservation, PossessionPhase,
};
use crate::performance::profile::PerformanceProfile;
use crate::performance::rating::PerformanceBreakdown;
use arlo_domain::Position;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
struct CategoryAccumulator {
    latent_contribution: f64,
    rating_latent_contribution: f64,
    observations: u32,
    opportunity_weight: f64,
    rating_opportunity_weight: f64,
    relevance_weight: f64,
    relevance_exposure: f64,
    rating_high_impact: f64,
    breakdown: PerformanceBreakdown,
}

impl CategoryAccumulator {
    fn positional_relevance(&self) -> f64 {
        if self.relevance_exposure > 0.0 {
            self.relevance_weight / self.relevance_exposure
        } else {
            1.0
        }
    }

    fn rating_latent_contribution(&self) -> f64 {
        if self.rating_opportunity_weight > 0.0 {
            self.rating_latent_contribution
        } else {
            self.latent_contribution
        }
    }

    fn rating_opportunity_weight(&self) -> f64 {
        if self.rating_opportunity_weight > 0.0 {
            self.rating_opportunity_weight
        } else {
            self.opportunity_weight
        }
    }

    fn rating_high_impact(&self) -> f64 {
        if self.relevance_exposure > 0.0 {
            self.rating_high_impact
        } else {
            self.breakdown.high_impact()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct LivePerformanceDiagnosticsState {
    effective_opportunity_weight: f64,
    categories: HashMap<ObservationCategory, CategoryAccumulator>,
}

impl LivePerformanceDiagnosticsState {
    pub fn effective_opportunity_weight(&self) -> f64 {
        self.effective_opportunity_weight
    }

    pub fn quality_signal(&self) -> Option<f64> {
        let mut weighted_quality = 0.0;
        let mut total_weight = 0.0;

        for (&category, data) in &self.categories {
            if data.opportunity_weight <= 0.0 {
                continue;
            }
            let policy = policy_for(category);
            let reliability =
                reliability(data.opportunity_weight, policy.saturation);
            let rating_opportunity = data.rating_opportunity_weight();
            if rating_opportunity <= 0.0 {
                continue;
            }
            let rate = data.rating_latent_contribution() / rating_opportunity;
            let quality = (rate / policy.rate_scale).tanh();
            let weight =
                policy.importance * reliability * data.positional_relevance();
            weighted_quality += quality * weight;
            total_weight += weight;
        }

        (total_weight > 0.0).then_some(weighted_quality / total_weight)
    }

    pub fn confidence_evidence(&self) -> f64 {
        self.categories
            .iter()
            .map(|(&category, data)| {
                let policy = policy_for(category);
                policy.saturation
                    * reliability(data.opportunity_weight, policy.saturation)
            })
            .sum()
    }

    pub fn high_impact_total(&self) -> f64 {
        self.categories
            .values()
            .map(CategoryAccumulator::rating_high_impact)
            .sum()
    }

    pub fn record(
        &mut self,
        observation: &PerformanceObservation,
        scaled_breakdown: PerformanceBreakdown,
        offensive_profile: &PerformanceProfile,
        defensive_profile: &PerformanceProfile,
        offensive_position: Position,
        defensive_position: Position,
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

        let relevance = relevance_for(
            offensive_position,
            defensive_position,
            observation.category(),
            observation.phase(),
        );
        let opportunity = observation.opportunity_value();
        let entry = self.categories.entry(observation.category()).or_default();
        entry.latent_contribution += latent;
        entry.rating_latent_contribution += latent * relevance;
        entry.observations = entry.observations.saturating_add(1);
        entry.opportunity_weight += opportunity;
        entry.rating_opportunity_weight += opportunity * relevance;
        entry.relevance_weight += opportunity * relevance;
        entry.relevance_exposure += opportunity;
        entry.rating_high_impact += scaled_breakdown.high_impact() * relevance;
        entry.breakdown = entry.breakdown + scaled_breakdown;
    }

    pub fn snapshot(
        &self,
        offensive_latent: f64,
        defensive_latent: f64,
        quality_signal: f64,
        confidence_evidence: f64,
        rating_latent: f64,
        impact_signal: f64,
        impact_adjustment: f64,
    ) -> PerformanceDiagnostics {
        let mut categories: Vec<_> = self
            .categories
            .iter()
            .map(|(&category, data)| {
                PerformanceCategoryContribution::new(
                    category,
                    data.latent_contribution,
                    data.rating_latent_contribution(),
                    data.observations,
                    data.opportunity_weight,
                    data.rating_opportunity_weight(),
                    data.positional_relevance(),
                    data.rating_high_impact(),
                    data.breakdown,
                )
            })
            .collect();

        categories.sort_by_key(|entry| entry.category().as_str());

        PerformanceDiagnostics::new(
            self.effective_opportunity_weight,
            offensive_latent,
            defensive_latent,
            quality_signal,
            confidence_evidence,
            rating_latent,
            impact_signal,
            impact_adjustment,
            categories,
        )
    }
}
