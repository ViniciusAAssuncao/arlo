use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{
    calculate_player_duel_rating, calculate_side_rating_from_index, identify_lead_player_from_index,
};
use crate::resolution::resolver::resolve_duel;
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use arlo_domain::{AttributeKey, Player, Position};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct SecurityResolutionResult {
    pub turnover_team_id: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub duel_outcome: AttributedDuelOutcome,
}

pub fn resolve_ball_security<R: Rng + ?Sized>(
    security_kind: DuelKind,
    ball_carrier: &Player,
    carrier_position: Position,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    defense_team_id: Uuid,
    context: &DuelContext,
    rng: &mut R,
) -> SecurityResolutionResult {
    let (attacker_profile, defender_profile) = get_duel_profiles(security_kind);
    let attacker_rating = calculate_player_duel_rating(
        ball_carrier,
        carrier_position,
        attribute_keys,
        &attacker_profile,
    );
    let defender_rating = calculate_side_rating_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &defender_profile,
    );
    let lead_defender = identify_lead_player_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        &defender_profile,
    );
    let defender_primary = lead_defender.unwrap_or(defenders[0]);

    let raw_outcome = resolve_duel(
        security_kind,
        attacker_rating,
        defender_rating,
        ball_carrier,
        defender_primary,
        attribute_keys,
        context,
        rng,
    );

    let attacker_ids = vec![ball_carrier.id()];
    let defender_ids = defenders.iter().map(|p| p.id()).collect();
    let duel_outcome = AttributedDuelOutcome::new(raw_outcome, attacker_ids, defender_ids);

    if raw_outcome.attacker_won() {
        SecurityResolutionResult {
            turnover_team_id: None,
            recovering_player_id: None,
            duel_outcome,
        }
    } else {
        SecurityResolutionResult {
            turnover_team_id: Some(defense_team_id),
            recovering_player_id: lead_defender.map(|p| p.id()),
            duel_outcome,
        }
    }
}