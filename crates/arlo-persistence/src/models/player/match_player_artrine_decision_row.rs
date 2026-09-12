use arlo_stats::PlayerArtrineDecisionStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerArtrineDecisionRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub total_decisions: i32,
    pub total_successful_decisions: i32,
    pub total_failed_decisions: i32,
    pub total_mirins_advanced: f64,
    pub total_points_generated: i32,
    pub goal_points_generated: i32,
    pub field_points_generated: i32,
    pub field_goals_generated: i32,
    pub success_rate: f64,
    pub average_mirins_per_decision: f64,
    pub average_points_per_decision: f64,
}

impl MatchPlayerArtrineDecisionRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        total_decisions: u32,
        total_successful_decisions: u32,
        total_failed_decisions: u32,
        total_mirins_advanced: f64,
        total_points_generated: u32,
        goal_points_generated: u32,
        field_points_generated: u32,
        field_goals_generated: u32,
        success_rate: f64,
        average_mirins_per_decision: f64,
        average_points_per_decision: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            total_decisions: total_decisions as i32,
            total_successful_decisions: total_successful_decisions as i32,
            total_failed_decisions: total_failed_decisions as i32,
            total_mirins_advanced,
            total_points_generated: total_points_generated as i32,
            goal_points_generated: goal_points_generated as i32,
            field_points_generated: field_points_generated as i32,
            field_goals_generated: field_goals_generated as i32,
            success_rate,
            average_mirins_per_decision,
            average_points_per_decision,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerArtrineDecisionStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.total_decisions,
            stats.total_successful_decisions,
            stats.total_failed_decisions,
            stats.total_mirins_advanced,
            stats.total_points_generated,
            stats.goal_points_generated,
            stats.field_points_generated,
            stats.field_goals_generated,
            stats.success_rate(),
            stats.average_mirins_per_decision(),
            stats.average_points_per_decision(),
        )
    }
}
