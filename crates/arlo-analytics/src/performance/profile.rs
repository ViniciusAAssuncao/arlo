use crate::error::{AnalyticsError, AnalyticsResult};
use crate::performance::rating::PerformanceBreakdown;
use arlo_domain::{Position, PositionLine, SlotRole};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PerformanceProfileWeights {
    execution_weight: f64,
    production_weight: f64,
    defense_weight: f64,
    ball_security_weight: f64,
    discipline_weight: f64,
    high_impact_weight: f64,
}

impl PerformanceProfileWeights {
    pub fn new(
        execution_weight: f64,
        production_weight: f64,
        defense_weight: f64,
        ball_security_weight: f64,
        discipline_weight: f64,
        high_impact_weight: f64,
    ) -> AnalyticsResult<Self> {
        let weights = [
            ("execution_weight", execution_weight),
            ("production_weight", production_weight),
            ("defense_weight", defense_weight),
            ("ball_security_weight", ball_security_weight),
            ("discipline_weight", discipline_weight),
            ("high_impact_weight", high_impact_weight),
        ];

        for (name, val) in weights {
            if !val.is_finite() || val < 0.0 {
                return Err(AnalyticsError::InvalidData(format!(
                    "{} must be finite and non-negative, got {}",
                    name, val
                )));
            }
        }

        let sum = execution_weight
            + production_weight
            + defense_weight
            + ball_security_weight
            + discipline_weight
            + high_impact_weight;

        if sum <= 0.0 {
            return Err(AnalyticsError::InvalidData(
                "Sum of performance profile weights must be strictly positive".into(),
            ));
        }

        Ok(Self {
            execution_weight,
            production_weight,
            defense_weight,
            ball_security_weight,
            discipline_weight,
            high_impact_weight,
        })
    }

    pub const fn new_unchecked(
        execution_weight: f64,
        production_weight: f64,
        defense_weight: f64,
        ball_security_weight: f64,
        discipline_weight: f64,
        high_impact_weight: f64,
    ) -> Self {
        Self {
            execution_weight,
            production_weight,
            defense_weight,
            ball_security_weight,
            discipline_weight,
            high_impact_weight,
        }
    }

    pub fn execution_weight(&self) -> f64 {
        self.execution_weight
    }

    pub fn production_weight(&self) -> f64 {
        self.production_weight
    }

    pub fn defense_weight(&self) -> f64 {
        self.defense_weight
    }

    pub fn ball_security_weight(&self) -> f64 {
        self.ball_security_weight
    }

    pub fn discipline_weight(&self) -> f64 {
        self.discipline_weight
    }

    pub fn high_impact_weight(&self) -> f64 {
        self.high_impact_weight
    }

    pub fn total_weight(&self) -> f64 {
        self.execution_weight
            + self.production_weight
            + self.defense_weight
            + self.ball_security_weight
            + self.discipline_weight
            + self.high_impact_weight
    }

    pub fn normalize(&self) -> Self {
        let total = self.total_weight();
        if total <= 0.0 {
            return *self;
        }
        Self {
            execution_weight: self.execution_weight / total,
            production_weight: self.production_weight / total,
            defense_weight: self.defense_weight / total,
            ball_security_weight: self.ball_security_weight / total,
            discipline_weight: self.discipline_weight / total,
            high_impact_weight: self.high_impact_weight / total,
        }
    }

    pub fn apply_overlay(&self, overlay: &SlotRoleOverlay) -> Self {
        let exec = (self.execution_weight * overlay.execution_modifier).max(0.0);
        let prod = (self.production_weight * overlay.production_modifier).max(0.0);
        let def = (self.defense_weight * overlay.defense_modifier).max(0.0);
        let sec = (self.ball_security_weight * overlay.ball_security_modifier).max(0.0);
        let disc = (self.discipline_weight * overlay.discipline_modifier).max(0.0);
        let hi = (self.high_impact_weight * overlay.high_impact_modifier).max(0.0);

        Self::new_unchecked(exec, prod, def, sec, disc, hi).normalize()
    }

    pub fn calculate_latent_score(&self, breakdown: &PerformanceBreakdown) -> f64 {
        let total = self.total_weight();
        if total <= 0.0 {
            return 0.0;
        }

        let raw = breakdown.execution() * self.execution_weight
            + breakdown.production() * self.production_weight
            + breakdown.defense() * self.defense_weight
            + breakdown.ball_security() * self.ball_security_weight
            + breakdown.discipline() * self.discipline_weight
            + breakdown.high_impact() * self.high_impact_weight;

        raw / total
    }
}

