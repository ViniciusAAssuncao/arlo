use crate::attributes::PlayerAttributeTable;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
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

pub fn is_line_fault(
    _receiver: &Player,
    receiver_table: &PlayerAttributeTable,
    receiver_x_mirim: f64,
    _last_defender: &Player,
    defender_table: &PlayerAttributeTable,
    defender_x_mirim: f64,
    attacking_positive_x: bool,
) -> (bool, f64) {
    let raw_margin = if attacking_positive_x {
        receiver_x_mirim - defender_x_mirim
    } else {
        defender_x_mirim - receiver_x_mirim
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