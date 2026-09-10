use crate::attributes::PlayerAttributeTable;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::dynamic_map::DynamicSpatialMap;
use crate::spatial::proximity::calculate_distance;
use crate::weighting::apply_saturation;
use arlo_domain::{AttributeKey, Player};
use arlo_math::units::{Position, MIRIM_TO_METERS};
use arlo_tactics::{PlayerInstructions, TeamInstructions};
use rand::Rng;
use std::collections::HashMap;
use std::f64::consts::PI;
use uuid::Uuid;

pub fn anchor_drift_radius_mirim(positioning: f64) -> f64 {
    let deficit = (20.0 - positioning).max(0.0);
    let raw = deficit * 0.175;
    apply_saturation(raw, 1.5, 0.6)
}

pub fn anchor_drift_radius_mirim_with_structure(positioning: f64, structure: f64) -> f64 {
    let base = anchor_drift_radius_mirim(positioning);
    let multiplier = (1.0 - structure).max(0.0);
    base * multiplier
}

pub fn apply_positioning_drift<R: Rng + ?Sized>(
    anchor: Position,
    positioning: f64,
    rng: &mut R,
) -> Position {
    let radius_mirim = anchor_drift_radius_mirim(positioning);
    if radius_mirim <= 1e-6 {
        return anchor;
    }
    let angle = rng.gen_range(0.0..2.0 * PI);
    let dist_mirim = radius_mirim * rng.gen_range(0.0..1.0_f64).sqrt();
    let dx_meters = dist_mirim * angle.cos() * MIRIM_TO_METERS;
    let dy_meters = dist_mirim * angle.sin() * MIRIM_TO_METERS;
    Position::from_components(
        anchor.raw().0 + dx_meters,
        anchor.raw().1 + dy_meters,
        anchor.raw().2,
    )
}

pub fn apply_positioning_drift_with_structure<R: Rng + ?Sized>(
    anchor: Position,
    positioning: f64,
    structure: f64,
    creative_license: f64,
    rng: &mut R,
) -> Position {
    let base = anchor_drift_radius_mirim(positioning);
    let structure_mult = (1.0 - structure).max(0.0);
    let multiplier = structure_mult.max(creative_license);
    let radius_mirim = base * multiplier;
    if radius_mirim <= 1e-6 {
        return anchor;
    }
    let angle = rng.gen_range(0.0..2.0 * PI);
    let dist_mirim = radius_mirim * rng.gen_range(0.0..1.0_f64).sqrt();
    let dx_meters = dist_mirim * angle.cos() * MIRIM_TO_METERS;
    let dy_meters = dist_mirim * angle.sin() * MIRIM_TO_METERS;
    Position::from_components(
        anchor.raw().0 + dx_meters,
        anchor.raw().1 + dy_meters,
        anchor.raw().2,
    )
}

pub fn get_drifted_defender_position_from_table<R: Rng + ?Sized>(
    defender: &Player,
    table: &PlayerAttributeTable,
    spatial_map: &DynamicSpatialMap,
    rng: &mut R,
) -> Option<Position> {
    let anchor = spatial_map.get_position(&defender.id())?;
    let positioning = extract_attribute_value(table, AttributeKey::Positioning);
    Some(apply_positioning_drift(anchor, positioning, rng))
}

pub fn get_drifted_defender_position<R: Rng + ?Sized>(
    defender: &Player,
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    rng: &mut R,
) -> Option<Position> {
    let table = PlayerAttributeTable::from_player(defender, attribute_keys);
    get_drifted_defender_position_from_table(defender, &table, spatial_map, rng)
}

pub fn get_drifted_attacker_position_from_table<R: Rng + ?Sized>(
    attacker: &Player,
    table: &PlayerAttributeTable,
    spatial_map: &DynamicSpatialMap,
    instructions: &TeamInstructions,
    player_instructions: PlayerInstructions,
    rng: &mut R,
) -> Option<Position> {
    let anchor = spatial_map.get_position(&attacker.id())?;
    let positioning = extract_attribute_value(table, AttributeKey::Positioning);
    let structure = instructions.in_possession().structure().value();
    let creative_license = player_instructions
        .in_possession()
        .creative_license()
        .value();
    Some(apply_positioning_drift_with_structure(
        anchor,
        positioning,
        structure,
        creative_license,
        rng,
    ))
}

pub fn get_drifted_attacker_position<R: Rng + ?Sized>(
    attacker: &Player,
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    instructions: &TeamInstructions,
    player_instructions: PlayerInstructions,
    rng: &mut R,
) -> Option<Position> {
    let table = PlayerAttributeTable::from_player(attacker, attribute_keys);
    get_drifted_attacker_position_from_table(
        attacker,
        &table,
        spatial_map,
        instructions,
        player_instructions,
        rng,
    )
}

pub fn nearest_drifted_opponent_from_tables<'a, R: Rng + ?Sized>(
    reference_pos: Position,
    candidates: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    rng: &mut R,
) -> Option<(&'a Player, Position)> {
    static DEFAULT_TABLE: PlayerAttributeTable = PlayerAttributeTable::new_default();
    candidates
        .iter()
        .filter_map(|&p| {
            let table = attribute_tables.get(&p.id()).unwrap_or(&DEFAULT_TABLE);
            get_drifted_defender_position_from_table(p, table, spatial_map, rng).map(|pos| (p, pos))
        })
        .min_by(|(_, pos_a), (_, pos_b)| {
            let dist_a = calculate_distance(reference_pos, *pos_a).value();
            let dist_b = calculate_distance(reference_pos, *pos_b).value();
            dist_a
                .partial_cmp(&dist_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
}

pub fn nearest_drifted_opponent<'a, R: Rng + ?Sized>(
    reference_pos: Position,
    candidates: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    rng: &mut R,
) -> Option<(&'a Player, Position)> {
    let mut attribute_tables = HashMap::with_capacity(candidates.len());
    for p in candidates {
        attribute_tables.insert(p.id(), PlayerAttributeTable::from_player(p, attribute_keys));
    }
    nearest_drifted_opponent_from_tables(
        reference_pos,
        candidates,
        spatial_map,
        &attribute_tables,
        rng,
    )
}
