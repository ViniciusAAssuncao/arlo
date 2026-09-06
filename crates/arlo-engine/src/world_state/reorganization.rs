
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::run_spatial_tick_loop;
use crate::tactics::dynamic_anchor::compute_dynamic_anchors;
use crate::world_state::match_state::MatchState;
use arlo_math::units::Duration;

pub fn derive_and_apply_reorganization(
    state: &mut MatchState,
    scrimmage_x_mirim: f64,
) -> Duration {
    let pitch = *state.pitch();
    let home_lineup = state.home_lineup().clone();
    let away_lineup = state.away_lineup().clone();
    let attribute_keys = state.attribute_keys().clone();

    let is_home_offense = state.possession().role().is_offense(state.home_team_id());

    let home_targets = compute_dynamic_anchors(
        &pitch,
        &home_lineup,
        scrimmage_x_mirim,
        is_home_offense,
        true,
        &attribute_keys,
    );
    let away_targets = compute_dynamic_anchors(
        &pitch,
        &away_lineup,
        scrimmage_x_mirim,
        !is_home_offense,
        false,
        &attribute_keys,
    );

    let mut movers = Vec::with_capacity(home_lineup.len() + away_lineup.len());
    for assignment in home_lineup.assignments() {
        if let Some(&target) = home_targets.get(&assignment.player().id()) {
            movers.push((assignment.player(), target));
        }
    }
    for assignment in away_lineup.assignments() {
        if let Some(&target) = away_targets.get(&assignment.player().id()) {
            movers.push((assignment.player(), target));
        }
    }

    let tick_result =
        run_spatial_tick_loop(state.spatial_map_mut(), &movers, &attribute_keys);

    for (player_id, traj) in tick_result.trajectories() {
        let dist_mirim: f64 = traj
            .segments()
            .iter()
            .map(|(a, b)| calculate_distance_mirim(*a, *b))
            .sum();
        state.record_distance(*player_id, dist_mirim);
    }

    Duration::new(tick_result.elapsed_seconds())
}