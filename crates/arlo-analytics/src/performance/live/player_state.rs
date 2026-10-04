use crate::error::AnalyticsResult;
use crate::performance::live::config::LiveRatingConfig;
use crate::performance::live::diagnostics::LivePerformanceDiagnosticsState;
use crate::performance::live::rating_calculator::{
    calculate_confidence, calculate_confidence_from_evidence, calculate_dual_rating_with_exposure,
    calculate_impact_adjustment, calculate_impact_signal, calculate_quality_latent,
    calculate_rating_from_latent,
};
use crate::performance::observation::{PerformanceObservation, PossessionPhase};
use crate::performance::profile::PerformanceProfile;
use crate::performance::rating::{
    ModelVersion, PerformanceBreakdown, PerformanceConfidence, PerformanceRating,
    PlayerPerformanceSnapshot,
};
use arlo_domain::{Position, SlotRole};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivePlayerState {
    player_id: Uuid,
    team_id: Uuid,
    offensive_position: Position,
    defensive_position: Position,
    slot_role: SlotRole,
    offensive_profile: PerformanceProfile,
    defensive_profile: PerformanceProfile,
    is_active: bool,
    seconds_played: f64,
    #[serde(default)]
    position_seconds: BTreeMap<String, f64>,
    effective_opportunities: u32,
    #[serde(default)]
    diagnostics: LivePerformanceDiagnosticsState,
    #[serde(default)]
    quality_signal: f64,
    #[serde(default)]
    confidence_evidence: f64,
    #[serde(default)]
    rating_latent: f64,
    #[serde(default)]
    impact_signal: f64,
    #[serde(default)]
    impact_adjustment: f64,
    offensive_breakdown: PerformanceBreakdown,
    defensive_breakdown: PerformanceBreakdown,
    accumulated_breakdown: PerformanceBreakdown,
    performance_rating: PerformanceRating,
    confidence: PerformanceConfidence,
    outcome_adjustment: f64,
    is_starter: bool,
    entry_time_seconds: f64,
}

