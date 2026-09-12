use arlo_domain::ArtrineDecisionKind;
use arlo_stats::DecisionKindStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerArtrineDecisionByKindRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub decision_kind: String,
    pub total: i32,
    pub successful: i32,
    pub failed: i32,
    pub mirins_advanced: f64,
    pub points_generated: i32,
    pub success_rate: f64,
    pub average_mirins_advanced: f64,
    pub average_points_generated: f64,
}

impl MatchPlayerArtrineDecisionByKindRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        decision_kind: impl Into<String>,
        total: u32,
        successful: u32,
        failed: u32,
        mirins_advanced: f64,
        points_generated: u32,
        success_rate: f64,
        average_mirins_advanced: f64,
        average_points_generated: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            decision_kind: decision_kind.into(),
            total: total as i32,
            successful: successful as i32,
            failed: failed as i32,
            mirins_advanced,
            points_generated: points_generated as i32,
            success_rate,
            average_mirins_advanced,
            average_points_generated,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        kind: ArtrineDecisionKind,
        stats: &DecisionKindStats,
    ) -> Self {
        let kind_str = match kind {
            ArtrineDecisionKind::SelfCarry => "SelfCarry",
            ArtrineDecisionKind::ShortPass => "ShortPass",
            ArtrineDecisionKind::LongLaunch => "LongLaunch",
            ArtrineDecisionKind::Cross => "Cross",
            ArtrineDecisionKind::SelfFinish => "SelfFinish",
        };
        Self::new(
            id,
            match_id,
            player_id,
            kind_str,
            stats.total,
            stats.successful,
            stats.failed,
            stats.mirins_advanced,
            stats.points_generated,
            stats.success_rate(),
            stats.average_mirins_advanced(),
            stats.average_points_generated(),
        )
    }
}
