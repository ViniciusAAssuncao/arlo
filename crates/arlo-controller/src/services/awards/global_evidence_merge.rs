use arlo_domain::{AwardCandidateEvidence, AwardMetric};
use std::collections::{BTreeMap, BTreeSet};
use uuid::Uuid;

struct CandidateAccumulator {
    candidate: AwardCandidateEvidence,
    representative_matches: u32,
    competitions: BTreeSet<Uuid>,
    metrics: BTreeMap<String, f64>,
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
