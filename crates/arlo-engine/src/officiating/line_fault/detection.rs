use crate::attributes::PlayerAttributeTable;
use arlo_domain::pitch::Pitch;
use arlo_domain::sport_constants::FIRST_ZONE_DEPTH_MIRIM;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn identify_last_defender<'a>(
    outfield_defenders: &[&'a Player],
    defense_pos_index: &HashMap<Uuid, DomainPosition>,
) -> Option<&'a Player> {
    if outfield_defenders.is_empty() {
        return None;
    }
    outfield_defenders
        .iter()
        .copied()
        .find(|p| defense_pos_index.get(&p.id()) == Some(&DomainPosition::Centerback))
        .or_else(|| {
            outfield_defenders
                .iter()
                .copied()
                .find(|p| defense_pos_index.get(&p.id()) == Some(&DomainPosition::MiddleZonerback))
        })
        .or_else(|| outfield_defenders.first().copied())
}

pub fn estimate_last_defender_position(
    pitch: &Pitch,
    attacking_positive_x: bool,
) -> VectorPosition {
    let pitch_len_m = pitch.length().value();
    let pitch_width_m = pitch.width().value();
    let last_def_x = if attacking_positive_x {
        (pitch_len_m - (FIRST_ZONE_DEPTH_MIRIM + 8.0) * MIRIM_TO_METERS).max(0.0)
    } else {
        ((FIRST_ZONE_DEPTH_MIRIM + 8.0) * MIRIM_TO_METERS).min(pitch_len_m)
    };
    VectorPosition::from_components(last_def_x, pitch_width_m * 0.5, 0.0)
}

pub fn is_line_fault(
    _receiver: &Player,
    receiver_table: &PlayerAttributeTable,
    receiver_pos: VectorPosition,
    _last_defender: &Player,
    defender_table: &PlayerAttributeTable,
    defender_pos: VectorPosition,
    attacking_positive_x: bool,
) -> (bool, f64) {
    let raw_margin = if attacking_positive_x {
        receiver_pos.raw().0 - defender_pos.raw().0
    } else {
        defender_pos.raw().0 - receiver_pos.raw().0
    };

    let rec_ant = receiver_table.get(AttributeKey::Anticipation);
    let rec_dec = receiver_table.get(AttributeKey::Decisions);
    let def_pos = defender_table.get(AttributeKey::Positioning);
    let def_ant = defender_table.get(AttributeKey::Anticipation);

    let skill_offset = ((def_pos + def_ant) - (rec_ant + rec_dec)) * 0.05;
    let effective_margin = raw_margin + skill_offset;

    if effective_margin > 0.5 {
        (true, effective_margin)
    } else {
        (false, 0.0)
    }
}