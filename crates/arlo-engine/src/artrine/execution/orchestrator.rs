use crate::artrine::execution::carry::execute_carry;
use crate::artrine::execution::context::ActionExecutionContext;
use crate::artrine::execution::distribution::execute_distribution;
use crate::artrine::execution::finish::{execute_cross_pipeline, execute_self_finish};
use crate::artrine::execution::outcome::ArtrineExecutionOutcome;
use crate::error::{EngineError, EngineResult};
use crate::physical::FatigueState;
use crate::resolution::DuelContext;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::{
    ArtrineDecisionKind, AttributeKey, Player, Position as DomainPosition, SlotRole,
};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::{PlayerInstructions, RouteAssignment, TeamInstructions};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn find_goalguard<'a>(defenders: &[&'a Player]) -> EngineResult<&'a Player> {
    defenders
        .iter()
        .copied()
        .find(|p| {
            p.positions()
                .iter()
                .any(|pos| pos.position() == DomainPosition::Goalguard && pos.proficiency() > 0)
        })
        .or_else(|| {
            defenders
                .iter()
                .copied()
                .find(|p| p.positions().iter().any(|pos| pos.position() == DomainPosition::Goalguard))
        })
        .or_else(|| defenders.first().copied())
        .ok_or_else(|| EngineError::MissingRequiredPosition("Goalguard".to_string()))
}

pub fn execute_artrine_decision<F, R>(
    decision: ArtrineDecisionKind,
    artrine: &Player,
    offense_lineup_players: &[&Player],
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    offense_role_index: &HashMap<Uuid, SlotRole>,
    offense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    offense_instructions: &TeamInstructions,
    defense_lineup_players: &[&Player],
    defense_position_index: &HashMap<Uuid, DomainPosition>,
    defense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    pitch: &Pitch,
    spatial_map: &mut DynamicSpatialMap,
    start_pos: VectorPosition,
    attacking_positive_x: bool,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    drives_in_series: u32,
    accumulated_advance_mirim: f64,
    is_last_down: bool,
    is_bonus_phase: bool,
    context: &DuelContext,
    fatigue_for: &F,
    defense_pressing_multiplier: f64,
    offense_tempo_value: f64,
    openness_by_player: &HashMap<Uuid, f64>,
    offense_route_index: &HashMap<Uuid, RouteAssignment>,
    rng: &mut R,
) -> EngineResult<ArtrineExecutionOutcome>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let offense_helpers: Vec<&Player> = offense_lineup_players
        .iter()
        .copied()
        .filter(|p| p.id() != artrine.id())
        .collect();

    let goalguard = find_goalguard(defense_lineup_players)?;

    let ctx = ActionExecutionContext {
        pitch,
        attribute_keys,
        offense_team_id,
        defense_team_id,
        attacking_positive_x,
        drives_in_series,
        accumulated_advance_mirim,
        is_last_down,
        is_bonus_phase,
        defense_pressing_multiplier,
        offense_tempo_value,
        duel_context: context,
        fatigue_for,
        offense_helpers: &offense_helpers,
        offense_position_index,
        offense_role_index,
        offense_instructions_index,
        offense_instructions,
        defenders: defense_lineup_players,
        defense_position_index,
        defense_instructions_index,
        goalguard,
        openness_by_player,
        offense_route_index,
    };

    let outcome = match decision {
        ArtrineDecisionKind::SelfCarry => execute_carry(&ctx, artrine, spatial_map, start_pos, rng),
        ArtrineDecisionKind::ShortPass | ArtrineDecisionKind::LongLaunch => {
            execute_distribution(&ctx, decision, artrine, spatial_map, start_pos, rng)
        }
        ArtrineDecisionKind::Cross => {
            execute_cross_pipeline(&ctx, artrine, spatial_map, start_pos, rng)
        }
        ArtrineDecisionKind::SelfFinish => {
            execute_self_finish(&ctx, artrine, spatial_map, start_pos, rng)
        }
    };

    Ok(outcome)
}