use crate::physical::FatigueState;
use crate::spatial::interception::identify_kinematic_lead_defender_with_drift;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::{AttributeKey, Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Position as VectorPosition, Velocity};
use arlo_tactics::{MarkingAssignment, PlayerInstructions};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_lead_defender_with_marking<'a, F, R>(
    carrier_id: Uuid,
    offense_pos_index: &HashMap<Uuid, DomainPosition>,
    target_pos: VectorPosition,
    target_vel: Velocity,
    defenders: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
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

    identify_kinematic_lead_defender_with_drift(
        target_pos,
        target_vel,
        defenders,
        spatial_map,
        instructions_index,
        attribute_keys,
        fatigue_for,
        base_contest_radius,
        max_duration,
        rng,
    )
}
