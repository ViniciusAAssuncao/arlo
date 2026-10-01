use crate::error::AnalyticsResult;
use crate::performance::live::config::LiveRatingConfig;
use crate::performance::live::rating_calculator::{calculate_confidence, calculate_rating};
use crate::performance::observation::PerformanceObservation;
use crate::performance::profile::PerformanceProfile;
use crate::performance::rating::{
    ModelVersion, PerformanceBreakdown, PerformanceConfidence, PerformanceRating,
    PlayerPerformanceSnapshot,
};
use arlo_domain::{Position, SlotRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LivePlayerState {
    player_id: Uuid,
    team_id: Uuid,
    offensive_position: Position,
    defensive_position: Position,
    slot_role: SlotRole,
    profile: PerformanceProfile,
    is_active: bool,
    seconds_played: f64,
    effective_opportunities: u32,
    accumulated_breakdown: PerformanceBreakdown,
    performance_rating: PerformanceRating,
    confidence: PerformanceConfidence,
    outcome_adjustment: f64,
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
        let profile = PerformanceProfile::for_position_and_role(offensive_position, slot_role);
        let confidence = calculate_confidence(0, 0.0, config);
        let accumulated_breakdown = PerformanceBreakdown::zero();
        let performance_rating =
            calculate_rating(&profile, &accumulated_breakdown, confidence, config);

        Self {
            player_id,
            team_id,
            offensive_position,
            defensive_position,
            slot_role,
            profile,
            is_active,
            seconds_played: 0.0,
            effective_opportunities: 0,
            accumulated_breakdown,
            performance_rating,
            confidence,
            outcome_adjustment: 0.0,
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
        &self.profile
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

    pub fn effective_opportunities(&self) -> u32 {
        self.effective_opportunities
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

    pub fn outcome_adjustment(&self) -> f64 {
        self.outcome_adjustment
    }

    pub fn set_outcome_adjustment(&mut self, adjustment: f64) {
        self.outcome_adjustment = adjustment;
    }

    pub fn advance_time(&mut self, dt_seconds: f64, config: &LiveRatingConfig) {
        if self.is_active && dt_seconds > 0.0 {
            self.seconds_played += dt_seconds;
            self.recalculate(config);
        }
    }

    pub fn apply_observation(
        &mut self,
        observation: &PerformanceObservation,
        config: &LiveRatingConfig,
    ) {
        let factor = observation.opportunity_value() * observation.leverage();
        let scaled_bd = PerformanceBreakdown::new_unchecked(
            observation.execution() * factor,
            observation.production() * factor,
            observation.defense() * factor,
            observation.ball_security() * factor,
            observation.discipline() * factor,
            observation.high_impact() * factor,
        );

        self.accumulated_breakdown = self.accumulated_breakdown + scaled_bd;
        self.effective_opportunities = self.effective_opportunities.saturating_add(1);
        self.recalculate(config);
    }

    pub fn recalculate(&mut self, config: &LiveRatingConfig) {
        self.confidence =
            calculate_confidence(self.effective_opportunities, self.seconds_played, config);
        self.performance_rating = calculate_rating(
            &self.profile,
            &self.accumulated_breakdown,
            self.confidence,
            config,
        );
    }

    pub fn to_snapshot(&self) -> PlayerPerformanceSnapshot {
        let final_rating = PerformanceRating::new_clamped(
            self.performance_rating.value() + self.outcome_adjustment,
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
            ModelVersion::default(),
        )
        .expect("valid player performance snapshot")
    }

    pub fn try_to_snapshot(&self) -> AnalyticsResult<PlayerPerformanceSnapshot> {
        let final_rating = PerformanceRating::new_clamped(
            self.performance_rating.value() + self.outcome_adjustment,
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
            ModelVersion::default(),
        )
    }
}