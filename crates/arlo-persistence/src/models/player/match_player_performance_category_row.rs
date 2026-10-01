use arlo_analytics::{PerformanceCategoryContribution, PlayerPerformanceSnapshot};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct MatchPlayerPerformanceCategoryRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub team_id: String,
    pub category: String,
    pub latent_contribution: f64,
    pub observations: i64,
    pub opportunity_weight: f64,
    pub execution_score: f64,
    pub production_score: f64,
    pub defense_score: f64,
    pub ball_security_score: f64,
    pub discipline_score: f64,
    pub high_impact_score: f64,
    pub model_version: i64,
}

impl MatchPlayerPerformanceCategoryRow {
    pub fn from_contribution(
        id: Uuid,
        match_id: Uuid,
        snapshot: &PlayerPerformanceSnapshot,
        contribution: &PerformanceCategoryContribution,
    ) -> Self {
        let breakdown = contribution.breakdown();
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: snapshot.player_id().to_string(),
            team_id: snapshot.team_id().to_string(),
            category: contribution.category().as_str().to_string(),
            latent_contribution: contribution.latent_contribution(),
            observations: i64::from(contribution.observations()),
            opportunity_weight: contribution.opportunity_weight(),
            execution_score: breakdown.execution(),
            production_score: breakdown.production(),
            defense_score: breakdown.defense(),
            ball_security_score: breakdown.ball_security(),
            discipline_score: breakdown.discipline(),
            high_impact_score: breakdown.high_impact(),
            model_version: i64::from(snapshot.model_version().value()),
        }
    }
}
