use crate::attributes::PlayerAttributeTable;
use crate::physical::FatigueState;
use crate::spatial::decision_vector::extract_attribute_value;
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, Player, SlotRole};
use arlo_math::geometry::{
    compute_point_team_control, compute_team_control_fraction, VoronoiRegion, VoronoiSite,
};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn build_player_voronoi_site_at_from_table<F>(
    player: &Player,
    table: &PlayerAttributeTable,
    position: VectorPosition,
    fatigue_for: &F,
    team_id: u8,
) -> VoronoiSite
where
    F: Fn(&Uuid) -> FatigueState,
{
    let fatigue = fatigue_for(&player.id());

    let stamina = extract_attribute_value(table, AttributeKey::Stamina);
    let natural_fitness = extract_attribute_value(table, AttributeKey::NaturalFitness);
    let mult = crate::physical::models::energy_model::fatigue_multiplier(
        &fatigue,
        stamina,
        natural_fitness,
    );

    let speed = crate::physical::systems::degradation::calculate_effective_player_speed_from_table(
        player,
        table,
        &crate::physical::PhysicalState::with_energy(mult),
    )
    .value();
    let ant = extract_attribute_value(table, AttributeKey::Anticipation);
    let reaction_time = ((20.0 - ant) * 0.015).max(0.05);

    VoronoiSite::new(position.raw().0, position.raw().1, speed, reaction_time, team_id)
}

pub fn build_player_voronoi_site_from_table<F>(
    player: &Player,
    table: &PlayerAttributeTable,
    spatial_map: &DynamicSpatialMap,
    fatigue_for: &F,
    team_id: u8,
) -> VoronoiSite
where
    F: Fn(&Uuid) -> FatigueState,
{
    let pos = spatial_map
        .get_position(&player.id())
        .unwrap_or_else(VectorPosition::zero);
    build_player_voronoi_site_at_from_table(player, table, pos, fatigue_for, team_id)
}

pub fn build_team_voronoi_sites_from_tables<F>(
    players: &[&Player],
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    spatial_map: &DynamicSpatialMap,
    fatigue_for: &F,
    team_id: u8,
) -> Vec<VoronoiSite>
where
    F: Fn(&Uuid) -> FatigueState,
{
    static DEFAULT_TABLE: PlayerAttributeTable = PlayerAttributeTable::new_default();
    players
        .iter()
        .map(|p| {
            let table = attribute_tables.get(&p.id()).unwrap_or(&DEFAULT_TABLE);
            build_player_voronoi_site_from_table(*p, table, spatial_map, fatigue_for, team_id)
        })
        .collect()
}

pub fn calculate_kinematic_pitch_control_from_tables<F>(
    attackers: &[&Player],
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fatigue_for: &F,
    region: &VoronoiRegion,
) -> f64
where
    F: Fn(&Uuid) -> FatigueState,
{
    let att_sites = build_team_voronoi_sites_from_tables(
        attackers,
        attribute_tables,
        spatial_map,
        fatigue_for,
        0,
    );
    let def_sites = build_team_voronoi_sites_from_tables(
        defenders,
        attribute_tables,
        spatial_map,
        fatigue_for,
        1,
    );
    compute_team_control_fraction(&att_sites, &def_sites, region, 5, 5)
}

