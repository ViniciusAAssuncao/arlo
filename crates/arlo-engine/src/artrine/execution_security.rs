use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::identify_lead_player;
use crate::resolution::outcome::DuelOutcome;
use crate::resolution::resolver::resolve_duel_for_participants;
use crate::resolution::{DuelContext, DuelKind};
use arlo_domain::{AttributeKey, Player};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct SecurityResolutionResult {
    pub turnover_team_id: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub duel_outcome: DuelOutcome,
}

pub fn resolve_ball_security<R: Rng + ?Sized>(
    security_kind: DuelKind,
    ball_carrier: &Player,
    defenders: &[&Player],
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    defense_team_id: Uuid,
    context: &DuelContext,
    rng: &mut R,
) -> SecurityResolutionResult {
    let (_, defender_profile) = get_duel_profiles(security_kind);
    let duel_outcome = resolve_duel_for_participants(
        security_kind,
        &[ball_carrier],
        defenders,
        attribute_keys,
        context,
        rng,
    );

    if duel_outcome.attacker_won() {
        SecurityResolutionResult {
            turnover_team_id: None,
            recovering_player_id: None,
            duel_outcome,
        }
    } else {
        let lead_defender = identify_lead_player(defenders, attribute_keys, &defender_profile);
        SecurityResolutionResult {
            turnover_team_id: Some(defense_team_id),
            recovering_player_id: lead_defender.map(|p| p.id()),
            duel_outcome,
        }
    }
}