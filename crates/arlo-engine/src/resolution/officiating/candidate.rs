use crate::error::EngineResult;
use crate::input::MatchInput;
use crate::resolution::ratings::RatingIndex;
use crate::state::MatchState;
use arlo_domain::{AttributeKey, FaultOffenderRole, PositionLine};
use arlo_events::{DuelResolved, FoulOrigin, RefereeDecisionResolved};
use rand::Rng;
use uuid::Uuid;

pub(in crate::resolution) fn sample_decision(
    input: &MatchInput,
    state: &mut MatchState,
    duel: &DuelResolved,
) -> EngineResult<Option<RefereeDecisionResolved>> {
    let (Some(attacker_id), Some(defender_id)) = (duel.primary_attacker(), duel.primary_defender()) else {
        return Ok(None);
    };
    sample_context_decision(input, state, duel.kind().as_str(), attacker_id, defender_id, FoulOrigin::ContactDuel(duel.kind()))
}

pub(in crate::resolution) fn sample_context_decision(
    input: &MatchInput,
    state: &mut MatchState,
    context: &str,
    attacker_id: Uuid,
    defender_id: Uuid,
    origin: FoulOrigin,
) -> EngineResult<Option<RefereeDecisionResolved>> {
    let attacker_is_home = input.home().roster().iter().any(|player| player.id() == attacker_id);
    let (offense, defense) = if attacker_is_home {
        (input.home(), input.away())
    } else {
        (input.away(), input.home())
    };
    let entries_all = input.fault_catalog().activations_for_context(context);
    if entries_all.is_empty() { return Ok(None); }
    let defender_eligible = entries_all.iter().any(|(_, role, _)| matches!(role, FaultOffenderRole::Defense | FaultOffenderRole::Either));
    let offense_eligible = entries_all.iter().any(|(_, role, _)| matches!(role, FaultOffenderRole::Offense | FaultOffenderRole::Either));
    let offender_is_defender = if !offense_eligible { true } else if !defender_eligible { false } else { state.rng_mut().gen_range(0.0..1.0) < 0.62 };
    let (offender_id, offender_team_id, opposing_id, opposing_team_id, role) =
        if offender_is_defender {
            (defender_id, defense.team_id(), attacker_id, offense.team_id(), FaultOffenderRole::Defense)
        } else {
            (attacker_id, offense.team_id(), defender_id, defense.team_id(), FaultOffenderRole::Offense)
        };
    let entries: Vec<_> = input
        .fault_catalog()
        .activations_for_context(context)
        .iter()
        .filter(|(_, eligible_role, _)| *eligible_role == role || *eligible_role == FaultOffenderRole::Either)
        .copied()
        .collect();
    if entries.is_empty() {
        return Ok(None);
    }
    let ratings = RatingIndex::new(input, state);
    let offender_team = if offender_is_defender { defense } else { offense };
    let aggression = ratings.player_value(offender_team, offender_id, AttributeKey::Aggressiveness)?;
    let factual_probability = (0.008 + (10.0 - aggression) * 0.00055).clamp(0.002, 0.02);
    let factual_foul = state.rng_mut().gen_range(0.0..1.0) < factual_probability;
    let head_rigor = referee_value(input, 0, AttributeKey::Rigor);
    let head_consistency = referee_value(input, 0, AttributeKey::Consistency);
    let head_authority = referee_value(input, 0, AttributeKey::Authority);
    let crowd_bias = if offender_team_id == input.home().team_id() { -0.015 } else { 0.015 };
    let original_call_probability = if factual_foul {
        (0.71 + (head_rigor - 10.0) * 0.012 + (head_consistency - 10.0) * 0.008
            + (head_authority - 10.0) * 0.004 + crowd_bias).clamp(0.45, 0.94)
    } else {
        (0.002 + (head_rigor - 10.0) * 0.00035 - (head_consistency - 10.0) * 0.00015
            + crowd_bias * 0.04).clamp(0.0002, 0.009)
    };
    let original_call = state.rng_mut().gen_range(0.0..1.0) < original_call_probability;
    if !factual_foul && !original_call {
        return Ok(None);
    }
    let definition_id = select_definition(&entries, state);
    let peace_intervened = if original_call == factual_foul {
        false
    } else {
        let peace_authority = referee_value(input, 1, AttributeKey::Authority);
        let peace_composure = referee_value(input, 1, AttributeKey::Composure);
        let chance = (0.48 + (peace_authority - 10.0) * 0.018
            + (peace_composure - 10.0) * 0.013).clamp(0.18, 0.86);
        state.rng_mut().gen_range(0.0..1.0) < chance
    };
    Ok(Some(RefereeDecisionResolved::new(
        offender_id,
        offender_team_id,
        opposing_id,
        opposing_team_id,
        origin,
        Some(definition_id),
        factual_foul,
        original_call,
        peace_intervened,
    )))
}

