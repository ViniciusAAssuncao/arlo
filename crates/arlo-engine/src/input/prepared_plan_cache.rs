use crate::error::EngineResult;
use crate::{MatchInput, TeamInput, TeamState};
use arlo_domain::{Position, SlotRole};
use arlo_tactics::TacticalLayout;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq)]
struct SlotSignature {
    anchor: Uuid,
    player: Uuid,
    index: usize,
    position: Position,
    offense: Position,
    defense: Position,
    role: SlotRole,
    active: bool,
}

#[derive(Debug, Default)]
struct TeamPreviews {
    signature: Vec<SlotSignature>,
    layouts: HashMap<Uuid, TacticalLayout>,
}

#[derive(Debug, Default)]
pub(crate) struct PreparedPlanCache {
    teams: Mutex<HashMap<Uuid, TeamPreviews>>,
}

impl PreparedPlanCache {
    pub(crate) fn get_or_build(
        &self,
        team: &TeamInput,
        current: &TeamState,
        plan_id: Uuid,
        build: impl FnOnce() -> EngineResult<TacticalLayout>,
    ) -> EngineResult<TacticalLayout> {
        let signature: Vec<_> = current
            .lineup(team)
            .assignments()
            .iter()
            .map(|assignment| {
                let player = current.slot_player_id(assignment.player_id());
                let slot = &current.formation(team).slots()[assignment.formation_slot_index()];
                SlotSignature {
                    anchor: assignment.player_id(),
                    player,
                    index: assignment.formation_slot_index(),
                    position: assignment.position(),
                    offense: slot.offensive_position(),
                    defense: slot.defensive_position(),
                    role: assignment.slot_role(),
                    active: current.active_player_ids().contains(&player),
                }
            })
            .collect();
        let mut teams = self.teams.lock().unwrap_or_else(|error| error.into_inner());
        let cache = teams.entry(team.team_id()).or_default();
        if cache.signature != signature {
            cache.signature = signature;
            cache.layouts.clear();
        }
        if let Some(layout) = cache.layouts.get(&plan_id) {
            return Ok(layout.clone());
        }
        let layout = build()?;
        cache.layouts.insert(plan_id, layout.clone());
        Ok(layout)
    }
}

impl MatchInput {
    pub(crate) fn preview_cached_plan(
        &self,
        team: &TeamInput,
        current: &TeamState,
        plan_id: Uuid,
        build: impl FnOnce() -> EngineResult<TacticalLayout>,
    ) -> EngineResult<TacticalLayout> {
        self.prepared_plan_cache
            .get_or_build(team, current, plan_id, build)
    }
}
