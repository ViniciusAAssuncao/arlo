use crate::attributes::PlayerAttributeTable;
use crate::physical::PhysicalState;
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::{logistic_slope_for, DuelKind};
use crate::resolution::duel_noise::{
    sample_player_noise, sample_player_noise_from_table_with_baseline,
};
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{calculate_side_rating, RatingParticipants};
use crate::resolution::outcome::DuelOutcome;
use arlo_domain::sport_constants::HOME_FIELD_ADVANTAGE_LOGIT;
use arlo_domain::{AttributeKey, Player};
use arlo_math::stats::contrast::bradley_terry_with_offset;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub struct DuelResolutionRequest<'a> {
    pub kind: DuelKind,
    pub attacker_rating: f64,
    pub defender_rating: f64,
    pub attacker_primary: &'a Player,
    pub defender_primary: &'a Player,
    pub attacker_state: PhysicalState,
    pub defender_state: PhysicalState,
    pub attribute_keys: &'a HashMap<Uuid, AttributeKey>,
    pub context: &'a DuelContext,
    pub attacker_table: Option<&'a PlayerAttributeTable>,
    pub defender_table: Option<&'a PlayerAttributeTable>,
}

impl<'a> DuelResolutionRequest<'a> {
    pub fn new(
        kind: DuelKind,
        attacker_rating: f64,
        defender_rating: f64,
        attacker_primary: &'a Player,
        defender_primary: &'a Player,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        context: &'a DuelContext,
    ) -> Self {
        Self {
            kind,
            attacker_rating,
            defender_rating,
            attacker_primary,
            defender_primary,
            attacker_state: PhysicalState::initial(),
            defender_state: PhysicalState::initial(),
            attribute_keys,
            context,
            attacker_table: None,
            defender_table: None,
        }
    }

    pub fn with_states(
        kind: DuelKind,
        attacker_rating: f64,
        defender_rating: f64,
        attacker_primary: &'a Player,
        defender_primary: &'a Player,
        attacker_state: PhysicalState,
        defender_state: PhysicalState,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        context: &'a DuelContext,
    ) -> Self {
        Self {
            kind,
            attacker_rating,
            defender_rating,
            attacker_primary,
            defender_primary,
            attacker_state,
            defender_state,
            attribute_keys,
            context,
            attacker_table: None,
            defender_table: None,
        }
    }

    pub fn from_participants(
        kind: DuelKind,
        attacker_primary: &'a Player,
        attackers: RatingParticipants<'a>,
        defender_primary: &'a Player,
        defenders: RatingParticipants<'a>,
        attribute_keys: &'a HashMap<Uuid, AttributeKey>,
        context: &'a DuelContext,
    ) -> Self {
        let (attacker_profile, defender_profile) = get_duel_profiles(kind);
        let attacker_rating = calculate_side_rating(attackers, attribute_keys, attacker_profile);
        let defender_rating = calculate_side_rating(defenders, attribute_keys, defender_profile);
        let default_state = PhysicalState::initial();
        let attacker_state = attackers
            .fatigue_lookup
            .map(|f| f(&attacker_primary.id()))
            .unwrap_or(default_state);
        let defender_state = defenders
            .fatigue_lookup
            .map(|f| f(&defender_primary.id()))
            .unwrap_or(default_state);

        let attacker_table = attackers
            .attribute_tables
            .and_then(|m| m.get(&attacker_primary.id()));
        let defender_table = defenders
            .attribute_tables
            .and_then(|m| m.get(&defender_primary.id()));

        Self {
            kind,
            attacker_rating,
            defender_rating,
            attacker_primary,
            defender_primary,
            attacker_state,
            defender_state,
            attribute_keys,
            context,
            attacker_table,
            defender_table,
        }
    }

    pub fn with_tables(
        mut self,
        attacker_table: Option<&'a PlayerAttributeTable>,
        defender_table: Option<&'a PlayerAttributeTable>,
    ) -> Self {
        self.attacker_table = attacker_table;
        self.defender_table = defender_table;
        self
    }
}

pub fn calculate_velocity_mitigation(
    kind: DuelKind,
    attacker_won: bool,
    net_advantage: f64,
) -> f64 {
    let base = match kind {
        DuelKind::ArtroBreakthrough | DuelKind::RunBreakthrough => {
            if attacker_won {
                0.80 + (net_advantage * 0.04)
            } else {
                0.25 + (net_advantage * 0.03)
            }
        }
        DuelKind::CentralBlock | DuelKind::LateralBlock => {
            if attacker_won {
                0.70 + (net_advantage * 0.03)
            } else {
                0.20 + (net_advantage * 0.02)
            }
        }
        DuelKind::BallSecurityCarry | DuelKind::BallSecurityDistribution => {
            if attacker_won {
                0.60 + (net_advantage * 0.04)
            } else {
                0.00
            }
        }
        _ => {
            if attacker_won {
                0.85 + (net_advantage * 0.02)
            } else {
                0.35 + (net_advantage * 0.02)
            }
        }
    };

    if attacker_won {
        base.clamp(0.40, 1.00)
    } else {
        base.clamp(0.00, 0.40)
    }
}

pub fn resolve_duel<R: Rng + ?Sized>(
    request: DuelResolutionRequest<'_>,
    rng: &mut R,
) -> DuelOutcome {
    let noise_a = match request.attacker_table {
        Some(table) => sample_player_noise_from_table_with_baseline(
            request.attacker_primary,
            table,
            &request.attacker_state,
            rng,
        ),
        None => sample_player_noise(
            request.attacker_primary,
            request.attribute_keys,
            &request.attacker_state,
            rng,
        ),
    };
    let noise_b = match request.defender_table {
        Some(table) => sample_player_noise_from_table_with_baseline(
            request.defender_primary,
            table,
            &request.defender_state,
            rng,
        ),
        None => sample_player_noise(
            request.defender_primary,
            request.attribute_keys,
            &request.defender_state,
            rng,
        ),
    };

    let mut hfa_logit = 0.0;
    if request.context.attacker_is_home() {
        hfa_logit += HOME_FIELD_ADVANTAGE_LOGIT;
    }
    if request.context.defender_is_home() {
        hfa_logit -= HOME_FIELD_ADVANTAGE_LOGIT;
    }
    hfa_logit += request.context.aggression_logit_offset();
    hfa_logit += request.context.physicality_logit_offset();
    hfa_logit += request.context.misdirection_logit_offset();

    let noisy_attacker = request.attacker_rating + noise_a;
    let noisy_defender = request.defender_rating + noise_b;
    let slope = logistic_slope_for(request.kind);

    let win_prob = bradley_terry_with_offset(noisy_attacker, noisy_defender, slope, hfa_logit);

    let attacker_won = win_prob.sample(rng);
    let net_advantage = request.attacker_rating - request.defender_rating;
    let velocity_mitigation =
        calculate_velocity_mitigation(request.kind, attacker_won, net_advantage);

    let outcome = DuelOutcome::with_mitigation(
        request.kind,
        attacker_won,
        request.attacker_rating,
        request.defender_rating,
        win_prob,
        net_advantage,
        velocity_mitigation,
    );

    crate::psychology::systems::instrumentation::instrument_duel_outcome(
        &outcome,
        request.attacker_primary.id(),
        request.defender_primary.id(),
    );

    outcome
}
