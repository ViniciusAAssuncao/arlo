use arlo_analytics::PlayerPerformanceSnapshot;
use arlo_domain::{Position, SlotRole};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, FromRow)]
pub struct MatchPlayerPerformanceRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub team_id: String,
    pub performance_rating: f64,
    pub outcome_adjustment: f64,
    pub final_rating: f64,
    pub confidence: f64,
    pub seconds_played: f64,
    pub effective_opportunities: i64,
    pub effective_opportunity_weight: f64,
    pub offensive_latent: f64,
    pub defensive_latent: f64,
    pub raw_latent: f64,
    pub offensive_position: String,
    pub defensive_position: String,
    pub slot_role: String,
    pub execution_score: f64,
    pub production_score: f64,
    pub defense_score: f64,
    pub ball_security_score: f64,
    pub discipline_score: f64,
    pub high_impact_score: f64,
    pub model_version: i64,
}

impl MatchPlayerPerformanceRow {
    pub fn from_snapshot(
        id: Uuid,
        match_id: Uuid,
        snapshot: &PlayerPerformanceSnapshot,
    ) -> Self {
        let breakdown = snapshot.breakdown();
        let diagnostics = snapshot.diagnostics();
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: snapshot.player_id().to_string(),
            team_id: snapshot.team_id().to_string(),
            performance_rating: snapshot.performance_rating().value(),
            outcome_adjustment: snapshot.outcome_adjustment(),
            final_rating: snapshot.final_rating().value(),
            confidence: snapshot.confidence().value(),
            seconds_played: snapshot.seconds_played(),
            effective_opportunities: i64::from(snapshot.effective_opportunities()),
            effective_opportunity_weight: diagnostics.effective_opportunity_weight(),
            offensive_latent: diagnostics.offensive_latent(),
            defensive_latent: diagnostics.defensive_latent(),
            raw_latent: diagnostics.raw_latent(),
            offensive_position: position_code(snapshot.offensive_position()).to_string(),
            defensive_position: position_code(snapshot.defensive_position()).to_string(),
            slot_role: slot_role_code(snapshot.slot_role()).to_string(),
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

fn position_code(position: Position) -> &'static str {
    match position {
        Position::CenterOffense => "CenterOffense",
        Position::WingOffense => "WingOffense",
        Position::Midcenter => "Midcenter",
        Position::TightWing => "TightWing",
        Position::CenterTight => "CenterTight",
        Position::Corridor => "Corridor",
        Position::Artrine => "Artrine",
        Position::Passer => "Passer",
        Position::PassRusher => "PassRusher",
        Position::WideEnd => "WideEnd",
        Position::RunningEnd => "RunningEnd",
        Position::Lineback => "Lineback",
        Position::Fullback => "Fullback",
        Position::Centerback => "Centerback",
        Position::DefensiveEnd => "DefensiveEnd",
        Position::Rougieback => "Rougieback",
        Position::DefensiveBlocker => "DefensiveBlocker",
        Position::WideBlocker => "WideBlocker",
        Position::OutsideZonerback => "OutsideZonerback",
        Position::MiddleZonerback => "MiddleZonerback",
        Position::Goalguard => "Goalguard",
    }
}

fn slot_role_code(role: SlotRole) -> &'static str {
    match role {
        SlotRole::Standard => "Standard",
        SlotRole::FalseArtrine => "FalseArtrine",
        SlotRole::Launcher => "Launcher",
        SlotRole::Safeguard => "Safeguard",
        SlotRole::Blocker => "Blocker",
        SlotRole::Kicker => "Kicker",
    }
}
