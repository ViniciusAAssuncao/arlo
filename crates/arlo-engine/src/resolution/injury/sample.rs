use crate::input::MatchInput;
use crate::resolution::ratings::RatingIndex;
use crate::state::MatchState;
use arlo_domain::{AttributeKey, InjuryCatalog, InjuryMechanism, InjurySeverityGrade};
use arlo_events::{DuelResolved, InjuryIncidentRecorded, PasserContactResolved};
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

    pub(super) fn passer_contact(contact: &PasserContactResolved) -> Option<Self> {
        contact.late().then_some(Self {
            player_id: contact.passer_id(),
            mechanism: InjuryMechanism::Contact,
            base_probability: if contact.violent() { 0.0018 } else if contact.rough() { 0.0008 } else { 0.0004 },
        })
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
    if !team_state.active_player_ids().contains(&exposure.player_id) {
        return None;
    }
    if let Some((definition_id, grade)) = state.match_injury(exposure.player_id) {
        let fatigue = 1.0 - state.player_energy(exposure.player_id);
        if grade != InjurySeverityGrade::Grade3
            && input.injury_catalog().has_recovery_profile(definition_id, InjurySeverityGrade::Grade3)
            && input.injury_catalog().requires_withdrawal(definition_id, InjurySeverityGrade::Grade3)
            && state.rng_mut().gen_range(0.0..1.0) < 0.004 * (1.0 + 2.0 * fatigue)
        {
            let definition = input.injury_catalog().definition(&definition_id)?;
            return Some(InjuryIncidentRecorded::new(
                exposure.player_id, team.team_id(), exposure.mechanism,
                definition.body_region(), InjurySeverityGrade::Grade3, definition_id,
                Probability::new_clamped(0.004 * (1.0 + 2.0 * fatigue)),
            ));
        }
        return None;
    }
    let ratings = RatingIndex::new(input, state);
    let natural_fitness = ratings.player_value(team, exposure.player_id, AttributeKey::NaturalFitness).ok()?;
    let balance = ratings.player_value(team, exposure.player_id, AttributeKey::Balance).ok()?;
    let stamina = ratings.player_value(team, exposure.player_id, AttributeKey::Stamina).ok()?;
    let resilience = 0.45 * natural_fitness + 0.30 * balance + 0.25 * stamina;
    let energy = state.player_energy(exposure.player_id);
    let fatigue_risk = 1.0 + 1.8 * (1.0 - energy).powi(2);
    let probability = (exposure.base_probability * (1.0 - (resilience - 10.0) * 0.025) * fatigue_risk)
        .clamp(exposure.base_probability * 0.65, exposure.base_probability * 3.5);
    if state.rng_mut().gen_range(0.0..1.0) >= probability {
        return None;
    }
    let mut candidates: Vec<_> = input.injury_catalog()
        .definitions_for_mechanism(exposure.mechanism)
        .iter()
        .filter_map(|id| input.injury_catalog().definition(id))
        .filter(|definition| supported_grades(input.injury_catalog(), definition.id(), definition.code()).next().is_some())
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
    let grade = sample_grade(input.injury_catalog(), definition.id(), definition.code(), state)?;
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

fn sample_grade(
    catalog: &InjuryCatalog,
    id: Uuid,
    code: &str,
    state: &mut MatchState,
) -> Option<InjurySeverityGrade> {
    let total: u32 = supported_grades(catalog, id, code).map(|(_, weight)| weight).sum();
    let mut draw = state.rng_mut().gen_range(0..total);
    for (grade, weight) in supported_grades(catalog, id, code) {
        if draw < weight {
            return Some(grade);
        }
        draw -= weight;
    }
    None
}

fn supported_grades<'a>(
    catalog: &'a InjuryCatalog,
    id: Uuid,
    code: &'a str,
) -> impl Iterator<Item = (InjurySeverityGrade, u32)> + 'a {
    [InjurySeverityGrade::Grade1, InjurySeverityGrade::Grade2, InjurySeverityGrade::Grade3]
        .into_iter()
        .filter_map(move |grade| {
            let minimum = catalog.minimum_grade(id);
            let above_minimum = match (minimum, grade) {
                (Some(InjurySeverityGrade::Grade3), InjurySeverityGrade::Grade1 | InjurySeverityGrade::Grade2) => false,
                (Some(InjurySeverityGrade::Grade2), InjurySeverityGrade::Grade1) => false,
                _ => true,
            };
            let weight = grade_weight(code, grade);
            (above_minimum && weight > 0 && catalog.has_recovery_profile(id, grade))
                .then_some((grade, weight))
        })
}

fn grade_weight(code: &str, grade: InjurySeverityGrade) -> u32 {
    let fixed = if code.contains("_GRADE1") || code.contains("_MILD") {
        Some(InjurySeverityGrade::Grade1)
    } else if code.contains("_GRADE2") || code.contains("_MODERATE") {
        Some(InjurySeverityGrade::Grade2)
    } else if code.contains("_GRADE3") || code.contains("_SEVERE") {
        Some(InjurySeverityGrade::Grade3)
    } else {
        None
    };
    if let Some(fixed) = fixed {
        return u32::from(grade == fixed);
    }
    if code.contains("ACL_TEAR") {
        return match grade {
            InjurySeverityGrade::Grade1 => 0,
            InjurySeverityGrade::Grade2 => 55,
            InjurySeverityGrade::Grade3 => 45,
        };
    }
    match grade {
        InjurySeverityGrade::Grade1 => 77,
        InjurySeverityGrade::Grade2 => 20,
        InjurySeverityGrade::Grade3 => 3,
    }
}