fn select_definition(entries: &[(Uuid, FaultOffenderRole, f64)], state: &mut MatchState) -> Uuid {
    let total: f64 = entries.iter().map(|(_, _, weight)| weight).sum();
    let mut draw = state.rng_mut().gen_range(0.0..total);
    for &(definition_id, _, weight) in entries {
        if draw < weight {
            return definition_id;
        }
        draw -= weight;
    }
    entries[entries.len() - 1].0
}

fn referee_value(input: &MatchInput, index: usize, key: AttributeKey) -> f64 {
    input.referees()[index]
        .attributes()
        .iter()
        .find(|attribute| input.referee_attribute_keys().get(&attribute.attribute_definition_id()) == Some(&key))
        .map(|attribute| attribute.value() as f64)
        .unwrap_or(10.0)
}

pub(in crate::resolution) fn sample_line_fault(
    input: &MatchInput,
    state: &mut MatchState,
    receiver_id: Uuid,
    defender_id: Uuid,
) -> EngineResult<Option<RefereeDecisionResolved>> {
    let receiver_is_home = input.home().roster().iter().any(|player| player.id() == receiver_id);
    let (offense, defense) = if receiver_is_home { (input.home(), input.away()) } else { (input.away(), input.home()) };
    let ratings = RatingIndex::new(input, state);
    let Some(assignment) = offense.lineup().assignments().iter()
        .find(|a| ratings.slot_player_id(offense, a.player_id()) == receiver_id) else { return Ok(None) };
    let depth = offense.formation().slots().get(assignment.formation_slot_index())
        .and_then(|slot| slot.pitch_length_ratio()).unwrap_or(match assignment.position().line() {
            PositionLine::OffensiveLine => 0.7,
            PositionLine::BackLine => 0.45,
            PositionLine::DefenseLine => 0.2,
            PositionLine::Goalguard => 0.05,
        });
    let positioning = ratings.player_value(offense, receiver_id, AttributeKey::Positioning)?;
    let factual_probability = ((depth - 0.4).max(0.0) * 0.024 * (1.0 - (positioning - 10.0) * 0.015)).clamp(0.0, 0.014);
    let factual_foul = state.rng_mut().gen_range(0.0..1.0) < factual_probability;
    let head_consistency = referee_value(input, 0, AttributeKey::Consistency);
    let original_call_probability = if factual_foul { (0.70 + (head_consistency - 10.0) * 0.018).clamp(0.45, 0.90) }
        else { (0.0015 - (head_consistency - 10.0) * 0.0001).clamp(0.0002, 0.004) };
    let original_call = state.rng_mut().gen_range(0.0..1.0) < original_call_probability;
    if !factual_foul && !original_call { return Ok(None); }
    let peace_intervened = if factual_foul == original_call { false } else {
        let chance = (0.52 + (referee_value(input, 1, AttributeKey::Authority) - 10.0) * 0.018
            + (referee_value(input, 1, AttributeKey::Composure) - 10.0) * 0.013).clamp(0.18, 0.87);
        state.rng_mut().gen_range(0.0..1.0) < chance
    };
    Ok(Some(RefereeDecisionResolved::new(receiver_id, offense.team_id(), defender_id, defense.team_id(),
        FoulOrigin::LineFault, None, factual_foul, original_call, peace_intervened)))
}
