use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PerformanceBreakdown {
    pub execution_score: f64,
    pub production_score: f64,
    pub defense_score: f64,
    pub ball_security_score: f64,
    pub discipline_score: f64,
    pub high_impact_score: f64,
}

impl PerformanceBreakdown {
    pub fn new(
        execution_score: f64,
        production_score: f64,
        defense_score: f64,
        ball_security_score: f64,
        discipline_score: f64,
        high_impact_score: f64,
    ) -> Self {
        Self {
            execution_score: Self::clamp_score(execution_score),
            production_score: Self::clamp_score(production_score),
            defense_score: Self::clamp_score(defense_score),
            ball_security_score: Self::clamp_score(ball_security_score),
            discipline_score: Self::clamp_score(discipline_score),
            high_impact_score: Self::clamp_score(high_impact_score),
        }
    }

    pub fn neutral() -> Self {
        Self {
            execution_score: 6.0,
            production_score: 5.5,
            defense_score: 6.0,
            ball_security_score: 7.0,
            discipline_score: 8.0,
            high_impact_score: 5.0,
        }
    }

    pub fn clamp_score(value: f64) -> f64 {
        if value.is_nan() {
            6.0
        } else {
            (value.clamp(0.0, 10.0) * 100.0).round() / 100.0
        }
    }

    pub fn average_score(&self) -> f64 {
        let sum = self.execution_score
            + self.production_score
            + self.defense_score
            + self.ball_security_score
            + self.discipline_score
            + self.high_impact_score;
        (sum / 6.0 * 100.0).round() / 100.0
    }
}