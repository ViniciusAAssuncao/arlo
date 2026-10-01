use crate::error::{AnalyticsError, AnalyticsResult};
use crate::performance::diagnostics::PerformanceDiagnostics;
use arlo_domain::{Position, SlotRole};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const PLAYER_PERFORMANCE_MODEL_VERSION: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct ModelVersion(u32);

impl ModelVersion {
    pub const CURRENT: Self = Self::new(PLAYER_PERFORMANCE_MODEL_VERSION);

    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> u32 {
        self.0
    }
}

impl Default for ModelVersion {
    fn default() -> Self {
        Self::CURRENT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PerformanceRating(f64);

impl PerformanceRating {
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 10.0;
    pub const NEUTRAL: f64 = 6.2;

    pub fn new(value: f64) -> AnalyticsResult<Self> {
        if !value.is_finite() {
            return Err(AnalyticsError::NonFinite {
                field: "performance_rating".into(),
            });
        }
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(AnalyticsError::OutOfRange {
                field: "performance_rating".into(),
                min: Self::MIN,
                max: Self::MAX,
                value,
            });
        }
        Ok(Self(value))
    }

    pub fn new_clamped(value: f64) -> Self {
        if value.is_nan() {
            Self(Self::NEUTRAL)
        } else {
            Self(value.clamp(Self::MIN, Self::MAX))
        }
    }

    pub fn neutral() -> Self {
        Self(Self::NEUTRAL)
    }

    pub fn value(&self) -> f64 {
        self.0
    }

    pub fn display_rounded(&self, decimals: u32) -> f64 {
        let factor = 10_f64.powi(decimals as i32);
        (self.0 * factor).round() / factor
    }
}

impl Default for PerformanceRating {
    fn default() -> Self {
        Self::neutral()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct PerformanceConfidence(f64);

impl PerformanceConfidence {
    pub const MIN: f64 = 0.0;
    pub const MAX: f64 = 1.0;

    pub fn new(value: f64) -> AnalyticsResult<Self> {
        if !value.is_finite() {
            return Err(AnalyticsError::NonFinite {
                field: "performance_confidence".into(),
            });
        }
        if !(Self::MIN..=Self::MAX).contains(&value) {
            return Err(AnalyticsError::OutOfRange {
                field: "performance_confidence".into(),
                min: Self::MIN,
                max: Self::MAX,
                value,
            });
        }
        Ok(Self(value))
    }

    pub fn new_clamped(value: f64) -> Self {
        if value.is_nan() {
            Self(0.0)
        } else {
            Self(value.clamp(Self::MIN, Self::MAX))
        }
    }

    pub fn zero() -> Self {
        Self(0.0)
    }

    pub fn full() -> Self {
        Self(1.0)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Default for PerformanceConfidence {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct PerformanceBreakdown {
    execution: f64,
    production: f64,
    defense: f64,
    ball_security: f64,
    discipline: f64,
    high_impact: f64,
}

impl PerformanceBreakdown {
    pub fn new(
        execution: f64,
        production: f64,
        defense: f64,
        ball_security: f64,
        discipline: f64,
        high_impact: f64,
    ) -> AnalyticsResult<Self> {
        let fields = [
            ("execution", execution),
            ("production", production),
            ("defense", defense),
            ("ball_security", ball_security),
            ("discipline", discipline),
            ("high_impact", high_impact),
        ];

        for (name, val) in fields {
            if !val.is_finite() {
                return Err(AnalyticsError::NonFinite { field: name.into() });
            }
        }

        Ok(Self {
            execution,
            production,
            defense,
            ball_security,
            discipline,
            high_impact,
        })
    }

    pub const fn new_unchecked(
        execution: f64,
        production: f64,
        defense: f64,
        ball_security: f64,
        discipline: f64,
        high_impact: f64,
    ) -> Self {
        Self {
            execution,
            production,
            defense,
            ball_security,
            discipline,
            high_impact,
        }
    }

    pub fn zero() -> Self {
        Self::default()
    }

    pub fn execution(&self) -> f64 {
        self.execution
    }

    pub fn production(&self) -> f64 {
        self.production
    }

    pub fn defense(&self) -> f64 {
        self.defense
    }

    pub fn ball_security(&self) -> f64 {
        self.ball_security
    }

    pub fn discipline(&self) -> f64 {
        self.discipline
    }

    pub fn high_impact(&self) -> f64 {
        self.high_impact
    }
}

impl std::ops::Add for PerformanceBreakdown {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            execution: self.execution + rhs.execution,
            production: self.production + rhs.production,
            defense: self.defense + rhs.defense,
            ball_security: self.ball_security + rhs.ball_security,
            discipline: self.discipline + rhs.discipline,
            high_impact: self.high_impact + rhs.high_impact,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MatchOutcome {
    Win,
    Draw,
    Loss,
}

impl MatchOutcome {
    pub fn from_scores(team_score: u32, opponent_score: u32) -> Self {
        match team_score.cmp(&opponent_score) {
            std::cmp::Ordering::Greater => Self::Win,
            std::cmp::Ordering::Equal => Self::Draw,
            std::cmp::Ordering::Less => Self::Loss,
        }
    }

    pub fn is_win(&self) -> bool {
        matches!(self, Self::Win)
    }

    pub fn is_draw(&self) -> bool {
        matches!(self, Self::Draw)
    }

    pub fn is_loss(&self) -> bool {
        matches!(self, Self::Loss)
    }

    pub fn opponent_outcome(&self) -> Self {
        match self {
            Self::Win => Self::Loss,
            Self::Draw => Self::Draw,
            Self::Loss => Self::Win,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct OutcomeAdjustmentPolicy {
    win_bonus: f64,
    draw_adjustment: f64,
    loss_penalty: f64,
}

impl OutcomeAdjustmentPolicy {
    pub fn new(win_bonus: f64, draw_adjustment: f64, loss_penalty: f64) -> AnalyticsResult<Self> {
        for (name, val) in [
            ("win_bonus", win_bonus),
            ("draw_adjustment", draw_adjustment),
            ("loss_penalty", loss_penalty),
        ] {
            if !val.is_finite() {
                return Err(AnalyticsError::NonFinite { field: name.into() });
            }
        }

        Ok(Self {
            win_bonus,
            draw_adjustment,
            loss_penalty,
        })
    }

    pub fn default_policy() -> Self {
        Self {
            win_bonus: 0.15,
            draw_adjustment: 0.0,
            loss_penalty: -0.10,
        }
    }

    pub fn zero() -> Self {
        Self {
            win_bonus: 0.0,
            draw_adjustment: 0.0,
            loss_penalty: 0.0,
        }
    }

    pub fn with_win_bonus(mut self, win_bonus: f64) -> Self {
        self.win_bonus = win_bonus;
        self
    }

    pub fn with_draw_adjustment(mut self, draw_adjustment: f64) -> Self {
        self.draw_adjustment = draw_adjustment;
        self
    }

    pub fn with_loss_penalty(mut self, loss_penalty: f64) -> Self {
        self.loss_penalty = loss_penalty;
        self
    }

    pub fn adjustment_for(&self, outcome: MatchOutcome) -> f64 {
        match outcome {
            MatchOutcome::Win => self.win_bonus,
            MatchOutcome::Draw => self.draw_adjustment,
            MatchOutcome::Loss => self.loss_penalty,
        }
    }

    pub fn win_bonus(&self) -> f64 {
        self.win_bonus
    }

    pub fn draw_adjustment(&self) -> f64 {
        self.draw_adjustment
    }

    pub fn loss_penalty(&self) -> f64 {
        self.loss_penalty
    }
}

impl Default for OutcomeAdjustmentPolicy {
    fn default() -> Self {
        Self::default_policy()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PlayerPerformanceSnapshot {
    player_id: Uuid,
    team_id: Uuid,
    offensive_position: Position,
    defensive_position: Position,
    slot_role: SlotRole,
    performance_rating: PerformanceRating,
    outcome_adjustment: f64,
    final_rating: PerformanceRating,
    confidence: PerformanceConfidence,
    seconds_played: f64,
    effective_opportunities: u32,
    breakdown: PerformanceBreakdown,
    diagnostics: PerformanceDiagnostics,
    model_version: ModelVersion,
}

impl PlayerPerformanceSnapshot {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        offensive_position: Position,
        defensive_position: Position,
        slot_role: SlotRole,
        performance_rating: PerformanceRating,
        outcome_adjustment: f64,
        final_rating: PerformanceRating,
        confidence: PerformanceConfidence,
        seconds_played: f64,
        effective_opportunities: u32,
        breakdown: PerformanceBreakdown,
        diagnostics: PerformanceDiagnostics,
        model_version: ModelVersion,
    ) -> AnalyticsResult<Self> {
        if !seconds_played.is_finite() || seconds_played < 0.0 {
            return Err(AnalyticsError::NonFinite {
                field: "seconds_played".into(),
            });
        }
        if !outcome_adjustment.is_finite() {
            return Err(AnalyticsError::NonFinite {
                field: "outcome_adjustment".into(),
            });
        }

        Ok(Self {
            player_id,
            team_id,
            offensive_position,
            defensive_position,
            slot_role,
            performance_rating,
            outcome_adjustment,
            final_rating,
            confidence,
            seconds_played,
            effective_opportunities,
            breakdown,
            diagnostics,
            model_version,
        })
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

    pub fn performance_rating(&self) -> PerformanceRating {
        self.performance_rating
    }

    pub fn outcome_adjustment(&self) -> f64 {
        self.outcome_adjustment
    }

    pub fn final_rating(&self) -> PerformanceRating {
        self.final_rating
    }

    pub fn confidence(&self) -> PerformanceConfidence {
        self.confidence
    }

    pub fn seconds_played(&self) -> f64 {
        self.seconds_played
    }

    pub fn effective_opportunities(&self) -> u32 {
        self.effective_opportunities
    }

    pub fn breakdown(&self) -> &PerformanceBreakdown {
        &self.breakdown
    }

    pub fn diagnostics(&self) -> &PerformanceDiagnostics {
        &self.diagnostics
    }

    pub fn model_version(&self) -> ModelVersion {
        self.model_version
    }
}