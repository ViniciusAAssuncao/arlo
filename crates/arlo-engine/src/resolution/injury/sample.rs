use crate::input::MatchInput;
use crate::resolution::ratings::RatingIndex;
use crate::state::MatchState;
use arlo_domain::{AttributeKey, InjuryMechanism, InjurySeverityGrade};
use arlo_events::{DuelResolved, InjuryIncidentRecorded};
use arlo_math::Probability;
use rand::Rng;
use uuid::Uuid;

pub(super) struct Exposure {
    player_id: Uuid,
    mechanism: InjuryMechanism,
    base_probability: f64,
}

impl Exposure {
    pub(super) fn duel(duel: &DuelResolved, state: &mut MatchState) -> Option<Self> {
        let attacker = duel.primary_attacker()?;
        let defender = duel.primary_defender()?;
        let loser = if duel.attacker_won() { defender } else { attacker };
        let winner = if duel.attacker_won() { attacker } else { defender };
        let player_id = if state.rng_mut().gen_bool(0.68) { loser } else { winner };
        Some(Self {
            player_id,
            mechanism: InjuryMechanism::Contact,
            base_probability: 0.00045,
        })
    }

    pub(super) fn carry(player_id: Uuid) -> Self {
        Self {
            player_id,
            mechanism: InjuryMechanism::NonContact,
            base_probability: 0.00009,
        }
    }
}

pub(super) fn sample_injury(
    input: &MatchInput,
    state: &mut MatchState,
    exposure: Exposure,
) -> Option<InjuryIncidentRecorded> {
    let (team, team_state) = if input.home().roster().iter().any(|p| p.id() == exposure.player_id) {
        (input.home(), state.home())
    } else {
        (input.away(), state.away())
    };
    if !team_state.active_player_ids().contains(&exposure.player_id)
        || team_state.injured_player_ids().contains(&exposure.player_id)
    {
        return None;
    }
    let ratings = RatingIndex::new(input, state);
    let natural_fitness = ratings.player_value(team, exposure.player_id, AttributeKey::NaturalFitness).ok()?;
    let balance = ratings.player_value(team, exposure.player_id, AttributeKey::Balance).ok()?;
    let stamina = ratings.player_value(team, exposure.player_id, AttributeKey::Stamina).ok()?;
    let resilience = 0.45 * natural_fitness + 0.30 * balance + 0.25 * stamina;
    let probability = (exposure.base_probability * (1.0 - (resilience - 10.0) * 0.025))
        .clamp(exposure.base_probability * 0.65, exposure.base_probability * 1.35);
    if state.rng_mut().gen_range(0.0..1.0) >= probability {
        return None;
    }
    let mut candidates: Vec<_> = input.injury_catalog()
        .definitions_for_mechanism(exposure.mechanism)
        .iter()
        .filter_map(|id| input.injury_catalog().definition(id))
        .collect();
    candidates.sort_by(|a, b| a.code().cmp(b.code()));
    let total_weight: f64 = candidates.iter().map(|definition| definition.relative_frequency()).sum();
    if total_weight <= 0.0 {
        return None;
    }
    let mut draw = state.rng_mut().gen_range(0.0..total_weight);
    let definition = candidates.iter().find(|definition| {
        draw -= definition.relative_frequency();
        draw < 0.0
    }).or_else(|| candidates.last())?;
    let grade = sample_grade(definition.code(), state);
    let grade = match input.injury_catalog().minimum_grade(definition.id()) {
        Some(InjurySeverityGrade::Grade3) => InjurySeverityGrade::Grade3,
        Some(InjurySeverityGrade::Grade2) if grade == InjurySeverityGrade::Grade1 => {
            InjurySeverityGrade::Grade2
        }
        _ => grade,
    };
    Some(InjuryIncidentRecorded::new(
        exposure.player_id,
        team.team_id(),
        exposure.mechanism,
        definition.body_region(),
        grade,
        definition.id(),
        Probability::new_clamped(probability),
    ))
}

fn sample_grade(code: &str, state: &mut MatchState) -> InjurySeverityGrade {
    if code.contains("_GRADE1") || code.contains("_MILD") {
        return InjurySeverityGrade::Grade1;
    }
    if code.contains("_GRADE2") || code.contains("_MODERATE") {
        return InjurySeverityGrade::Grade2;
    }
    if code.contains("_GRADE3") || code.contains("_SEVERE") {
        return InjurySeverityGrade::Grade3;
    }
    let draw = state.rng_mut().gen_range(0.0..1.0);
    if code.contains("ACL_TEAR") {
        return if draw < 0.55 { InjurySeverityGrade::Grade2 } else { InjurySeverityGrade::Grade3 };
    }
    if draw < 0.77 {
        InjurySeverityGrade::Grade1
    } else if draw < 0.97 {
        InjurySeverityGrade::Grade2
    } else {
        InjurySeverityGrade::Grade3
    }
}
