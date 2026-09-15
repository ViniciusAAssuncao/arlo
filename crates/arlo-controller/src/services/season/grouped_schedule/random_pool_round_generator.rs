use crate::services::season::grouped_schedule::random_pool_pairing_constraint_tracker::RandomPoolPairingConstraintTracker;
use crate::services::season::grouped_schedule::random_pool_round_pairer::pair_random_pool_round;
use crate::services::season::round_robin::RoundRobinMatch;
use uuid::Uuid;

pub fn generate_random_pool_rounds(
    team_ids: &[Uuid],
    start_round_index: u32,
    rounds_count: u32,
) -> Vec<RoundRobinMatch> {
    if team_ids.len() < 2 || rounds_count == 0 {
        return Vec::new();
    }

    let matches_per_round = team_ids.len() / 2;
    let mut all_matches = Vec::with_capacity(matches_per_round * rounds_count as usize);
    let mut tracker = RandomPoolPairingConstraintTracker::new();
    let mut rng = rand::thread_rng();

    for r in 0..rounds_count {
        let current_round = start_round_index + r;
        let round_matches =
            pair_random_pool_round(team_ids, current_round, &mut tracker, &mut rng);
        all_matches.extend(round_matches);
    }

    all_matches
}
