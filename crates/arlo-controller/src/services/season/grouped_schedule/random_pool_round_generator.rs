use crate::services::season::round_robin::RoundRobinMatch;
use rand::seq::SliceRandom;
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
    let mut rng = rand::thread_rng();

    for r in 0..rounds_count {
        let current_round = start_round_index + r;
        let mut shuffled = team_ids.to_vec();
        shuffled.shuffle(&mut rng);

        for chunk in shuffled.chunks_exact(2) {
            all_matches.push(RoundRobinMatch::new(
                current_round,
                chunk[0],
                chunk[1],
            ));
        }
    }

    all_matches
}
