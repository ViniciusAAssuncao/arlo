use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::lineup_runtime::dynamic_anchor::AnchorComputationContext;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::team_identity::depth_from_bipolar;
use crate::team_identity::marking::dynamic_shifting::calculate_carrier_defensive_shift_from_table;
use crate::team_identity::marking::resolve_man_marking_target_position;
use crate::team_identity::BlockMarkingRole;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::{
    BLOCK_MARKING_BITE_TRACKING_BOOST, BLOCK_MARKING_COVER_DISCIPLINE_BOOST,
};
use arlo_domain::{AttributeKey, FormationSlot, Player, PositionLine};
use arlo_math::units::Position as VectorPosition;
use arlo_tactics::{MarkingAssignment, TeamInstructions};
use std::collections::HashMap;
use uuid::Uuid;

pub fn calculate_defense_attractor_coordinates_from_table(
    pitch: &Pitch,
    player: &Player,
    table: &PlayerAttributeTable,
    slot: &FormationSlot,
    _scrimmage_x_m: f64,
    base_x: f64,
    base_y: f64,
    attacking_positive_x: bool,
    instructions: &TeamInstructions,
    marking: Option<MarkingAssignment>,
    ctx: &AnchorComputationContext,
) -> (f64, f64) {
    if let Some(MarkingAssignment::Man(target)) = marking {
        if let (Some(opposing_lineup), Some(spatial_map)) = (ctx.opposing_lineup, ctx.spatial_map) {
            let opposing_players: Vec<&Player> = opposing_lineup
                .assignments()
                .iter()
                .map(|a| a.player())
                .collect();
            let opposing_pos_index = opposing_lineup.offensive_position_index();
            let defender_pos = VectorPosition::from_components(base_x, base_y, 0.0);
            if let Some(man_pos) = resolve_man_marking_target_position(
                target,
                &opposing_players,
                &opposing_pos_index,
                spatial_map,
                defender_pos,
            ) {
                return (man_pos.raw().0, man_pos.raw().1);
            }
        }
    }

    let block_role = ctx
        .block_marking_roles
        .and_then(|m| m.get(&player.id()).copied());

    if block_role == Some(BlockMarkingRole::Biter) {
        if let Some(ref_pos) = ctx.press_reference_pos {
            let x_pos = base_x + (ref_pos.raw().0 - base_x) * BLOCK_MARKING_BITE_TRACKING_BOOST;
            let y_pos = base_y + (ref_pos.raw().1 - base_y) * BLOCK_MARKING_BITE_TRACKING_BOOST;
            return (x_pos, y_pos);
        }
    }

    let pitch_length_m = pitch.length().value();
    let pitch_width_m = pitch.width().value();
    let target_position = slot.defensive_position();

    let work_rate = extract_attribute_value(table, AttributeKey::WorkRate);
    let tactical_knowledge = extract_attribute_value(table, AttributeKey::TacticalKnowledge);
    let determination = extract_attribute_value(table, AttributeKey::Determination);

    let tracking_factor = ((work_rate * 0.5 + tactical_knowledge * 0.35 + determination * 0.15)
        / 20.0)
        .clamp(0.1, 1.0);

    let target_bipolar = match target_position.line() {
        PositionLine::DefenseLine => Some(
            instructions
                .out_of_possession()
                .defensive_line_height()
                .value(),
        ),
        PositionLine::OffensiveLine => {
            Some(instructions.out_of_possession().engagement_line().value())
        }
        PositionLine::BackLine => {
            let dl = instructions
                .out_of_possession()
                .defensive_line_height()
                .value();
            let el = instructions.out_of_possession().engagement_line().value();
            Some((dl + el) * 0.5)
        }
        _ => None,
    };

    let x_pos = match target_bipolar {
        Some(bipolar_val) => {
            let target_depth =
                depth_from_bipolar(bipolar_val, pitch_length_m, attacking_positive_x);
            base_x + (target_depth - base_x) * tracking_factor
        }
        None => base_x,
    };

    let center_y = pitch_width_m * 0.5;
    let mut pinch_factor =
        (1.0 - instructions.out_of_possession().compactness().value()).clamp(0.0, 1.0);
    if block_role == Some(BlockMarkingRole::Coverer) {
        pinch_factor =
            (pinch_factor * (1.0 + BLOCK_MARKING_COVER_DISCIPLINE_BOOST)).clamp(0.0, 1.0);
    }
    let y_pos = base_y + (center_y - base_y) * pinch_factor;

    if let Some(ref_pos) = ctx.press_reference_pos {
        let shifted = calculate_carrier_defensive_shift_from_table(
            player,
            table,
            VectorPosition::from_components(x_pos, y_pos, 0.0),
            ref_pos,
            1.0,
            target_position,
            pitch,
            attacking_positive_x,
        );
        return (shifted.raw().0, shifted.raw().1);
    }

    (x_pos, y_pos)
}

pub fn calculate_defense_attractor_coordinates(
    pitch: &Pitch,
    player: &Player,
    slot: &FormationSlot,
    scrimmage_x_m: f64,
    base_x: f64,
    base_y: f64,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
    marking: Option<MarkingAssignment>,
    ctx: &AnchorComputationContext,
) -> (f64, f64) {
    let table = ctx
        .attribute_tables
        .and_then(|m| m.get(&player.id()))
        .cloned()
        .unwrap_or_else(|| {
            if attribute_keys.is_empty() {
                DEFAULT_PLAYER_ATTRIBUTE_TABLE
            } else {
                PlayerAttributeTable::from_player(player, attribute_keys)
            }
        });

    calculate_defense_attractor_coordinates_from_table(
        pitch,
        player,
        &table,
        slot,
        scrimmage_x_m,
        base_x,
        base_y,
        attacking_positive_x,
        instructions,
        marking,
        ctx,
    )
}
