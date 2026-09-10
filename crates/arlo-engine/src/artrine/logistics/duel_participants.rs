use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::physical::systems::degradation::calculate_effective_player_speed_from_table;
use crate::physical::FatigueState;
use crate::spatial::positioning_drift::get_drifted_defender_position_from_table;
use crate::spatial::proximity::filter_active_duelists_swept;
use crate::spatial::DynamicSpatialMap;
use crate::team_identity::marking::resolve_lead_defender_with_marking_from_tables;
use arlo_domain::{Player, Position as DomainPosition};
use arlo_math::units::{Duration, Length, Position as VectorPosition, Speed, Velocity};
use arlo_tactics::PlayerInstructions;
use rand::Rng;
use smallvec::SmallVec;
use std::collections::HashMap;
use uuid::Uuid;

pub fn collect_helper_candidates_from_tables<'a, F>(
    helpers: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fallback_pos: VectorPosition,
    fatigue_for: &F,
) -> Vec<(&'a Player, VectorPosition, Speed)>
where
    F: Fn(&Uuid) -> FatigueState,
{
    helpers
        .iter()
        .map(|&p| {
            let pos = spatial_map.get_position(&p.id()).unwrap_or(fallback_pos);
            let st = fatigue_for(&p.id());
            let table = attribute_tables.get(&p.id()).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let spd = calculate_effective_player_speed_from_table(p, table, &st);
            (p, pos, spd)
        })
        .collect()
}

pub fn collect_drifted_defender_candidates_from_tables<'a, F, R>(
    defenders: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fallback_pos: VectorPosition,
    fatigue_for: &F,
    rng: &mut R,
) -> Vec<(&'a Player, VectorPosition, Speed)>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    defenders
        .iter()
        .map(|&p| {
            let table = attribute_tables.get(&p.id()).unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let pos = get_drifted_defender_position_from_table(p, table, spatial_map, rng)
                .or_else(|| spatial_map.get_position(&p.id()))
                .unwrap_or(fallback_pos);
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed_from_table(p, table, &st);
            (p, pos, spd)
        })
        .collect()
}

pub fn collect_swept_participant_ids(
    lead_id: Uuid,
    origin_pos: VectorPosition,
    carrier_vel: Velocity,
    candidates: &[(&Player, VectorPosition, Speed)],
    contest_radius: Length,
    duration: Duration,
) -> SmallVec<[Uuid; 4]> {
    let mut ids = SmallVec::new();
    ids.push(lead_id);
    for id in filter_active_duelists_swept(
        origin_pos,
        carrier_vel,
        candidates,
        contest_radius,
        duration,
    ) {
        if !ids.contains(&id) {
            ids.push(id);
        }
    }
    ids
}

pub fn resolve_primary_lead_defender_from_tables<'a, F, R>(
    target_id: Uuid,
    offense_position_index: &HashMap<Uuid, DomainPosition>,
    reference_pos: VectorPosition,
    velocity: Velocity,
    defenders: &[&'a Player],
    spatial_map: &DynamicSpatialMap,
    defense_instructions_index: &HashMap<Uuid, PlayerInstructions>,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fatigue_for: &F,
    contest_radius: Length,
    window_duration: Option<Duration>,
    rng: &mut R,
) -> &'a Player
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    resolve_lead_defender_with_marking_from_tables(
        target_id,
        offense_position_index,
        reference_pos,
        velocity,
        defenders,
        spatial_map,
        defense_instructions_index,
        attribute_tables,
        fatigue_for,
        contest_radius,
        window_duration,
        rng,
    )
    .unwrap_or(defenders[0])
}
