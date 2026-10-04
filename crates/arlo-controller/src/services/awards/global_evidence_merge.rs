use arlo_domain::{award_position_family, AwardCandidateEvidence, AwardMetric, AwardPositionUsage};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

struct CandidateAccumulator {
    candidate: AwardCandidateEvidence,
    representative_matches: u32,
    competitions: BTreeSet<Uuid>,
    metrics: BTreeMap<String, f64>,
    position_usage: BTreeMap<String, (f64, Option<u32>)>,
}

pub(super) fn merge_player_evidence(
    seasons: impl IntoIterator<Item = Vec<AwardCandidateEvidence>>,
) -> Vec<AwardCandidateEvidence> {
    let mut by_player = BTreeMap::<Uuid, CandidateAccumulator>::new();
    for season in seasons {
        for candidate in season {
            let entry = by_player.entry(candidate.subject_id).or_insert_with(|| {
                let mut initial = candidate.clone();
                initial.matches_played = 0;
                initial.positions.clear();
                CandidateAccumulator {
                    representative_matches: 0,
                    competitions: BTreeSet::new(),
                    metrics: BTreeMap::new(),
                    position_usage: BTreeMap::new(),
                    candidate: initial,
                }
            });
            if candidate.matches_played > entry.representative_matches {
                entry.candidate.position = candidate.position.clone();
                entry.candidate.position_family = candidate.position_family.clone();
                entry.candidate.team_id = candidate.team_id;
                entry.candidate.competition_id = candidate.competition_id;
                entry.representative_matches = candidate.matches_played;
            }
            entry.candidate.age_years = entry.candidate.age_years.max(candidate.age_years);
            entry.candidate.matches_played += candidate.matches_played;
            entry.candidate.positions.extend(candidate.positions);
            entry.competitions.extend(candidate.competition_ids);
            for usage in candidate.position_usage {
                let total = entry.position_usage.entry(usage.position_code).or_default();
                total.0 += usage.seconds_played;
                total.1 = total.1.max(usage.proficiency);
            }
            for metric in candidate.metrics {
                let value = if metric.key.starts_with("average_") {
                    metric.value * f64::from(candidate.matches_played)
                } else {
                    metric.value
                };
                *entry.metrics.entry(metric.key).or_default() += value;
            }
        }
    }
    by_player
        .into_values()
        .map(|mut entry| {
            let matches = f64::from(entry.candidate.matches_played.max(1));
            entry.candidate.positions.sort();
            entry.candidate.positions.dedup();
            entry.candidate.competition_ids = entry.competitions.into_iter().collect();
            entry.candidate.position_usage = entry
                .position_usage
                .into_iter()
                .map(
                    |(position_code, (seconds_played, proficiency))| AwardPositionUsage {
                        position_code,
                        seconds_played,
                        proficiency,
                    },
                )
                .collect();
            if !entry.candidate.position_usage.is_empty() {
                entry.candidate.position = entry
                    .candidate
                    .position_usage
                    .iter()
                    .max_by(|left, right| {
                        left.seconds_played
                            .total_cmp(&right.seconds_played)
                            .then_with(|| left.proficiency.cmp(&right.proficiency))
                            .then_with(|| right.position_code.cmp(&left.position_code))
                    })
                    .map(|item| item.position_code.clone());
                entry.candidate.position_family = entry
                    .candidate
                    .position
                    .as_deref()
                    .and_then(award_position_family)
                    .map(str::to_string);
                entry.candidate.positions = entry
                    .candidate
                    .position_usage
                    .iter()
                    .map(|item| item.position_code.clone())
                    .collect();
            }
            entry.candidate.metrics = entry
                .metrics
                .into_iter()
                .map(|(key, mut value)| {
                    if key.starts_with("average_") {
                        value /= matches;
                    }
                    if key == "matches_played" {
                        value = f64::from(entry.candidate.matches_played);
                    }
                    AwardMetric { key, value }
                })
                .collect();
            entry.candidate
        })
        .collect()
}