pub fn base_weights_for_line(line: PositionLine) -> PerformanceProfileWeights {
    match line {
        PositionLine::OffensiveLine => PerformanceProfileWeights::new_unchecked(
            0.35, 0.15, 0.10, 0.20, 0.10, 0.10,
        ),
        PositionLine::BackLine => PerformanceProfileWeights::new_unchecked(
            0.25, 0.25, 0.10, 0.20, 0.05, 0.15,
        ),
        PositionLine::DefenseLine => PerformanceProfileWeights::new_unchecked(
            0.25, 0.05, 0.35, 0.10, 0.10, 0.15,
        ),
        PositionLine::Goalguard => PerformanceProfileWeights::new_unchecked(
            0.20, 0.05, 0.45, 0.15, 0.10, 0.05,
        ),
    }
}

pub fn weights_for_position(position: Position) -> PerformanceProfileWeights {
    match position {
        Position::CenterOffense => PerformanceProfileWeights::new_unchecked(
            0.40, 0.10, 0.15, 0.15, 0.10, 0.10,
        ),
        Position::WingOffense => PerformanceProfileWeights::new_unchecked(
            0.30, 0.25, 0.05, 0.20, 0.10, 0.10,
        ),
        Position::Midcenter => PerformanceProfileWeights::new_unchecked(
            0.35, 0.15, 0.15, 0.15, 0.10, 0.10,
        ),
        Position::TightWing => PerformanceProfileWeights::new_unchecked(
            0.35, 0.20, 0.10, 0.15, 0.10, 0.10,
        ),
        Position::CenterTight => PerformanceProfileWeights::new_unchecked(
            0.35, 0.10, 0.20, 0.15, 0.10, 0.10,
        ),
        Position::Corridor => PerformanceProfileWeights::new_unchecked(
            0.30, 0.20, 0.10, 0.20, 0.10, 0.10,
        ),
        Position::Artrine => PerformanceProfileWeights::new_unchecked(
            0.20, 0.35, 0.05, 0.20, 0.05, 0.15,
        ),
        Position::Passer => PerformanceProfileWeights::new_unchecked(
            0.35, 0.20, 0.05, 0.25, 0.05, 0.10,
        ),
        Position::PassRusher => PerformanceProfileWeights::new_unchecked(
            0.25, 0.10, 0.30, 0.10, 0.10, 0.15,
        ),
        Position::WideEnd => PerformanceProfileWeights::new_unchecked(
            0.25, 0.30, 0.05, 0.20, 0.05, 0.15,
        ),
        Position::RunningEnd => PerformanceProfileWeights::new_unchecked(
            0.25, 0.30, 0.05, 0.20, 0.05, 0.15,
        ),
        Position::Lineback => PerformanceProfileWeights::new_unchecked(
            0.25, 0.15, 0.25, 0.15, 0.10, 0.10,
        ),
        Position::Fullback => PerformanceProfileWeights::new_unchecked(
            0.30, 0.15, 0.20, 0.15, 0.10, 0.10,
        ),
        Position::Centerback => PerformanceProfileWeights::new_unchecked(
            0.25, 0.05, 0.40, 0.10, 0.10, 0.10,
        ),
        Position::DefensiveEnd => PerformanceProfileWeights::new_unchecked(
            0.25, 0.05, 0.35, 0.10, 0.10, 0.15,
        ),
        Position::Rougieback => PerformanceProfileWeights::new_unchecked(
            0.25, 0.05, 0.35, 0.15, 0.10, 0.10,
        ),
        Position::DefensiveBlocker => PerformanceProfileWeights::new_unchecked(
            0.30, 0.05, 0.40, 0.05, 0.10, 0.10,
        ),
        Position::WideBlocker => PerformanceProfileWeights::new_unchecked(
            0.30, 0.05, 0.35, 0.10, 0.10, 0.10,
        ),
        Position::OutsideZonerback => PerformanceProfileWeights::new_unchecked(
            0.20, 0.05, 0.40, 0.15, 0.10, 0.10,
        ),
        Position::MiddleZonerback => PerformanceProfileWeights::new_unchecked(
            0.20, 0.05, 0.40, 0.15, 0.10, 0.10,
        ),
        Position::Goalguard => PerformanceProfileWeights::new_unchecked(
            0.20, 0.05, 0.45, 0.15, 0.10, 0.05,
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SlotRoleOverlay {
    role: SlotRole,
    execution_modifier: f64,
    production_modifier: f64,
    defense_modifier: f64,
    ball_security_modifier: f64,
    discipline_modifier: f64,
    high_impact_modifier: f64,
}

impl SlotRoleOverlay {
    pub fn role(&self) -> SlotRole {
        self.role
    }

    pub fn execution_modifier(&self) -> f64 {
        self.execution_modifier
    }

    pub fn production_modifier(&self) -> f64 {
        self.production_modifier
    }

    pub fn defense_modifier(&self) -> f64 {
        self.defense_modifier
    }

    pub fn ball_security_modifier(&self) -> f64 {
        self.ball_security_modifier
    }

    pub fn discipline_modifier(&self) -> f64 {
        self.discipline_modifier
    }

    pub fn high_impact_modifier(&self) -> f64 {
        self.high_impact_modifier
    }
}

pub fn overlay_for_role(role: SlotRole) -> SlotRoleOverlay {
    match role {
        SlotRole::Standard => SlotRoleOverlay {
            role: SlotRole::Standard,
            execution_modifier: 1.0,
            production_modifier: 1.0,
            defense_modifier: 1.0,
            ball_security_modifier: 1.0,
            discipline_modifier: 1.0,
            high_impact_modifier: 1.0,
        },
        SlotRole::FalseArtrine => SlotRoleOverlay {
            role: SlotRole::FalseArtrine,
            execution_modifier: 1.15,
            production_modifier: 0.85,
            defense_modifier: 1.0,
            ball_security_modifier: 1.20,
            discipline_modifier: 1.0,
            high_impact_modifier: 1.10,
        },
        SlotRole::Launcher => SlotRoleOverlay {
            role: SlotRole::Launcher,
            execution_modifier: 1.25,
            production_modifier: 1.20,
            defense_modifier: 0.70,
            ball_security_modifier: 1.10,
            discipline_modifier: 1.0,
            high_impact_modifier: 1.20,
        },
        SlotRole::Safeguard => SlotRoleOverlay {
            role: SlotRole::Safeguard,
            execution_modifier: 1.0,
            production_modifier: 0.70,
            defense_modifier: 1.30,
            ball_security_modifier: 1.30,
            discipline_modifier: 1.10,
            high_impact_modifier: 0.90,
        },
        SlotRole::Blocker => SlotRoleOverlay {
            role: SlotRole::Blocker,
            execution_modifier: 1.30,
            production_modifier: 0.60,
            defense_modifier: 1.30,
            ball_security_modifier: 0.90,
            discipline_modifier: 1.10,
            high_impact_modifier: 1.0,
        },
        SlotRole::Kicker => SlotRoleOverlay {
            role: SlotRole::Kicker,
            execution_modifier: 1.20,
            production_modifier: 1.30,
            defense_modifier: 0.60,
            ball_security_modifier: 1.0,
            discipline_modifier: 1.0,
            high_impact_modifier: 1.30,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PerformanceProfile {
    position: Position,
    line: PositionLine,
    role: SlotRole,
    weights: PerformanceProfileWeights,
}

impl PerformanceProfile {
    pub fn for_position(position: Position) -> Self {
        Self::for_position_and_role(position, SlotRole::Standard)
    }

    pub fn for_position_and_role(position: Position, role: SlotRole) -> Self {
        let line = position.line();
        let base_pos_weights = weights_for_position(position);
        let overlay = overlay_for_role(role);
        let final_weights = base_pos_weights.apply_overlay(&overlay);

        Self {
            position,
            line,
            role,
            weights: final_weights,
        }
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn line(&self) -> PositionLine {
        self.line
    }

    pub fn role(&self) -> SlotRole {
        self.role
    }

    pub fn weights(&self) -> &PerformanceProfileWeights {
        &self.weights
    }

    pub fn calculate_latent_score(&self, breakdown: &PerformanceBreakdown) -> f64 {
        self.weights.calculate_latent_score(breakdown)
    }
}
