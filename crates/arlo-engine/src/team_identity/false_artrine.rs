use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::calculate_effective_player_speed_from_table;
use crate::physical::FatigueState;
use arlo_domain::{AttributeKey, Player};
use arlo_math::geometry::VoronoiSite;
use arlo_math::units::Position as VectorPosition;
use uuid::Uuid;

pub fn phantom_voronoi_site_from_table<F>(
    false_artrine: &Player,
    table: &PlayerAttributeTable,
    fallback_pos: VectorPosition,
    fatigue_for: &F,
    team_id: u8,
) -> VoronoiSite
where
    F: Fn(&Uuid) -> FatigueState,
{
    let fatigue = fatigue_for(&false_artrine.id());
    let speed = calculate_effective_player_speed_from_table(false_artrine, table, &fatigue).value();
    let bluff = table.get(AttributeKey::FalseArtrineBluff);
    let reaction_time = ((20.0 - bluff) * 0.015).max(0.05);

    VoronoiSite::new(fallback_pos.raw().0, fallback_pos.raw().1, speed, reaction_time, team_id)
}