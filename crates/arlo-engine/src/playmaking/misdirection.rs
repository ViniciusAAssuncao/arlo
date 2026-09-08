use crate::lineup_runtime::Lineup;
use arlo_domain::sport_constants::{HOME_FIELD_ADVANTAGE_LOGIT, MISDIRECTION_MAX_LOGIT_MULTIPLIER};
use arlo_tactics::{PlayCall, RouteAssignment};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_misdirection_logit_offset(
    play_call: Option<&PlayCall>,
    route_index: &HashMap<Uuid, RouteAssignment>,
    lineup: &Lineup,
) -> f64 {
    if let Some(pc) = play_call {
        if let Some(misdirection) = pc.misdirection() {
            let decoy_player = lineup.player_at_slot_index(misdirection.decoy_slot_index());
            let true_carrier_player =
                lineup.player_at_slot_index(misdirection.true_carrier_slot_index());

            if let (Some(decoy), Some(true_carrier)) = (decoy_player, true_carrier_player) {
                let decoy_route = route_index.get(&decoy.id());
                let true_route = route_index.get(&true_carrier.id());

                if let (Some(d_route), Some(t_route)) = (decoy_route, true_route) {
                    let similarity = misdirection.geometric_similarity(d_route, t_route);
                    return -MISDIRECTION_MAX_LOGIT_MULTIPLIER
                        * HOME_FIELD_ADVANTAGE_LOGIT
                        * similarity;
                }
            }
        }
    }
    0.0
}