use super::TeamState;
use crate::TeamInput;
use arlo_domain::Formation;
use arlo_tactics::{TacticalLayout, TacticalLineup};
use uuid::Uuid;

impl TeamState {
    pub fn formation<'a>(&'a self, input: &'a TeamInput) -> &'a Formation {
        self.active_layout
            .as_ref()
            .map_or(input.formation(), |layout| &layout.formation)
    }

    pub fn lineup<'a>(&'a self, input: &'a TeamInput) -> &'a TacticalLineup {
        self.active_layout
            .as_ref()
            .map_or(input.lineup(), |layout| &layout.lineup)
    }

    pub fn tactical_layout(&self, input: &TeamInput) -> TacticalLayout {
        TacticalLayout {
            formation: self.formation(input).clone(),
            lineup: self.lineup(input).clone(),
        }
    }

    pub fn active_plan_id(&self) -> Option<Uuid> {
        self.active_plan_id
    }
    pub fn last_plan_activation_at(&self) -> Option<f64> {
        self.last_plan_activation_at
    }

    pub(crate) fn activate_prepared_plan(
        &mut self,
        id: Uuid,
        profile_id: Uuid,
        layout: TacticalLayout,
        elapsed: f64,
    ) {
        self.active_layout = Some(layout);
        self.active_plan_id = Some(id);
        self.active_tactical_profile_id = profile_id;
        self.last_plan_activation_at = Some(elapsed);
    }
}
