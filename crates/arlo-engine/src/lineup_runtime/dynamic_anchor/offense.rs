use crate::attributes::PlayerAttributeTable;
use crate::lineup_runtime::dynamic_anchor::AnchorComputationContext;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::positioning_drift::anchor_drift_radius_mirim;
use crate::team_identity::{
    depth_from_bipolar, individual_line_depth_offset, lateral_flank_shift, lateral_spread,
    line_rx_band,
};
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, FormationSlot, Player, Position, PositionLine, SlotRole};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use arlo_tactics::{PlayerInstructions, TeamInstructions};
use rand::Rng;
use std::collections::HashMap;
use std::f64::consts::PI;
use uuid::Uuid;

pub fn resolve_offense_player_attractor(
    pitch: &Pitch,
    player: &Player,
    slot: &FormationSlot,
    scrimmage_x_mirim: f64,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
    role_index: &HashMap<Uuid, SlotRole>,
    ctx: &AnchorComputationContext,
) -> VectorPosition {
    if role_index.get(&player.id()) == Some(&SlotRole::FalseArtrine) {
        crate::team_identity::false_artrine::decoy_attractor(
            pitch,
            player,
            slot,
            scrimmage_x_mirim,
            true,
            attacking_positive_x,
            attribute_keys,
            instructions,
            ctx,
        )
    } else {
        crate::lineup_runtime::dynamic_anchor::calculate_player_dynamic_attractor(
            pitch,
            player,
            slot,
            scrimmage_x_mirim,
            true,
            attacking_positive_x,
            attribute_keys,
            instructions,
            ctx,
        )
    }
}

pub fn calculate_offense_attractor_coordinates_for_position(
    pitch: &Pitch,
    player: &Player,
    target_position: Position,
    _scrimmage_x_m: f64,
    base_x: f64,
    base_y: f64,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
    ctx: &AnchorComputationContext,
) -> (f64, f64) {
    let pitch_length_m = pitch.length().value();
    let pitch_width_m = pitch.width().value();

    let push_distance_m = match target_position.line() {
        PositionLine::DefenseLine => {
            let table = PlayerAttributeTable::from_player(player, attribute_keys);
            let tactical_knowledge =
                extract_attribute_value(&table, AttributeKey::TacticalKnowledge);
            let positioning = extract_attribute_value(&table, AttributeKey::Positioning);
            let anticipation = extract_attribute_value(&table, AttributeKey::Anticipation);

            let push_factor =
                ((tactical_knowledge * 0.45 + positioning * 0.35 + anticipation * 0.2) / 20.0)
                    .clamp(0.1, 1.0);

            let target_depth = depth_from_bipolar(
                instructions.in_possession().mentality().value(),
                pitch_length_m,
                attacking_positive_x,
            );

            let distance_to_target = if attacking_positive_x {
                target_depth - base_x
            } else {
                base_x - target_depth
            };

            distance_to_target.max(0.0) * push_factor
        }
        _ => 0.0,
    };

    let x_shift = if attacking_positive_x {
        push_distance_m
    } else {
        -push_distance_m
    };

    let y_pos = match target_position {
        Position::WingOffense | Position::TightWing | Position::WideEnd | Position::Corridor => {
            let width_val = instructions.in_possession().width().value();
            let flank_bias_val = instructions.in_possession().flank_bias().value();
            let spread_y = lateral_spread(width_val, base_y, pitch_width_m);
            lateral_flank_shift(flank_bias_val, spread_y, pitch_width_m)
        }
        _ => base_y,
    };

    let positioning_bias = ctx
        .player_instructions_index
        .get(&player.id())
        .copied()
        .unwrap_or_default()
        .in_possession()
        .positioning_bias();
    let (band_min, band_max) = line_rx_band(target_position.line());
    let bias_offset = individual_line_depth_offset(
        positioning_bias.value(),
        band_min,
        band_max,
        pitch_length_m,
        attacking_positive_x,
    );

    (base_x + x_shift + bias_offset, y_pos)
}

pub fn calculate_offense_attractor_coordinates(
    pitch: &Pitch,
    player: &Player,
    slot: &FormationSlot,
    scrimmage_x_m: f64,
    base_x: f64,
    base_y: f64,
    attacking_positive_x: bool,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
    ctx: &AnchorComputationContext,
) -> (f64, f64) {
    calculate_offense_attractor_coordinates_for_position(
        pitch,
        player,
        slot.offensive_position(),
        scrimmage_x_m,
        base_x,
        base_y,
        attacking_positive_x,
        attribute_keys,
        instructions,
        ctx,
    )
}

pub fn calculate_offense_drift_radius_mirim(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
    player_instructions: PlayerInstructions,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let positioning = extract_attribute_value(&table, AttributeKey::Positioning);
    let structure_val = instructions.in_possession().structure().value();
    let base_radius = anchor_drift_radius_mirim(positioning);
    let team_structure_multiplier = (1.0 - structure_val).max(0.0);
    let creative_license_val = player_instructions
        .in_possession()
        .creative_license()
        .value();
    let effective_multiplier = team_structure_multiplier.max(creative_license_val);
    base_radius * effective_multiplier
}

pub fn apply_offensive_positioning_drift<R: Rng + ?Sized>(
    anchor: VectorPosition,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
    player_instructions: PlayerInstructions,
    rng: &mut R,
) -> VectorPosition {
    let radius_mirim = calculate_offense_drift_radius_mirim(
        player,
        attribute_keys,
        instructions,
        player_instructions,
    );
    if radius_mirim <= 1e-6 {
        return anchor;
    }
    let angle = rng.gen_range(0.0..2.0 * PI);
    let dist_mirim = radius_mirim * rng.gen_range(0.0..1.0_f64).sqrt();
    let dx_meters = dist_mirim * angle.cos() * MIRIM_TO_METERS;
    let dy_meters = dist_mirim * angle.sin() * MIRIM_TO_METERS;
    VectorPosition::from_components(
        anchor.raw().0 + dx_meters,
        anchor.raw().1 + dy_meters,
        anchor.raw().2,
    )
}