use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::physical::systems::degradation::calculate_effective_player_speed_from_table;
use crate::physical::FatigueState;
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
            let st = fatigue_for(&p.id());
            let table = attribute_tables
                .get(&p.id())
                .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let spd = calculate_effective_player_speed_from_table(p, table, &st);
            (p, fallback_pos, spd)
        })
        .collect()
}

pub fn collect_drifted_defender_candidates_from_tables<'a, F, R>(
    defenders: &[&'a Player],
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fallback_pos: VectorPosition,
    fatigue_for: &F,
    _rng: &mut R,
) -> Vec<(&'a Player, VectorPosition, Speed)>
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    defenders
        .iter()
        .map(|&p| {
            let table = attribute_tables
                .get(&p.id())
                .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
            let st = fatigue_for(&p.id());
            let spd = calculate_effective_player_speed_from_table(p, table, &st);
            (p, fallback_pos, spd)
        })
        .collect()
}

pub fn collect_swept_participant_ids(
    lead_id: Uuid,
    _origin_pos: VectorPosition,
    _carrier_vel: Velocity,
    candidates: &[(&Player, VectorPosition, Speed)],
    _contest_radius: Length,
    _duration: Duration,
) -> SmallVec<[Uuid; 4]> {
    let mut ids = SmallVec::new();
    ids.push(lead_id);
    for (p, _, _) in candidates {
        if !ids.contains(&p.id()) {
            ids.push(p.id());
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
        defense_instructions_index,
        attribute_tables,
        fatigue_for,
        contest_radius,
        window_duration,
        rng,
    )
    .unwrap_or(defenders[0])
}