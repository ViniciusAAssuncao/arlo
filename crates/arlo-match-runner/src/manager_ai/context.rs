use super::plan_context::{PlanContext, TacticalEffect};
use arlo_domain::{Position, SlotRole};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub(super) struct ManagerSkills {
    pub judging: f64,
    pub adjustments: f64,
    pub knowledge: f64,
    pub adaptability: f64,
    pub offense: f64,
    pub defense: f64,
    pub artro: f64,
    pub load: f64,
    pub composure: f64,
    pub patience: f64,
    pub flexibility: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct PerformanceEvidence {
    pub deviation: f64,
    pub confidence: f64,
    pub seconds_played: f64,
    pub opportunities: f64,
    pub execution: f64,
    pub production: f64,
    pub defense: f64,
    pub security: f64,
    pub discipline: f64,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct SlotContext {
    pub player_id: Uuid,
    pub offensive_position: Position,
    pub defensive_position: Position,
    pub role: SlotRole,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct RoleFit {
    pub quality: f64,
    pub proficiency: f64,
}

#[derive(Debug, Clone)]
pub(super) struct PlayerContext {
    pub id: Uuid,
    pub active: bool,
    pub available: bool,
    pub injured: bool,
    pub starter: bool,
    pub captain: bool,
    pub energy: f64,
    pub morale: f64,
    pub settling: f64,
    pub entered_at: Option<f64>,
    pub exited_at: Option<f64>,
    pub fits: Vec<RoleFit>,
    pub attack: f64,
    pub defense: f64,
    pub control: f64,
    pub security: f64,
    pub discipline: f64,
    pub evidence: PerformanceEvidence,
}

#[derive(Debug, Clone)]
pub(super) struct ManagerDecisionContext {
    pub seed: u64,
    pub sequence: u64,
    pub team_id: Uuid,
    pub manager_id: Uuid,
    pub elapsed: f64,
    pub progress: f64,
    pub deficit: f64,
    pub last_substitution_at: Option<f64>,
    pub last_tactical_switch_at: Option<f64>,
    pub last_realignment_at: Option<f64>,
    pub last_plan_activation_at: Option<f64>,
    pub current_effect: TacticalEffect,
    pub plans: Vec<PlanContext>,
    pub skills: ManagerSkills,
    pub slots: Vec<SlotContext>,
    pub players: Vec<PlayerContext>,
}

impl ManagerDecisionContext {
    pub fn player(&self, id: Uuid) -> Option<&PlayerContext> {
        self.players.iter().find(|player| player.id == id)
    }

    pub fn pressure(&self) -> f64 {
        (self.deficit.abs() / 20.0).clamp(0.0, 1.0) * self.progress
    }
}
