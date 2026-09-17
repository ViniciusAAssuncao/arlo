use crate::attributes::PlayerAttributeTable;
use crate::physical::FatigueState;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Position as VectorPosition, Velocity};
use arlo_tactics::{MarkingAssignment, PlayerInstructions};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_lead_defender_with_marking_from_tables<'a, F, R>(
    carrier_id: Uuid,
    offense_pos_index: &HashMap<Uuid, DomainPosition>,
    _target_pos: VectorPosition,
    _target_vel: Velocity,
    defenders: &[&'a Player],
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    _attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    _fatigue_for: &F,
    _base_contest_radius: Length,
    _max_duration: Option<Duration>,
    _rng: &mut R,
) -> Option<&'a Player>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    if let Some(&carrier_position) = offense_pos_index.get(&carrier_id) {
        for &defender in defenders {
            if let Some(instructions) = instructions_index.get(&defender.id()) {
                if let Some(MarkingAssignment::Man(target)) =
                    instructions.out_of_possession().marking()
                {
                    if target == carrier_position {
                        return Some(defender);
                    }
                }
            }
        }
    }

    defenders.first().copied()
}

pub fn resolve_lead_defender_with_marking<'a, F, R>(
    carrier_id: Uuid,
    offense_pos_index: &HashMap<Uuid, DomainPosition>,
    target_pos: VectorPosition,
    target_vel: Velocity,
    defenders: &[&'a Player],
    instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    base_contest_radius: Length,
    max_duration: Option<Duration>,
    rng: &mut R,
) -> Option<&'a Player>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let mut attribute_tables = HashMap::with_capacity(defenders.len());
    for d in defenders {
        attribute_tables.insert(d.id(), PlayerAttributeTable::from_player(d, attribute_keys));
    }
    resolve_lead_defender_with_marking_from_tables(
        carrier_id,
        offense_pos_index,
        target_pos,
        target_vel,
        defenders,
        instructions_index,
        &attribute_tables,
        fatigue_for,
        base_contest_radius,
        max_duration,
        rng,
    )
}