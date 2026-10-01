use crate::error::{AnalyticsError, AnalyticsResult};
use crate::performance::rating::PerformanceBreakdown;
use arlo_events::MatchClockInstant;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub enum PossessionPhase {
    #[default]
    Offense,
    Defense,
    Neutral,
}

impl PossessionPhase {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Offense => "Offense",
            Self::Defense => "Defense",
            Self::Neutral => "Neutral",
        }
    }

    pub fn is_offense(&self) -> bool {
        matches!(self, Self::Offense)
    }

    pub fn is_defense(&self) -> bool {
        matches!(self, Self::Defense)
    }

    pub fn is_neutral(&self) -> bool {
        matches!(self, Self::Neutral)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObservationCategory {
    Duel,
    Pass,
    Reception,
    Carry,
    Drive,
    ArtrineDecision,
    Recovery,
    Turnover,
    Scoring,
    Assist,
    Foul,
    Punishment,
    PasserContact,
    KickFoul,
}

impl ObservationCategory {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Duel => "Duel",
            Self::Pass => "Pass",
            Self::Reception => "Reception",
            Self::Carry => "Carry",
            Self::Drive => "Drive",
            Self::ArtrineDecision => "ArtrineDecision",
            Self::Recovery => "Recovery",
            Self::Turnover => "Turnover",
            Self::Scoring => "Scoring",
            Self::Assist => "Assist",
            Self::Foul => "Foul",
            Self::Punishment => "Punishment",
            Self::PasserContact => "PasserContact",
            Self::KickFoul => "KickFoul",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceObservation {
    player_id: Uuid,
    team_id: Uuid,
    clock: MatchClockInstant,
    phase: PossessionPhase,
    category: ObservationCategory,
    breakdown: PerformanceBreakdown,
    opportunity_value: f64,
    leverage: f64,
    description: String,
}

impl PerformanceObservation {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        clock: MatchClockInstant,
        phase: PossessionPhase,
        category: ObservationCategory,
        breakdown: PerformanceBreakdown,
        opportunity_value: f64,
        leverage: f64,
        description: impl Into<String>,
    ) -> AnalyticsResult<Self> {
        if !opportunity_value.is_finite() || opportunity_value < 0.0 {
            return Err(AnalyticsError::InvalidData(format!(
                "opportunity_value must be finite and non-negative, got {}",
                opportunity_value
            )));
        }
        if !leverage.is_finite() || leverage < 0.0 {
            return Err(AnalyticsError::InvalidData(format!(
                "leverage must be finite and non-negative, got {}",
                leverage
            )));
        }

        Ok(Self {
            player_id,
            team_id,
            clock,
            phase,
            category,
            breakdown,
            opportunity_value,
            leverage,
            description: description.into(),
        })
    }

    pub const fn new_unchecked(
        player_id: Uuid,
        team_id: Uuid,
        clock: MatchClockInstant,
        phase: PossessionPhase,
        category: ObservationCategory,
        breakdown: PerformanceBreakdown,
        opportunity_value: f64,
        leverage: f64,
        description: String,
    ) -> Self {
        Self {
            player_id,
            team_id,
            clock,
            phase,
            category,
            breakdown,
            opportunity_value,
            leverage,
            description,
        }
    }

    pub fn builder(
        player_id: Uuid,
        team_id: Uuid,
        clock: MatchClockInstant,
        category: ObservationCategory,
    ) -> PerformanceObservationBuilder {
        PerformanceObservationBuilder::new(player_id, team_id, clock, category)
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn team_id(&self) -> Uuid {
        self.team_id
    }

    pub fn clock(&self) -> MatchClockInstant {
        self.clock
    }

    pub fn phase(&self) -> PossessionPhase {
        self.phase
    }

    pub fn category(&self) -> ObservationCategory {
        self.category
    }

    pub fn breakdown(&self) -> &PerformanceBreakdown {
        &self.breakdown
    }

    pub fn execution(&self) -> f64 {
        self.breakdown.execution()
    }

    pub fn production(&self) -> f64 {
        self.breakdown.production()
    }

    pub fn defense(&self) -> f64 {
        self.breakdown.defense()
    }

    pub fn ball_security(&self) -> f64 {
        self.breakdown.ball_security()
    }

    pub fn discipline(&self) -> f64 {
        self.breakdown.discipline()
    }

    pub fn high_impact(&self) -> f64 {
        self.breakdown.high_impact()
    }

    pub fn opportunity_value(&self) -> f64 {
        self.opportunity_value
    }

    pub fn leverage(&self) -> f64 {
        self.leverage
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn with_phase(mut self, phase: PossessionPhase) -> Self {
        self.phase = phase;
        self
    }

    pub fn with_leverage(mut self, leverage: f64) -> Self {
        self.leverage = leverage;
        self
    }

    pub fn with_opportunity_value(mut self, opportunity_value: f64) -> Self {
        self.opportunity_value = opportunity_value;
        self
    }
}

#[derive(Debug, Clone)]
pub struct PerformanceObservationBuilder {
    player_id: Uuid,
    team_id: Uuid,
    clock: MatchClockInstant,
    phase: PossessionPhase,
    category: ObservationCategory,
    execution: f64,
    production: f64,
    defense: f64,
    ball_security: f64,
    discipline: f64,
    high_impact: f64,
    opportunity_value: f64,
    leverage: f64,
    description: String,
}

impl PerformanceObservationBuilder {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        clock: MatchClockInstant,
        category: ObservationCategory,
    ) -> Self {
        Self {
            player_id,
            team_id,
            clock,
            phase: PossessionPhase::Neutral,
            category,
            execution: 0.0,
            production: 0.0,
            defense: 0.0,
            ball_security: 0.0,
            discipline: 0.0,
            high_impact: 0.0,
            opportunity_value: 1.0,
            leverage: 1.0,
            description: String::new(),
        }
    }

    pub fn phase(mut self, phase: PossessionPhase) -> Self {
        self.phase = phase;
        self
    }

    pub fn execution(mut self, val: f64) -> Self {
        self.execution = val;
        self
    }

    pub fn production(mut self, val: f64) -> Self {
        self.production = val;
        self
    }

    pub fn defense(mut self, val: f64) -> Self {
        self.defense = val;
        self
    }

    pub fn ball_security(mut self, val: f64) -> Self {
        self.ball_security = val;
        self
    }

    pub fn discipline(mut self, val: f64) -> Self {
        self.discipline = val;
        self
    }

    pub fn high_impact(mut self, val: f64) -> Self {
        self.high_impact = val;
        self
    }

    pub fn opportunity_value(mut self, val: f64) -> Self {
        self.opportunity_value = val;
        self
    }

    pub fn leverage(mut self, val: f64) -> Self {
        self.leverage = val;
        self
    }

    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }

    pub fn build(self) -> AnalyticsResult<PerformanceObservation> {
        let breakdown = PerformanceBreakdown::new(
            self.execution,
            self.production,
            self.defense,
            self.ball_security,
            self.discipline,
            self.high_impact,
        )?;

        PerformanceObservation::new(
            self.player_id,
            self.team_id,
            self.clock,
            self.phase,
            self.category,
            breakdown,
            self.opportunity_value,
            self.leverage,
            self.description,
        )
    }
}
