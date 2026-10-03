use crate::error::{EngineError, EngineResult};
use crate::resolution::context::validate_match_state;
use crate::{MatchInput, MatchPhase, MatchState, StepResult};
use arlo_domain::AttributeKey;
use arlo_events::{MatchEvent, TacticalAssignment, TacticalPlanActivated};
use arlo_manager_control::PreparedPlanIntent;
use arlo_tactics::{adapt_layout, static_role_fit, TacticalLayout};
use uuid::Uuid;

pub fn preview_prepared_plan(
    input: &MatchInput,
    state: &MatchState,
    team_id: Uuid,
    plan_id: Uuid,
) -> EngineResult<TacticalLayout> {
    let team = if team_id == input.home().team_id() {
        input.home()
    } else if team_id == input.away().team_id() {
        input.away()
    } else {
        return Err(EngineError::InvalidInput(
            "unknown prepared plan team".into(),
        ));
    };
    let current = state.team_state(team_id)?;
    let plan = team
        .prepared_plans()
        .iter()
        .find(|plan| plan.id == plan_id)
        .ok_or_else(|| EngineError::InvalidInput("unknown prepared plan".into()))?;
    let profile = team
        .tactical_profiles()
        .find(|profile| profile.id() == plan.profile_id)
        .ok_or_else(|| EngineError::InvalidInput("unknown prepared plan profile".into()))?;
    input.preview_cached_plan(team, current, plan_id, || {
        let layout = current.tactical_layout(team);
        adapt_layout(
            team.lineup(),
            &layout,
            &plan.layout,
            |anchor| {
                current
                    .active_player_ids()
                    .contains(&current.slot_player_id(anchor))
            },
            |anchor, index| {
                let Some(player) = team
                    .roster()
                    .iter()
                    .find(|player| player.id() == current.slot_player_id(anchor))
                else {
                    return -1.0;
                };
                let slot = &plan.layout.formation.slots()[index];
                let role = plan
                    .layout
                    .lineup
                    .assignment_for_slot(index)
                    .map_or(slot.role(), |slot| slot.slot_role());
                let values = input.player_attributes(player.id());
                let attribute = |key: AttributeKey| {
                    values
                        .and_then(|values| values[key.index()])
                        .unwrap_or(10.0)
                };
                (static_role_fit(
                    player,
                    slot.offensive_position(),
                    role,
                    profile.instructions(),
                    &attribute,
                ) + static_role_fit(
                    player,
                    slot.defensive_position(),
                    role,
                    profile.instructions(),
                    &attribute,
                )) * 0.5
            },
        )
        .ok_or_else(|| {
            EngineError::InvalidInput(
                "prepared plan cannot preserve the current slot occupants".into(),
            )
        })
    })
}

pub fn resolve_prepared_plan_segment(
    input: &MatchInput,
    state: &mut MatchState,
    team_id: Uuid,
    intent: &PreparedPlanIntent,
) -> EngineResult<StepResult> {
    validate_match_state(input, state)?;
    if state.phase() != MatchPhase::Stopped
        || state.clock().seconds_in_period() >= state.clock().period_limit_seconds()
    {
        return Err(EngineError::InvalidTransition(
            "prepared plan requires a stoppage before the next play".into(),
        ));
    }
    let current = state.team_state(team_id)?;
    if current.active_plan_id() == Some(intent.plan_id()) {
        return Err(EngineError::InvalidInput(
            "prepared plan is already active".into(),
        ));
    }
    let layout = preview_prepared_plan(input, state, team_id, intent.plan_id())?;
    let team = if team_id == input.home().team_id() {
        input.home()
    } else {
        input.away()
    };
    let plan = team
        .prepared_plans()
        .iter()
        .find(|plan| plan.id == intent.plan_id())
        .expect("validated plan");
    let assignments: Vec<_> = layout
        .lineup
        .assignments()
        .iter()
        .map(|assignment| {
            let slot = &layout.formation.slots()[assignment.formation_slot_index()];
            TacticalAssignment {
                player_id: current.slot_player_id(assignment.player_id()),
                formation_slot_index: assignment.formation_slot_index(),
                offensive_position: slot.offensive_position(),
                defensive_position: slot.defensive_position(),
                slot_role: assignment.slot_role(),
            }
        })
        .collect();
    let mut next = state.clone();
    next.activate_prepared_plan(team_id, plan.id, plan.profile_id, layout)?;
    let event = next.emit(MatchEvent::TacticalPlanActivated(TacticalPlanActivated {
        team_id,
        plan_id: plan.id,
        plan_name: plan.name.clone(),
        formation_id: plan.layout.formation.id(),
        profile_id: plan.profile_id,
        assignments,
    }))?;
    *state = next;
    Ok(StepResult::resolved(vec![event]))
}
