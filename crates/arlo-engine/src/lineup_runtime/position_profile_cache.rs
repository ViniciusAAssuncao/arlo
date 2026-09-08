use crate::current_ability::profiles::get_profile_for_position;
use crate::current_ability::weights::PositionWeightProfile;
use crate::lineup_runtime::position_similarity::calculate_profile_similarity;
use arlo_domain::Position;
use std::collections::HashMap;
use std::sync::OnceLock;

static PROFILE_CACHE: OnceLock<HashMap<Position, PositionWeightProfile>> = OnceLock::new();
static SIMILARITY_CACHE: OnceLock<HashMap<(Position, Position), f64>> = OnceLock::new();

fn init_profile_cache() -> HashMap<Position, PositionWeightProfile> {
    let mut map = HashMap::with_capacity(21);
    for pos in Position::all() {
        map.insert(pos, get_profile_for_position(pos));
    }
    map
}

fn init_similarity_cache(
    profiles: &HashMap<Position, PositionWeightProfile>,
) -> HashMap<(Position, Position), f64> {
    let mut map = HashMap::with_capacity(441);
    let all_positions = Position::all();
    for &pos_a in &all_positions {
        for &pos_b in &all_positions {
            let sim = if pos_a == pos_b {
                1.0
            } else {
                let prof_a = &profiles[&pos_a];
                let prof_b = &profiles[&pos_b];
                calculate_profile_similarity(prof_a, prof_b)
            };
            map.insert((pos_a, pos_b), sim);
        }
    }
    map
}

pub fn get_position_profiles() -> &'static HashMap<Position, PositionWeightProfile> {
    PROFILE_CACHE.get_or_init(init_profile_cache)
}

pub fn get_position_profile(position: Position) -> &'static PositionWeightProfile {
    &get_position_profiles()[&position]
}

pub fn get_similarity_map() -> &'static HashMap<(Position, Position), f64> {
    SIMILARITY_CACHE.get_or_init(|| {
        let profiles = get_position_profiles();
        init_similarity_cache(profiles)
    })
}

pub fn get_position_similarity(a: Position, b: Position) -> f64 {
    *get_similarity_map().get(&(a, b)).unwrap_or(&0.0)
}