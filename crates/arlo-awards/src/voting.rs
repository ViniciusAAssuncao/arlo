use crate::resolution::{CandidateResult, ElectorateResult};
use crate::scoring::criterion_value;
use arlo_domain::{AwardCandidateEvidence, AwardDefinition, AwardVoterGroup};
use rand::Rng;
use rand_chacha::ChaCha8Rng;

pub(crate) fn vote(
    definition: &AwardDefinition,
    pool: &[AwardCandidateEvidence],
    results: &mut [CandidateResult],
    groups: &[AwardVoterGroup],
    ballot_points: &[u32],
    rng: &mut ChaCha8Rng,
) -> Vec<ElectorateResult> {
    let total_weight: f64 = groups.iter().map(|group| group.result_weight).sum();
    groups
        .iter()
        .map(|group| {
            let mut points = vec![0_u64; results.len()];
            let mut first_place = vec![0_u32; results.len()];
            for _ in 0..group.voter_count {
                let mut ranking: Vec<(usize, f64)> = results
                    .iter()
                    .enumerate()
                    .map(|(index, result)| {
                        let preference = group
                            .criterion_preferences
                            .iter()
                            .map(|item| {
                                definition
                                    .criteria
                                    .iter()
                                    .find(|criterion| criterion.key == item.criterion_key)
                                    .map(|criterion| {
                                        item.multiplier
                                            * criterion.weight
                                            * criterion_value(criterion, &pool[index], pool)
                                    })
                                    .unwrap_or(0.0)
                            })
                            .sum::<f64>();
                        let draw = rng.gen_range(f64::MIN_POSITIVE..1.0_f64);
                        let noise = -(-draw.ln()).ln();
                        (index, result.utility + preference + noise)
                    })
                    .collect();
                ranking.sort_by(|a, b| b.1.total_cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
                for (rank, (index, _)) in ranking.iter().take(ballot_points.len()).enumerate() {
                    points[*index] += u64::from(ballot_points[rank]);
                    if rank == 0 {
                        first_place[*index] += 1;
                    }
                }
            }
            let possible = group.voter_count as f64 * ballot_points[0] as f64;
            for (index, result) in results.iter_mut().enumerate() {
                result.vote_points += points[index];
                result.first_place_votes += first_place[index];
                if possible > 0.0 {
                    result.selection_score +=
                        (points[index] as f64 / possible) * group.result_weight / total_weight;
                }
            }
            ElectorateResult {
                group_code: group.code.clone(),
                voter_count: group.voter_count,
                result_weight: group.result_weight,
                subject_ids: results.iter().map(|result| result.subject_id).collect(),
                points,
                first_place,
            }
        })
        .collect()
}
