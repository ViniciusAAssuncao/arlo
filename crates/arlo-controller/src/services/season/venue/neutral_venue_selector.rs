use arlo_domain::{Venue, VenueKind};
use uuid::Uuid;

pub fn select_neutral_venue(
    venues: &[Venue],
    home_team_id: Uuid,
    away_team_id: Uuid,
    seed: u64,
) -> Option<Uuid> {
    let mut eligible: Vec<&Venue> = venues
        .iter()
        .filter(|v| {
            v.kind() == VenueKind::MatchStadium
                && v.owner_team_id() != Some(home_team_id)
                && v.owner_team_id() != Some(away_team_id)
        })
        .collect();

    if eligible.is_empty() {
        return None;
    }

    eligible.sort_by_key(|v| v.id());
    let selected_index = (seed as usize) % eligible.len();
    Some(eligible[selected_index].id())
}