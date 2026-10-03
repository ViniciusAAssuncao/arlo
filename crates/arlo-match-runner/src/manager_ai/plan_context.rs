use super::context::SlotContext;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct TacticalEffect {
    pub attack: f64,
    pub defense: f64,
    pub control: f64,
    pub security: f64,
    pub load: f64,
    pub discipline: f64,
}

#[derive(Debug, Clone)]
pub(super) struct PlannedAssignment {
    pub destination: SlotContext,
    pub current_index: usize,
    pub quality: f64,
    pub proficiency: f64,
}

#[derive(Debug, Clone)]
pub(super) struct PlanContext {
    pub id: Uuid,
    pub effect: TacticalEffect,
    pub assignments: Vec<PlannedAssignment>,
    pub changed_roles: usize,
    pub formation_changed: bool,
}
