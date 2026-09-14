use crate::services::season::round_robin::circle_method_generator::RoundRobinMatch;

pub fn expand_double_round_robin(single_leg_matches: &[RoundRobinMatch]) -> Vec<RoundRobinMatch> {
    if single_leg_matches.is_empty() {
        return Vec::new();
    }

    let single_rounds_count = single_leg_matches
        .iter()
        .map(|m| m.round_index)
        .max()
        .map(|max_idx| max_idx + 1)
        .unwrap_or(0);

    let mut result = Vec::with_capacity(single_leg_matches.len() * 2);
    result.extend_from_slice(single_leg_matches);

    for m in single_leg_matches {
        result.push(RoundRobinMatch {
            round_index: m.round_index + single_rounds_count,
            home_team_id: m.away_team_id,
            away_team_id: m.home_team_id,
            is_neutral_venue: false,
        });
    }

    result
}