impl LivePlayerState {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        offensive_position: Position,
        defensive_position: Position,
        slot_role: SlotRole,
        is_active: bool,
        config: &LiveRatingConfig,
    ) -> Self {
        let offensive_profile =
            PerformanceProfile::for_position_and_role(offensive_position, slot_role);
        let defensive_profile =
            PerformanceProfile::for_position_and_role(defensive_position, slot_role);
        let confidence = calculate_confidence(0, 0.0, config);
        let offensive_breakdown = PerformanceBreakdown::zero();
        let defensive_breakdown = PerformanceBreakdown::zero();
        let accumulated_breakdown = PerformanceBreakdown::zero();
        let performance_rating = calculate_dual_rating_with_exposure(
            &offensive_profile,
            &offensive_breakdown,
            &defensive_profile,
            &defensive_breakdown,
            0,
            0.0,
            confidence,
            config,
        );

        Self {
            player_id,
            team_id,
            offensive_position,
            defensive_position,
            slot_role,
            offensive_profile,
            defensive_profile,
            is_active,
            seconds_played: 0.0,
            position_seconds: BTreeMap::new(),
            effective_opportunities: 0,
            diagnostics: LivePerformanceDiagnosticsState::default(),
            quality_signal: config.quality_center(),
            confidence_evidence: 0.0,
            rating_latent: 0.0,
            impact_signal: 0.0,
            impact_adjustment: 0.0,
            offensive_breakdown,
            defensive_breakdown,
            accumulated_breakdown,
            performance_rating,
            confidence,
            outcome_adjustment: 0.0,
            is_starter: is_active,
            entry_time_seconds: 0.0,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn offensive_position(&self) -> Position {
        self.offensive_position
    }

    pub fn defensive_position(&self) -> Position {
        self.defensive_position
    }

    pub fn slot_role(&self) -> SlotRole {
        self.slot_role
    }

    pub fn profile(&self) -> &PerformanceProfile {
        &self.offensive_profile
    }

    pub fn offensive_profile(&self) -> &PerformanceProfile {
        &self.offensive_profile
    }

    pub fn defensive_profile(&self) -> &PerformanceProfile {
        &self.defensive_profile
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_active(&mut self, is_active: bool) {
        self.is_active = is_active;
    }

    pub fn seconds_played(&self) -> f64 {
        self.seconds_played
    }

    pub fn position_seconds(&self) -> &BTreeMap<String, f64> {
        &self.position_seconds
    }

    pub fn effective_opportunities(&self) -> u32 {
        self.effective_opportunities
    }

    pub fn effective_opportunity_weight(&self) -> f64 {
        self.diagnostics.effective_opportunity_weight()
    }

    pub fn is_starter(&self) -> bool {
        self.is_starter
    }

    pub fn set_starter(&mut self, is_starter: bool) {
        self.is_starter = is_starter;
    }

    pub fn entry_time_seconds(&self) -> f64 {
        self.entry_time_seconds
    }

    pub fn set_entry_time_seconds(&mut self, entry_time: f64) {
        self.entry_time_seconds = entry_time;
    }

    pub fn offensive_breakdown(&self) -> &PerformanceBreakdown {
        &self.offensive_breakdown
    }

    pub fn defensive_breakdown(&self) -> &PerformanceBreakdown {
        &self.defensive_breakdown
    }

    pub fn accumulated_breakdown(&self) -> &PerformanceBreakdown {
        &self.accumulated_breakdown
    }

    pub fn performance_rating(&self) -> PerformanceRating {
        self.performance_rating
    }

    pub fn confidence(&self) -> PerformanceConfidence {
        self.confidence
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

    pub fn outcome_adjustment(&self) -> f64 {
        self.outcome_adjustment
    }

    pub fn set_outcome_adjustment(&mut self, adjustment: f64) {
        self.outcome_adjustment = adjustment;
    }

    pub fn clear_outcome_adjustment(&mut self) {
        self.outcome_adjustment = 0.0;
    }

    pub fn final_rating(&self) -> PerformanceRating {
        PerformanceRating::new_clamped(self.performance_rating.value() + self.outcome_adjustment)
    }

    pub fn update_assignment(
        &mut self,
        offensive_position: Position,
        defensive_position: Position,
        slot_role: SlotRole,
        config: &LiveRatingConfig,
    ) {
        self.offensive_position = offensive_position;
        self.defensive_position = defensive_position;
        self.slot_role = slot_role;
        self.offensive_profile =
            PerformanceProfile::for_position_and_role(offensive_position, slot_role);
        self.defensive_profile =
            PerformanceProfile::for_position_and_role(defensive_position, slot_role);
        self.recalculate(config);
    }

    pub fn advance_time(&mut self, dt_seconds: f64, config: &LiveRatingConfig) {
        if self.is_active && dt_seconds > 0.0 {
            self.seconds_played += dt_seconds;
            self.recalculate(config);
        }
    }

    pub fn record_possession_time(&mut self, team_id: Uuid, duration: f64) {
        if self.is_active && duration > 0.0 {
            let position = if self.team_id == team_id {
                self.offensive_position
            } else {
                self.defensive_position
            };
            *self
                .position_seconds
                .entry(format!("{position:?}"))
                .or_default() += duration;
        }
    }

    pub fn apply_observation(
        &mut self,
        observation: &PerformanceObservation,
        config: &LiveRatingConfig,
    ) {
        let opp_val = observation.opportunity_value();
        let factor = opp_val * observation.leverage();
        let scaled_bd = PerformanceBreakdown::new_unchecked(
            observation.execution() * factor,
            observation.production() * factor,
            observation.defense() * factor,
            observation.ball_security() * factor,
            observation.discipline() * factor,
            observation.high_impact() * factor,
        );

        match observation.phase() {
            PossessionPhase::Offense => {
                self.offensive_breakdown = self.offensive_breakdown + scaled_bd;
            }
            PossessionPhase::Defense => {
                self.defensive_breakdown = self.defensive_breakdown + scaled_bd;
            }
            PossessionPhase::Neutral => {
                let half_bd = PerformanceBreakdown::new_unchecked(
                    scaled_bd.execution() * 0.5,
                    scaled_bd.production() * 0.5,
                    scaled_bd.defense() * 0.5,
                    scaled_bd.ball_security() * 0.5,
                    scaled_bd.discipline() * 0.5,
                    scaled_bd.high_impact() * 0.5,
                );
                self.offensive_breakdown = self.offensive_breakdown + half_bd;
                self.defensive_breakdown = self.defensive_breakdown + half_bd;
            }
        }

        self.diagnostics.record(
            observation,
            scaled_bd,
            &self.offensive_profile,
            &self.defensive_profile,
            self.offensive_position,
            self.defensive_position,
        );
        self.accumulated_breakdown = self.offensive_breakdown + self.defensive_breakdown;
        self.effective_opportunities = self.effective_opportunities.saturating_add(1);
        self.recalculate(config);
    }

    pub fn recalculate(&mut self, config: &LiveRatingConfig) {
        self.confidence_evidence = self.diagnostics.confidence_evidence();
        self.quality_signal = self
            .diagnostics
            .quality_signal()
            .unwrap_or(config.quality_center());
        self.confidence = calculate_confidence_from_evidence(
            self.confidence_evidence,
            self.seconds_played,
            config,
        );
        self.rating_latent = calculate_quality_latent(self.quality_signal, config);
        let routine_rating =
            calculate_rating_from_latent(self.rating_latent, self.confidence, config);
        self.impact_signal = calculate_impact_signal(
            self.diagnostics.high_impact_total(),
            self.diagnostics.effective_opportunity_weight(),
            config,
        );
        self.impact_adjustment =
            calculate_impact_adjustment(self.impact_signal, self.confidence, config);
        self.performance_rating =
            PerformanceRating::new_clamped(routine_rating.value() + self.impact_adjustment);
    }

    pub fn to_snapshot(&self) -> PlayerPerformanceSnapshot {
        let final_rating = self.final_rating();
        let offensive_latent = self
            .offensive_profile
            .calculate_latent_score(&self.offensive_breakdown);
        let defensive_latent = self
            .defensive_profile
            .calculate_latent_score(&self.defensive_breakdown);
        let diagnostics = self.diagnostics.snapshot(
            offensive_latent,
            defensive_latent,
            self.quality_signal,
            self.confidence_evidence,
            self.rating_latent,
            self.impact_signal,
            self.impact_adjustment,
        );

        PlayerPerformanceSnapshot::new(
            self.player_id,
            self.team_id,
            self.offensive_position,
            self.defensive_position,
            self.slot_role,
            self.performance_rating,
            self.outcome_adjustment,
            final_rating,
            self.confidence,
            self.seconds_played,
            self.effective_opportunities,
            self.accumulated_breakdown,
            diagnostics,
            ModelVersion::default(),
        )
        .expect("valid player performance snapshot")
    }

    pub fn try_to_snapshot(&self) -> AnalyticsResult<PlayerPerformanceSnapshot> {
        let final_rating = self.final_rating();
        let offensive_latent = self
            .offensive_profile
            .calculate_latent_score(&self.offensive_breakdown);
        let defensive_latent = self
            .defensive_profile
            .calculate_latent_score(&self.defensive_breakdown);
        let diagnostics = self.diagnostics.snapshot(
            offensive_latent,
            defensive_latent,
            self.quality_signal,
            self.confidence_evidence,
            self.rating_latent,
            self.impact_signal,
            self.impact_adjustment,
        );

        PlayerPerformanceSnapshot::new(
            self.player_id,
            self.team_id,
            self.offensive_position,
            self.defensive_position,
            self.slot_role,
            self.performance_rating,
            self.outcome_adjustment,
            final_rating,
            self.confidence,
            self.seconds_played,
            self.effective_opportunities,
            self.accumulated_breakdown,
            diagnostics,
            ModelVersion::default(),
        )
    }
}
