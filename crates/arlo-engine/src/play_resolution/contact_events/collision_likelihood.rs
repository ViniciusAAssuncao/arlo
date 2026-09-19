use crate::attributes::{PlayerAttributeTable, RefereeAttributeTable};
use crate::physical::PhysicalState;
use crate::possession::PitchState;
use arlo_domain::sport_constants::ATTRIBUTE_MAX;
use arlo_domain::{AttributeKey, PitchZone};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use arlo_tactics::TeamInstructions;
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ContactLikelihoodProfile {
    contact_probability: f64,
    expected_contact_severity: f64,
    foul_probability: f64,
}

impl ContactLikelihoodProfile {
    pub fn new(
        contact_probability: f64,
        expected_contact_severity: f64,
        foul_probability: f64,
    ) -> Self {
        Self {
            contact_probability: contact_probability.clamp(0.0, 1.0),
            expected_contact_severity: expected_contact_severity.clamp(0.0, 1.0),
            foul_probability: foul_probability.clamp(0.0, 1.0),
        }
    }

    pub fn contact_probability(&self) -> f64 {
        self.contact_probability
    }

    pub fn expected_contact_severity(&self) -> f64 {
        self.expected_contact_severity
    }

    pub fn foul_probability(&self) -> f64 {
        self.foul_probability
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ContactEventSamplingResult {
    pub contact_occurred: bool,
    pub foul_occurred: bool,
    pub contact_severity: f64,
}

pub fn evaluate_contact_likelihood(
    carrier_table: &PlayerAttributeTable,
    carrier_fatigue: &PhysicalState,
    _carrier_susceptibility: f64,
    defender_table: &PlayerAttributeTable,
    defender_fatigue: &PhysicalState,
    _defender_susceptibility: f64,
    offense_instructions: &TeamInstructions,
    defense_instructions: &TeamInstructions,
    referee_table: &RefereeAttributeTable,
    pitch_state: &PitchState,
) -> ContactLikelihoodProfile {
    let off_physicality = offense_instructions.in_possession().physicality().value();
    let def_aggression = defense_instructions
        .out_of_possession()
        .aggression()
        .value();
    let def_pressing = defense_instructions
        .out_of_possession()
        .pressing_intensity()
        .value();

    let carrier_bravery = carrier_table.get(AttributeKey::Bravery) / ATTRIBUTE_MAX;
    let def_controlled_agg =
        defender_table.get(AttributeKey::ControlledAggression) / ATTRIBUTE_MAX;
    let def_strength = defender_table.get(AttributeKey::Strength) / ATTRIBUTE_MAX;

    let def_recklessness = (def_strength - def_controlled_agg).max(0.0);

    let carrier_exhaustion = 1.0 - carrier_fatigue.energy();
    let defender_exhaustion = 1.0 - defender_fatigue.energy();
    let mean_exhaustion = (carrier_exhaustion + defender_exhaustion) * 0.5;

    let zone_bonus = match pitch_state.zone() {
        PitchZone::FirstZone => 0.55,
        PitchZone::SecondZone => 0.25,
        _ => 0.0,
    };

    let base_contact_logit = -0.40
        + 1.30 * off_physicality
        + 1.50 * def_aggression
        + 0.90 * def_pressing
        + 0.40 * carrier_bravery
        + 0.50 * def_recklessness
        + zone_bonus;

    let contact_probability = logistic(base_contact_logit).clamp(0.05, 0.95);

    let expected_contact_severity = ((0.20
        + 0.35 * off_physicality
        + 0.35 * def_aggression
        + 0.30 * def_strength)
        * (1.0 + 0.30 * mean_exhaustion))
        .clamp(0.05, 1.0);

    let ref_rigor = referee_table.get(AttributeKey::Rigor) / ATTRIBUTE_MAX;
    let foul_logit = base_contact_logit - 0.80
        + 1.20 * def_recklessness
        + 0.80 * ref_rigor
        + 0.30 * mean_exhaustion;
    let foul_probability = (contact_probability * logistic(foul_logit)).clamp(0.01, 0.80);

    ContactLikelihoodProfile::new(
        contact_probability,
        expected_contact_severity,
        foul_probability,
    )
}

pub fn sample_contact_event<R: Rng + ?Sized>(
    profile: &ContactLikelihoodProfile,
    rng: &mut R,
) -> ContactEventSamplingResult {
    let contact_occurred = Probability::new_clamped(profile.contact_probability()).sample(rng);
    if !contact_occurred {
        return ContactEventSamplingResult {
            contact_occurred: false,
            foul_occurred: false,
            contact_severity: 0.0,
        };
    }

    let foul_occurred = Probability::new_clamped(profile.foul_probability()).sample(rng);

    ContactEventSamplingResult {
        contact_occurred: true,
        foul_occurred,
        contact_severity: profile.expected_contact_severity(),
    }
}
