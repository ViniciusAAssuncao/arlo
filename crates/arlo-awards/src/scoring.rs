use arlo_domain::{
    AwardCandidateEvidence, AwardCriterion, AwardDefinition, AwardNormalization, AwardTieDirection,
};
use std::cmp::Ordering;

pub(crate) fn criterion_value(
    criterion: &AwardCriterion,
    candidate: &AwardCandidateEvidence,
    pool: &[AwardCandidateEvidence],
) -> f64 {
    let Some(value) = candidate
        .metrics
        .iter()
        .find(|metric| metric.key == criterion.key)
        .map(|metric| metric.value)
    else {
        return 0.0;
    };
    let comparable: Vec<f64> = pool
        .iter()
        .filter(|other| same_group(criterion.normalization, candidate, other))
        .filter_map(|other| {
            other
                .metrics
                .iter()
                .find(|metric| metric.key == criterion.key)
        })
        .map(|metric| metric.value)
        .filter(|value| value.is_finite())
        .collect();
    if comparable.len() < 2 {
        return 0.5;
    }
    let lower = comparable.iter().filter(|other| **other < value).count();
    let equal = comparable.iter().filter(|other| **other == value).count();
    (lower as f64 + equal as f64 * 0.5) / comparable.len() as f64
}

pub(crate) fn base_utility(
    definition: &AwardDefinition,
    candidate: &AwardCandidateEvidence,
    pool: &[AwardCandidateEvidence],
) -> f64 {
    definition
        .criteria
        .iter()
        .map(|criterion| criterion.weight * criterion_value(criterion, candidate, pool))
        .sum()
}

fn same_group(
    normalization: AwardNormalization,
    candidate: &AwardCandidateEvidence,
    other: &AwardCandidateEvidence,
) -> bool {
    match normalization {
        AwardNormalization::Global | AwardNormalization::CandidatePool => true,
        AwardNormalization::Position => candidate.position == other.position,
        AwardNormalization::PositionFamily => candidate.position_family == other.position_family,
        AwardNormalization::Competition => candidate.competition_id == other.competition_id,
    }
}

pub(crate) fn compare_tie_breaks(
    definition: &AwardDefinition,
    left: &AwardCandidateEvidence,
    right: &AwardCandidateEvidence,
) -> Ordering {
    for criterion in &definition.tie_breaks {
        let left_value = left
            .metrics
            .iter()
            .find(|metric| metric.key == criterion.metric_key)
            .map(|metric| metric.value)
            .unwrap_or(0.0);
        let right_value = right
            .metrics
            .iter()
            .find(|metric| metric.key == criterion.metric_key)
            .map(|metric| metric.value)
            .unwrap_or(0.0);
        let order = match criterion.direction {
            AwardTieDirection::Descending => right_value.total_cmp(&left_value),
            AwardTieDirection::Ascending => left_value.total_cmp(&right_value),
        };
        if order != Ordering::Equal {
            return order;
        }
    }
    Ordering::Equal
}
