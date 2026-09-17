use crate::attributes::PlayerAttributeTable;
use crate::play_resolution::field_context::PitchState;
use arlo_domain::{ArtroPlacement, AttributeKey, PitchZone};
use arlo_math::stats::contrast::logistic;
use arlo_tactics::TeamInstructions;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TeamSpaceRating {
    space_index: f64,
    expected_free_mirim: f64,
    pressure_intensity: f64,
    lane_clearance: f64,
    flank_openness: f64,
}

impl TeamSpaceRating {
    pub fn new(
        space_index: f64,
        expected_free_mirim: f64,
        pressure_intensity: f64,
        lane_clearance: f64,
        flank_openness: f64,
    ) -> Self {
        Self {
            space_index: space_index.clamp(0.0, 1.0),
            expected_free_mirim: expected_free_mirim.max(0.0),
            pressure_intensity: pressure_intensity.clamp(0.0, 1.0),
            lane_clearance: lane_clearance.clamp(0.0, 1.0),
            flank_openness: flank_openness.clamp(0.0, 1.0),
        }
    }

    pub fn space_index(&self) -> f64 {
        self.space_index
    }

    pub fn expected_free_mirim(&self) -> f64 {
        self.expected_free_mirim
    }

    pub fn pressure_intensity(&self) -> f64 {
        self.pressure_intensity
    }

    pub fn lane_clearance(&self) -> f64 {
        self.lane_clearance
    }

    pub fn flank_openness(&self) -> f64 {
        self.flank_openness
    }
}

impl Default for TeamSpaceRating {
    fn default() -> Self {
        Self {
            space_index: 0.50,
            expected_free_mirim: 6.0,
            pressure_intensity: 0.50,
            lane_clearance: 0.50,
            flank_openness: 0.50,
        }
    }
}

pub fn calculate_team_space_rating(
    carrier_table: &PlayerAttributeTable,
    offense_tables: &[&PlayerAttributeTable],
    defense_tables: &[&PlayerAttributeTable],
    offense_instructions: &TeamInstructions,
    defense_instructions: &TeamInstructions,
    pitch_state: &PitchState,
    variance: f64,
) -> TeamSpaceRating {
    let carrier_flair = carrier_table.get(AttributeKey::Flair) / 20.0;
    let carrier_vision = carrier_table.get(AttributeKey::Vision) / 20.0;
    let carrier_agility = carrier_table.get(AttributeKey::Agility) / 20.0;
    let carrier_pace = carrier_table.get(AttributeKey::Pace) / 20.0;
    let carrier_tech = carrier_table.get(AttributeKey::Technique) / 20.0;

    let carrier_creation = carrier_flair * 0.25
        + carrier_vision * 0.20
        + carrier_agility * 0.20
        + carrier_pace * 0.20
        + carrier_tech * 0.15;

    let off_team_creation = if offense_tables.is_empty() {
        carrier_creation
    } else {
        let avg_flair: f64 = offense_tables
            .iter()
            .map(|t| t.get(AttributeKey::Flair))
            .sum::<f64>()
            / (offense_tables.len() as f64 * 20.0);
        let avg_vis: f64 = offense_tables
            .iter()
            .map(|t| t.get(AttributeKey::Vision))
            .sum::<f64>()
            / (offense_tables.len() as f64 * 20.0);
        let avg_pass: f64 = offense_tables
            .iter()
            .map(|t| t.get(AttributeKey::Passing))
            .sum::<f64>()
            / (offense_tables.len() as f64 * 20.0);
        avg_flair * 0.35 + avg_vis * 0.35 + avg_pass * 0.30
    };

    let tempo_val = offense_instructions.in_possession().tempo().value();
    let width_val = offense_instructions.in_possession().width().value();
    let structure_val = offense_instructions.in_possession().structure().value();
    let directness_val = offense_instructions.in_possession().directness().value();

    let off_tactics = tempo_val * 0.30
        + width_val * 0.25
        + (1.0 - structure_val) * 0.25
        + directness_val * 0.20;
    let off_total_power = (carrier_creation * 0.40
        + off_team_creation * 0.30
        + off_tactics * 0.30)
        .clamp(0.0, 2.0);

    let def_team_containment = if defense_tables.is_empty() {
        0.50
    } else {
        let avg_pos: f64 = defense_tables
            .iter()
            .map(|t| t.get(AttributeKey::Positioning))
            .sum::<f64>()
            / (defense_tables.len() as f64 * 20.0);
        let avg_ant: f64 = defense_tables
            .iter()
            .map(|t| t.get(AttributeKey::Anticipation))
            .sum::<f64>()
            / (defense_tables.len() as f64 * 20.0);
        let avg_cont: f64 = defense_tables
            .iter()
            .map(|t| t.get(AttributeKey::DefensiveContainment))
            .sum::<f64>()
            / (defense_tables.len() as f64 * 20.0);
        let avg_pace: f64 = defense_tables
            .iter()
            .map(|t| t.get(AttributeKey::Pace))
            .sum::<f64>()
            / (defense_tables.len() as f64 * 20.0);
        let avg_str: f64 = defense_tables
            .iter()
            .map(|t| t.get(AttributeKey::Strength))
            .sum::<f64>()
            / (defense_tables.len() as f64 * 20.0);
        avg_pos * 0.30 + avg_ant * 0.25 + avg_cont * 0.20 + avg_pace * 0.15 + avg_str * 0.10
    };

    let compactness_val = defense_instructions
        .out_of_possession()
        .compactness()
        .value();
    let pressing_val = defense_instructions
        .out_of_possession()
        .pressing_intensity()
        .value();
    let dline_val = defense_instructions
        .out_of_possession()
        .defensive_line_height()
        .value();
    let aggression_val = defense_instructions
        .out_of_possession()
        .aggression()
        .value();

    let def_tactics = compactness_val * 0.35
        + pressing_val * 0.30
        + dline_val * 0.20
        + aggression_val * 0.15;
    let def_total_power = (def_team_containment * 0.60 + def_tactics * 0.40).clamp(0.0, 2.0);

    let zone_mod = match pitch_state.zone() {
        PitchZone::FirstZone => -0.45,
        PitchZone::SecondZone => -0.20,
        _ => 0.0,
    };

    let channel_mod = match pitch_state.channel() {
        ArtroPlacement::LeftLateral | ArtroPlacement::RightLateral => 0.15 + width_val * 0.20,
        ArtroPlacement::Central => -0.10 - compactness_val * 0.15,
    };

    let raw_diff = (off_total_power - def_total_power) + zone_mod + channel_mod + variance;
    let space_index = logistic(raw_diff * 3.2);

    let base_free = 12.0 * space_index * (1.0 - 0.25 * pitch_state.normalized_proximity());
    let channel_mult = match pitch_state.channel() {
        ArtroPlacement::Central => 0.85,
        _ => 1.15,
    };
    let expected_free_mirim = (base_free * channel_mult).clamp(0.5, 25.0);

    let pressure_intensity = (1.0 - space_index).clamp(0.0, 1.0);
    let lane_clearance =
        (0.35 + 0.65 * space_index - 0.10 * pitch_state.normalized_proximity()).clamp(0.10, 1.0);
    let flank_openness = (space_index * (1.0 + width_val * 0.40)).clamp(0.0, 1.0);

    TeamSpaceRating::new(
        space_index,
        expected_free_mirim,
        pressure_intensity,
        lane_clearance,
        flank_openness,
    )
}
