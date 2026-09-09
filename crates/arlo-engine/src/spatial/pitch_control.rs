use crate::physical::{compute_player_fatigue_multiplier, FatigueState};
use crate::spatial::decision_vector::{calculate_player_speed, extract_attribute_value};
use crate::spatial::DynamicSpatialMap;
use crate::team_identity::false_artrine::phantom_voronoi_site;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, Player, SlotRole};
use arlo_math::geometry::{
    compute_point_team_control, compute_team_control_fraction, VoronoiRegion, VoronoiSite,
};
use arlo_math::units::{Position as VectorPosition, MIRIM_TO_METERS};
use std::collections::HashMap;
use uuid::Uuid;

pub fn build_player_voronoi_site_at<F>(
    player: &Player,
    position: VectorPosition,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    team_id: u8,
) -> VoronoiSite
where
    F: Fn(&Uuid) -> FatigueState,
{
    let fatigue = fatigue_for(&player.id());
    let mult = compute_player_fatigue_multiplier(player, &fatigue, attribute_keys);
    let speed = calculate_player_speed(player, attribute_keys, mult).value();
    let ant = extract_attribute_value(player, attribute_keys, AttributeKey::Anticipation);
    let reaction_time = ((20.0 - ant) * 0.015).max(0.05);

    VoronoiSite::new(
        position.raw().0,
        position.raw().1,
        speed,
        reaction_time,
        team_id,
    )
}

pub fn build_player_voronoi_site<F>(
    player: &Player,
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    team_id: u8,
) -> VoronoiSite
where
    F: Fn(&Uuid) -> FatigueState,
{
    let pos = spatial_map
        .get_position(&player.id())
        .unwrap_or_else(VectorPosition::zero);
    build_player_voronoi_site_at(player, pos, attribute_keys, fatigue_for, team_id)
}

pub fn build_team_voronoi_sites<F>(
    players: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    team_id: u8,
) -> Vec<VoronoiSite>
where
    F: Fn(&Uuid) -> FatigueState,
{
    players
        .iter()
        .map(|p| build_player_voronoi_site(p, spatial_map, attribute_keys, fatigue_for, team_id))
        .collect()
}

pub fn calculate_kinematic_pitch_control<F>(
    attackers: &[&Player],
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    region: &VoronoiRegion,
) -> f64
where
    F: Fn(&Uuid) -> FatigueState,
{
    let att_sites =
        build_team_voronoi_sites(attackers, spatial_map, attribute_keys, fatigue_for, 0);
    let def_sites =
        build_team_voronoi_sites(defenders, spatial_map, attribute_keys, fatigue_for, 1);
    compute_team_control_fraction(&att_sites, &def_sites, region, 5, 5)
}

pub fn calculate_artro_advance_pitch_control<F>(
    artrine: &Player,
    helpers: &[&Player],
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
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
    let mut att_sites =
        build_team_voronoi_sites(&attackers, spatial_map, attribute_keys, fatigue_for, 0);

    for &helper in helpers {
        if offense_role_index.get(&helper.id()) == Some(&SlotRole::FalseArtrine) {
            let phantom = phantom_voronoi_site(helper, spatial_map, attribute_keys, fatigue_for, 0);
            att_sites.push(phantom);
        }
    }

    let def_sites =
        build_team_voronoi_sites(defenders, spatial_map, attribute_keys, fatigue_for, 1);
    compute_team_control_fraction(&att_sites, &def_sites, &region, 5, 5)
}

pub fn calculate_point_pitch_control_players<F>(
    attackers: &[&Player],
    defenders: &[&Player],
    spatial_map: &DynamicSpatialMap,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    fatigue_for: &F,
    point: VectorPosition,
) -> f64
where
    F: Fn(&Uuid) -> FatigueState,
{
    let att_sites =
        build_team_voronoi_sites(attackers, spatial_map, attribute_keys, fatigue_for, 0);
    let def_sites =
        build_team_voronoi_sites(defenders, spatial_map, attribute_keys, fatigue_for, 1);
    compute_point_team_control(&att_sites, &def_sites, point.raw().0, point.raw().1)
}
