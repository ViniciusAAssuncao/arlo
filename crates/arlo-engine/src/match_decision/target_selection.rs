use crate::attributes::DEFAULT_PLAYER_ATTRIBUTE_TABLE;
use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::calculate_fit_for_position;
use crate::physical::systems::degradation::{extract_effective_attribute_value, DegradationContext};
use crate::physical::PhysicalState;
use arlo_domain::{AttributeKey, Pitch, Player, Position, SlotRole};
use arlo_math::stats::sample_categorical;
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReceptionRole {
    Finisher,
    OpenPlayReceiver,
    ContinuationReceiver,
}

pub fn player_base_reception_weight(
    table: &PlayerAttributeTable,
    role: ReceptionRole,
    state: Option<&PhysicalState>,
) -> f64 {
    let default_state = PhysicalState::initial();
    let effective_state = state.unwrap_or(&default_state);
    let deg_ctx = DegradationContext::new(effective_state);

    match role {
        ReceptionRole::OpenPlayReceiver => {
            let hands = extract_effective_attribute_value(
                table,
                AttributeKey::HandsReception,
                &deg_ctx,
            );
            let ant = extract_effective_attribute_value(
                table,
                AttributeKey::Anticipation,
                &deg_ctx,
            );
            let pos = extract_effective_attribute_value(
                table,
                AttributeKey::Positioning,
                &deg_ctx,
            );
            (hands * 0.45 + ant * 0.35 + pos * 0.20).max(0.1)
        }
        ReceptionRole::ContinuationReceiver => {
            let pos = extract_effective_attribute_value(
                table,
                AttributeKey::Positioning,
                &deg_ctx,
            );
            let ant = extract_effective_attribute_value(
                table,
                AttributeKey::Anticipation,
                &deg_ctx,
            );
            let accel = extract_effective_attribute_value(
                table,
                AttributeKey::Acceleration,
                &deg_ctx,
            );
            (pos * 0.40 + ant * 0.40 + accel * 0.20).max(0.1)
        }
        ReceptionRole::Finisher => {
            let finishing =
                extract_effective_attribute_value(table, AttributeKey::Finishing, &deg_ctx);
            let technique =
                extract_effective_attribute_value(table, AttributeKey::Technique, &deg_ctx);
            let composure =
                extract_effective_attribute_value(table, AttributeKey::Composure, &deg_ctx);
            (finishing * 0.50 + technique * 0.30 + composure * 0.20).max(0.1)
        }
    }
}

pub fn calculate_player_target_weight(
    player: &Player,
    table: &PlayerAttributeTable,
    _pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    role_index: Option<&HashMap<Uuid, SlotRole>>,
    _attacking_positive_x: bool,
    role: ReceptionRole,
    openness_by_player: &HashMap<Uuid, f64>,
    state: Option<&PhysicalState>,
) -> f64 {
    let base_weight = player_base_reception_weight(table, role, state);
    let assigned_pos = position_index
        .get(&player.id())
        .copied()
        .unwrap_or_else(|| {
            player
                .positions()
                .first()
                .map(|pp| pp.position())
                .unwrap_or(Position::CenterOffense)
        });

    let proximity_factor = match assigned_pos {
        Position::CenterOffense => 1.8,
        Position::WingOffense | Position::WideEnd => 1.5,
        Position::Corridor | Position::RunningEnd => 1.3,
        Position::TightWing | Position::Midcenter => 1.1,
        Position::Goalguard => 0.5,
        _ => 0.8,
    };

    let fit_mult = calculate_fit_for_position(player, assigned_pos)
        .efficiency_multiplier()
        .max(0.1);
    let priority_mult = 1.0
        + instructions_index
            .get(&player.id())
            .copied()
            .unwrap_or_default()
            .in_possession()
            .involvement_priority()
            .value();
    let openness = openness_by_player
        .get(&player.id())
        .copied()
        .unwrap_or(1.0)
        .max(0.05);

    let role_mult = match role_index.and_then(|r| r.get(&player.id())) {
        Some(SlotRole::Kicker) if role == ReceptionRole::Finisher => 1.5,
        Some(SlotRole::Launcher) if role == ReceptionRole::OpenPlayReceiver => 1.4,
        Some(SlotRole::FalseArtrine) => 1.1,
        _ => 1.0,
    };

    (base_weight * proximity_factor * fit_mult * priority_mult * openness * role_mult).max(0.05)
}

pub fn select_target<F, R>(
    candidates: &[&Player],
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    role_index: Option<&HashMap<Uuid, SlotRole>>,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    attacking_positive_x: bool,
    role: ReceptionRole,
    openness_by_player: &HashMap<Uuid, f64>,
    fatigue_for: Option<&F>,
    rng: &mut R,
) -> Option<Uuid>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    if candidates.is_empty() {
        return None;
    }
    if candidates.len() == 1 {
        return Some(candidates[0].id());
    }

    let default_state = PhysicalState::initial();

    let weights: SmallVec<[f64; 16]> = candidates
        .iter()
        .map(|p| {
            let state = match fatigue_for {
                Some(lookup) => lookup(&p.id()),
                None => default_state,
            };
            let table = attribute_tables
                .get(&p.id())
                .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            calculate_player_target_weight(
                p,
                table,
                pitch,
                position_index,
                instructions_index,
                role_index,
                attacking_positive_x,
                role,
                openness_by_player,
                Some(&state),
            )
        })
        .collect();

    let index = sample_categorical(&weights, rng).unwrap_or(0);
    Some(candidates[index].id())
}

pub fn select_finisher<F, R>(
    candidates: &[&Player],
    role_index: Option<&HashMap<Uuid, SlotRole>>,
    pitch: &Pitch,
    position_index: &HashMap<Uuid, Position>,
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    attacking_positive_x: bool,
    openness_by_player: &HashMap<Uuid, f64>,
    fatigue_for: Option<&F>,
    rng: &mut R,
) -> Option<Uuid>
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    select_target(
        candidates,
        pitch,
        position_index,
        instructions_index,
        role_index,
        attribute_tables,
        attacking_positive_x,
        ReceptionRole::Finisher,
        openness_by_player,
        fatigue_for,
        rng,
    )
}
