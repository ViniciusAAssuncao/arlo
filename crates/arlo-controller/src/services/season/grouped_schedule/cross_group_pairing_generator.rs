use crate::error::{ControllerError, ControllerResult};
use crate::services::season::round_robin::RoundRobinMatch;
use uuid::Uuid;

pub fn generate_cross_group_pairings(
    group_a_team_ids: &[Uuid],
    group_b_team_ids: &[Uuid],
    start_round_index: u32,
    mirrored: bool,
) -> ControllerResult<Vec<RoundRobinMatch>> {
    if group_a_team_ids.len() != group_b_team_ids.len() {
        return Err(ControllerError::Validation(format!(
            "Cross group pairing requires groups of equal size, got {} and {}",
            group_a_team_ids.len(),
            group_b_team_ids.len()
        )));
    }

    if group_a_team_ids.is_empty() {
        return Ok(Vec::new());
    }

    let pair_count = group_a_team_ids.len();
    let total_matches = if mirrored { pair_count * 2 } else { pair_count };
    let mut matches = Vec::with_capacity(total_matches);

    for i in 0..pair_count {
        matches.push(RoundRobinMatch::new(
            start_round_index,
            group_a_team_ids[i],
            group_b_team_ids[i],
        ));
    }

    if mirrored {
        for i in 0..pair_count {
            matches.push(RoundRobinMatch::new(
                start_round_index + 1,
                group_b_team_ids[i],
                group_a_team_ids[i],
            ));
        }
    }

    Ok(matches)
}