pub fn calculate_artro_advance_pitch_control_from_tables<F>(
    artrine: &Player,
    helpers: &[&Player],
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fatigue_for: &F,
    start_pos: VectorPosition,
    target_artro_pos: VectorPosition,
    pitch: &Pitch,
    offense_role_index: &HashMap<Uuid, SlotRole>,
) -> f64
where
    F: Fn(&Uuid) -> FatigueState,
{
    let mut attackers = Vec::with_capacity(helpers.len() + 1);
    attackers.push(artrine);
    attackers.extend_from_slice(helpers);

    let x0 = start_pos.raw().0.min(target_artro_pos.raw().0);
    let x1 = start_pos.raw().0.max(target_artro_pos.raw().0);
    let margin = 5.0 * MIRIM_TO_METERS;
    let y_center = (start_pos.raw().1 + target_artro_pos.raw().1) / 2.0;
    let half_width = 12.0 * MIRIM_TO_METERS;

    let x_min = (x0 - margin).max(0.0);
    let x_max = (x1 + margin).min(pitch.length().value());
    let y_min = (y_center - half_width).max(0.0);
    let y_max = (y_center + half_width).min(pitch.width().value());

    let region = VoronoiRegion::new(x_min, x_max, y_min, y_max);
    let mut att_sites = build_team_voronoi_sites_from_tables(
        &attackers,
        attribute_tables,
        spatial_map,
        fatigue_for,
        0,
    );

    static DEFAULT_TABLE: PlayerAttributeTable = PlayerAttributeTable::new_default();
    for &helper in helpers {
        if offense_role_index.get(&helper.id()) == Some(&SlotRole::FalseArtrine) {
            let table = attribute_tables.get(&helper.id()).unwrap_or(&DEFAULT_TABLE);
            let phantom = crate::team_identity::false_artrine::phantom_voronoi_site_from_table(
                helper,
                table,
                spatial_map,
                fatigue_for,
                0,
            );
            att_sites.push(phantom);
        }
    }

    let def_sites = build_team_voronoi_sites_from_tables(
        defenders,
        attribute_tables,
        spatial_map,
        fatigue_for,
        1,
    );
    compute_team_control_fraction(&att_sites, &def_sites, &region, 5, 5)
}

pub fn calculate_point_pitch_control_players_from_tables<F>(
    attackers: &[&Player],
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_tables: &HashMap<Uuid, PlayerAttributeTable>,
    fatigue_for: &F,
    point: VectorPosition,
) -> f64
where
    F: Fn(&Uuid) -> FatigueState,
{
    let att_sites = build_team_voronoi_sites_from_tables(
        attackers,
        attribute_tables,
        spatial_map,
        fatigue_for,
        0,
    );
    let def_sites = build_team_voronoi_sites_from_tables(
        defenders,
        attribute_tables,
        spatial_map,
        fatigue_for,
        1,
    );
    compute_point_team_control(&att_sites, &def_sites, point.raw().0, point.raw().1)
}

pub fn query_control_along_vector(
    attackers: &[VoronoiSite],
    defenders: &[VoronoiSite],
    start_pos: VectorPosition,
    direction: VectorPosition,
    step_size_meters: f64,
    max_distance_meters: f64,
) -> Vec<(f64, f64)> {
    let dir_raw = direction.raw();
    let dir_mag = (dir_raw.0 * dir_raw.0 + dir_raw.1 * dir_raw.1).sqrt();
    if dir_mag <= 1e-9 || max_distance_meters <= 0.0 || step_size_meters <= 0.0 {
        let c0 = compute_point_team_control(
            attackers,
            defenders,
            start_pos.raw().0,
            start_pos.raw().1,
        );
        return vec![(0.0, c0)];
    }

    let norm_dx = dir_raw.0 / dir_mag;
    let norm_dy = dir_raw.1 / dir_mag;
    let steps = (max_distance_meters / step_size_meters).ceil() as usize;
    let mut samples = Vec::with_capacity(steps + 1);

    for i in 0..=steps {
        let dist = ((i as f64) * step_size_meters).min(max_distance_meters);
        let qx = start_pos.raw().0 + norm_dx * dist;
        let qy = start_pos.raw().1 + norm_dy * dist;
        let control = compute_point_team_control(attackers, defenders, qx, qy);
        samples.push((dist, control));
        if dist >= max_distance_meters {
            break;
        }
    }

    samples
}

pub fn control_at_point(
    attackers: &[VoronoiSite],
    defenders: &[VoronoiSite],
    x: f64,
    y: f64,
) -> f64 {
    compute_point_team_control(attackers, defenders, x, y)
}