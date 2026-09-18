use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::DegradationContext;
use crate::resolution::duel_kind::{logistic_slope_for, DuelKind};
use crate::resolution::duel_noise::sample_player_noise_with_pressure;
use crate::resolution::resolver::DuelResolutionRequest;
use arlo_domain::sport_constants::HOME_FIELD_ADVANTAGE_LOGIT;
use arlo_math::stats::{calculate_duel_probability, calculate_net_advantage, Probability};
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EvaluatedDuel {
    pub kind: DuelKind,
    pub effective_attacker: f64,
    pub effective_defender: f64,
    pub noisy_attacker: f64,
    pub noisy_defender: f64,
    pub win_probability: Probability,
    pub net_advantage: f64,
    pub slope: f64,
}

pub fn evaluate_duel<R: Rng + ?Sized>(
    request: &DuelResolutionRequest<'_>,
    rng: &mut R,
) -> EvaluatedDuel {
    let effective_attacker = request
        .attacker_team_power
        .unwrap_or(request.attacker_rating);
    let effective_defender = request
        .defender_team_power
        .unwrap_or(request.defender_rating);

    let pressure = request
        .pressure
        .as_ref()
        .or_else(|| request.context.pressure());

    let deg_ctx_a = DegradationContext::new(&request.attacker_state);
    let table_a;
    let opt_table_a = match request.attacker_table {
        Some(table) => Some(table),
        None => match (request.attacker_primary, request.attribute_keys) {
            (Some(p), Some(keys)) => {
                table_a = PlayerAttributeTable::from_player(p, keys);
                Some(&table_a)
            }
            _ => None,
        },
    };
    let noise_a = match opt_table_a {
        Some(table) => sample_player_noise_with_pressure(table, &deg_ctx_a, pressure, rng),
        None => 0.0,
    };

    let deg_ctx_b = DegradationContext::new(&request.defender_state);
    let table_b;
    let opt_table_b = match request.defender_table {
        Some(table) => Some(table),
        None => match (request.defender_primary, request.attribute_keys) {
            (Some(p), Some(keys)) => {
                table_b = PlayerAttributeTable::from_player(p, keys);
                Some(&table_b)
            }
            _ => None,
        },
    };
    let noise_b = match opt_table_b {
        Some(table) => sample_player_noise_with_pressure(table, &deg_ctx_b, pressure, rng),
        None => 0.0,
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

    let noisy_attacker = effective_attacker + noise_a;
    let noisy_defender = effective_defender + noise_b;
    let slope = request
        .slope_override
        .unwrap_or_else(|| logistic_slope_for(request.kind));

    let win_prob = calculate_duel_probability(noisy_attacker, noisy_defender, slope, hfa_logit);
    let net_advantage = calculate_net_advantage(effective_attacker, effective_defender);

    EvaluatedDuel {
        kind: request.kind,
        effective_attacker,
        effective_defender,
        noisy_attacker,
        noisy_defender,
        win_probability: win_prob,
        net_advantage,
        slope,
    }
}