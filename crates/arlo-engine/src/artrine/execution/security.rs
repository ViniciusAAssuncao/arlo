use crate::artrine::execution::context::ActionExecutionContext;
use crate::physical::systems::degradation::calculate_effective_player_speed;
use crate::physical::{FatigueState, PhysicalState};
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::duel_timing::derive_duel_duration;
use crate::resolution::group_rating::{
    calculate_player_duel_rating_with_state, calculate_side_rating,
    identify_lead_player_from_index, RatingParticipants,
};
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::spatial::positioning_drift::get_drifted_defender_position;
use crate::spatial::proximity::calculate_distance_mirim;
use crate::spatial::DynamicSpatialMap;
use crate::time::{DurationComponentKind, DurationLedger};
use arlo_domain::sport_constants::PROXIMITY_CONTEST_RADIUS_MIRIM;
use arlo_domain::{AttributeKey, Player, Position};
use arlo_math::units::{Position as VectorPosition, Speed};
use rand::Rng;
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct SecurityResolutionResult {
    pub turnover_team_id: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub duel_outcome: AttributedDuelOutcome,
}

pub struct ProximitySecurityContestResult {
    pub turnover_team_id: Option<Uuid>,
    pub recovering_player_id: Option<Uuid>,
    pub duel_outcome: Option<AttributedDuelOutcome>,
}

pub fn resolve_ball_security<F, R>(
    security_kind: DuelKind,
    ball_carrier: &Player,
    carrier_position: Position,
    defenders: &[&Player],
    defense_position_index: &HashMap<Uuid, Position>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    defense_team_id: Uuid,
    context: &DuelContext,
    fatigue_for: &F,
    rng: &mut R,
) -> SecurityResolutionResult
where
    F: Fn(&Uuid) -> PhysicalState,
    R: Rng + ?Sized,
{
    let (attacker_profile, defender_profile) = get_duel_profiles(security_kind);
    let carrier_state = fatigue_for(&ball_carrier.id());
    let attacker_rating = calculate_player_duel_rating_with_state(
        ball_carrier,
        carrier_position,
        attribute_keys,
        attacker_profile,
        &carrier_state,
    );
    let defender_rating = calculate_side_rating(
        RatingParticipants::from_slice_with_index(defenders, defense_position_index)
            .with_fatigue(fatigue_for),
        attribute_keys,
        defender_profile,
    );
    let lead_defender = identify_lead_player_from_index(
        defenders,
        defense_position_index,
        attribute_keys,
        defender_profile,
    );
    let defender_primary = lead_defender.unwrap_or(defenders[0]);
    let defender_state = fatigue_for(&defender_primary.id());

    let req = DuelResolutionRequest::with_states(
        security_kind,
        attacker_rating,
        defender_rating,
        ball_carrier,
        defender_primary,
        carrier_state,
        defender_state,
        attribute_keys,
        context,
    );
    let raw_outcome = resolve_duel(req, rng);

    let attacker_ids = smallvec![ball_carrier.id()];
    let defender_ids: SmallVec<[Uuid; 4]> = defenders.iter().map(|p| p.id()).collect();
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

pub fn resolve_proximity_ball_security<F, R>(
    ctx: &ActionExecutionContext<'_, F>,
    carrier: &Player,
    carrier_position: Position,
    carrier_pos_vec: VectorPosition,
    carrier_speed: Speed,
    nearest_def_opt: Option<(&Player, VectorPosition)>,
    spatial_map: &DynamicSpatialMap,
    security_kind: DuelKind,
    ledger: &mut DurationLedger,
    rng: &mut R,
) -> ProximitySecurityContestResult
where
    F: Fn(&Uuid) -> FatigueState,
    R: Rng + ?Sized,
{
    let close_defenders: Vec<&Player> = match nearest_def_opt {
        Some((_, pos))
            if calculate_distance_mirim(carrier_pos_vec, pos) <= PROXIMITY_CONTEST_RADIUS_MIRIM =>
        {
            ctx.defenders
                .iter()
                .copied()
                .filter(|cand| {
                    get_drifted_defender_position(cand, spatial_map, ctx.attribute_keys, rng)
                        .map(|p| {
                            calculate_distance_mirim(carrier_pos_vec, p)
                                <= PROXIMITY_CONTEST_RADIUS_MIRIM
                        })
                        .unwrap_or(false)
                })
                .collect()
        }
        _ => Vec::new(),
    };

    if close_defenders.is_empty() {
        return ProximitySecurityContestResult {
            turnover_team_id: None,
            recovering_player_id: None,
            duel_outcome: None,
        };
    }

    if let Some((closest_def, closest_pos)) = nearest_def_opt {
        let closest_def_state = ctx.fatigue(&closest_def.id());
        let closest_def_speed =
            calculate_effective_player_speed(closest_def, ctx.attribute_keys, &closest_def_state);
        let sec_duration = derive_duel_duration(
            carrier_pos_vec,
            carrier_speed,
            closest_pos,
            closest_def_speed,
        );
        ledger.record_live(DurationComponentKind::BallSecurityEngagement, sec_duration);
    }

    let sec_context = ctx.duel_context.for_duel_kind(security_kind);
    let sec_result = resolve_ball_security(
        security_kind,
        carrier,
        carrier_position,
        &close_defenders,
        ctx.defense_position_index,
        ctx.attribute_keys,
        ctx.defense_team_id,
        &sec_context,
        ctx.fatigue_for,
        rng,
    );

    ProximitySecurityContestResult {
        turnover_team_id: sec_result.turnover_team_id,
        recovering_player_id: sec_result.recovering_player_id,
        duel_outcome: Some(sec_result.duel_outcome),
    }
}