use crate::services::season::grouped_schedule::random_pool_pairing_constraint_tracker::RandomPoolPairingConstraintTracker;
use crate::services::season::round_robin::RoundRobinMatch;
use rand::seq::SliceRandom;
use rand::Rng;
use uuid::Uuid;

const MAX_PAIRING_ATTEMPTS: usize = 50;

pub fn pair_random_pool_round<R: Rng + ?Sized>(
    team_ids: &[Uuid],
    round_index: u32,
    tracker: &mut RandomPoolPairingConstraintTracker,
    rng: &mut R,
) -> Vec<RoundRobinMatch> {
    if team_ids.len() < 2 {
        return Vec::new();
    }

    let active_teams = if team_ids.len() % 2 != 0 {
        let min_byes = team_ids
            .iter()
            .map(|id| tracker.bye_count(id))
            .min()
            .unwrap_or(0);

        let bye_candidates: Vec<Uuid> = team_ids
            .iter()
            .filter(|id| tracker.bye_count(id) == min_byes)
            .copied()
            .collect();

        let bye_team = *bye_candidates.choose(rng).unwrap();
        tracker.record_bye(bye_team);

        team_ids
            .iter()
            .filter(|&&id| id != bye_team)
            .copied()
            .collect::<Vec<Uuid>>()
    } else {
        team_ids.to_vec()
    };

    for _ in 0..MAX_PAIRING_ATTEMPTS {
        let mut shuffled = active_teams.clone();
        shuffled.shuffle(rng);

        if let Some(pairs) = try_pair_without_repetition(&shuffled, tracker) {
            for &(a, b) in &pairs {
                tracker.record_pair(a, b, round_index);
            }
            return pairs
                .into_iter()
                .map(|(home, away)| RoundRobinMatch::new(round_index, home, away))
                .collect();
        }
    }

    let mut best_pairs: Option<Vec<(Uuid, Uuid)>> = None;
    let mut best_penalty = u64::MAX;

    for _ in 0..MAX_PAIRING_ATTEMPTS {
        let mut shuffled = active_teams.clone();
        shuffled.shuffle(rng);

        let (pairs, penalty) = pair_with_fallback(&shuffled, tracker, rng);
        if penalty < best_penalty {
            best_penalty = penalty;
            best_pairs = Some(pairs);
        }
    }

    if let Some(pairs) = best_pairs {
        for &(a, b) in &pairs {
            tracker.record_pair(a, b, round_index);
        }
        pairs
            .into_iter()
            .map(|(home, away)| RoundRobinMatch::new(round_index, home, away))
            .collect()
    } else {
        Vec::new()
    }
}

fn try_pair_without_repetition(
    teams: &[Uuid],
    tracker: &RandomPoolPairingConstraintTracker,
) -> Option<Vec<(Uuid, Uuid)>> {
    let mut available = teams.to_vec();
    let mut pairs = Vec::with_capacity(teams.len() / 2);

    while !available.is_empty() {
        let t1 = available.remove(0);
        let partner_pos = available
            .iter()
            .position(|&t2| !tracker.is_pair_used(t1, t2))?;
        let t2 = available.remove(partner_pos);
        pairs.push((t1, t2));
    }

    Some(pairs)
}

fn pair_with_fallback<R: Rng + ?Sized>(
    teams: &[Uuid],
    tracker: &RandomPoolPairingConstraintTracker,
    rng: &mut R,
) -> (Vec<(Uuid, Uuid)>, u64) {
    let mut available = teams.to_vec();
    let mut pairs = Vec::with_capacity(teams.len() / 2);
    let mut total_penalty = 0u64;

    while !available.is_empty() {
        let t1 = available.remove(0);

        let mut candidate_indices: Vec<usize> = (0..available.len()).collect();
        candidate_indices.shuffle(rng);

        let best_idx = candidate_indices
            .into_iter()
            .min_by_key(|&idx| {
                let t2 = available[idx];
                calc_pair_penalty(t1, t2, tracker)
            })
            .unwrap();

        let t2 = available.remove(best_idx);
        let penalty = calc_pair_penalty(t1, t2, tracker);
        total_penalty = total_penalty.saturating_add(penalty);
        pairs.push((t1, t2));
    }

    (pairs, total_penalty)
}

fn calc_pair_penalty(a: Uuid, b: Uuid, tracker: &RandomPoolPairingConstraintTracker) -> u64 {
    let usage_count = tracker.pair_usage_count(a, b) as u64;
    let last_round = tracker.pair_last_round(a, b).unwrap_or(0) as u64;
    (usage_count << 32) | last_round
}