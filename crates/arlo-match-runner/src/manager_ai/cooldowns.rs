use super::context::ManagerDecisionContext;
use arlo_engine::TeamState;

#[derive(Clone, Copy)]
pub(super) struct DecisionReadiness {
    pub substitution: bool,
    pub realignment: bool,
    pub plan: bool,
}

impl DecisionReadiness {
    pub fn from_state(state: &TeamState, elapsed: f64) -> Self {
        Self::new(
            elapsed,
            state.last_voluntary_substitution_at(),
            state.last_tactical_realignment_at(),
            state.last_plan_activation_at(),
            state.last_tactical_switch_at(),
        )
    }

    pub fn from_context(context: &ManagerDecisionContext) -> Self {
        Self::new(
            context.elapsed,
            context.last_substitution_at,
            context.last_realignment_at,
            context.last_plan_activation_at,
            context.last_tactical_switch_at,
        )
    }

    fn new(
        elapsed: f64,
        substitution: Option<f64>,
        realignment: Option<f64>,
        plan: Option<f64>,
        switch: Option<f64>,
    ) -> Self {
        let blocked =
            |last: Option<f64>, delay: f64| last.is_some_and(|last| elapsed - last < delay);
        Self {
            substitution: !blocked(plan, 900.0)
                && !blocked(substitution, 900.0)
                && !blocked(realignment, 600.0),
            realignment: !blocked(plan, 900.0)
                && !blocked(realignment, 1200.0)
                && !blocked(substitution, 600.0)
                && !blocked(switch, 600.0),
            plan: !blocked(plan, 1800.0)
                && !blocked(substitution, 600.0)
                && !blocked(realignment, 900.0)
                && !blocked(switch, 900.0),
        }
    }
}
