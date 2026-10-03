use super::TeamInput;
use crate::error::{EngineError, EngineResult};
use arlo_domain::{Position, SlotRole};
use arlo_tactics::{max_concurrent_count, validate_tactical_lineup, PreparedTacticalPlan};
use std::collections::HashSet;

impl TeamInput {
    pub fn with_prepared_plans(mut self, plans: Vec<PreparedTacticalPlan>) -> EngineResult<Self> {
        if plans.is_empty()
            || plans[0].layout.formation != self.formation
            || plans[0].layout.lineup != self.lineup
            || plans[0].profile_id != self.tactics.id()
        {
            return Err(EngineError::InvalidInput(
                "prepared plans require a matching initial plan".into(),
            ));
        }
        let anchors: HashSet<_> = self
            .lineup
            .assignments()
            .iter()
            .map(|slot| slot.player_id())
            .collect();
        let mut ids = HashSet::new();
        for (index, plan) in plans.iter().enumerate() {
            let layout = &plan.layout;
            let assignments = layout.lineup.assignments();
            if !ids.insert(plan.id)
                || plan.name.is_empty()
                || layout.lineup.team_id() != self.team_id
                || layout.lineup.formation_id() != layout.formation.id()
                || !self
                    .tactical_profiles()
                    .any(|profile| profile.id() == plan.profile_id)
                || assignments
                    .iter()
                    .map(|slot| slot.player_id())
                    .collect::<HashSet<_>>()
                    != anchors
                || assignments
                    .iter()
                    .map(|slot| slot.formation_slot_index())
                    .collect::<HashSet<_>>()
                    .len()
                    != 14
                || assignments.iter().any(|slot| {
                    layout
                        .formation
                        .slots()
                        .get(slot.formation_slot_index())
                        .is_none_or(|position| position.offensive_position() != slot.position())
                })
            {
                return Err(EngineError::InvalidInput(
                    "invalid prepared plan layout or profile".into(),
                ));
            }
            validate_tactical_lineup(&layout.lineup, &layout.formation, &self.roster)
                .map_err(|error| EngineError::InvalidInput(error.to_string()))?;
            if index != 0 {
                for position in [Position::Artrine, Position::Passer, Position::Goalguard] {
                    let initial = self
                        .lineup
                        .assignments()
                        .iter()
                        .filter(|slot| slot.position() == position)
                        .map(|slot| slot.player_id())
                        .collect::<Vec<_>>();
                    let target = assignments
                        .iter()
                        .filter(|slot| slot.position() == position)
                        .map(|slot| slot.player_id())
                        .collect::<Vec<_>>();
                    if initial.len() != 1 || initial != target {
                        return Err(EngineError::InvalidInput(
                            "prepared plans preserve mandatory designations".into(),
                        ));
                    }
                }
                for role in [
                    SlotRole::FalseArtrine,
                    SlotRole::Launcher,
                    SlotRole::Safeguard,
                    SlotRole::Kicker,
                    SlotRole::Blocker,
                ] {
                    if max_concurrent_count(role).is_some_and(|limit| {
                        assignments
                            .iter()
                            .filter(|slot| slot.slot_role() == role)
                            .count()
                            > limit as usize
                    }) {
                        return Err(EngineError::InvalidInput(
                            "prepared plan exceeds a tactical role limit".into(),
                        ));
                    }
                }
            }
        }
        self.prepared_plans = plans;
        Ok(self)
    }

    pub fn prepared_plans(&self) -> &[PreparedTacticalPlan] {
        &self.prepared_plans
    }
}
