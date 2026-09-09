use crate::error::TacticsResult;
use crate::instructions::axes::{
    Aeriality, Aggression, Compactness, CounterAttackIntensity, CounterPressIntensity,
    DefensiveLineHeight, Directness, FlankBias, Mentality, PassingRange, Physicality,
    PressingIntensity, ScoringPatience, Structure, Tempo, Width,
};
use crate::instructions::transition::PressBlockShape;
use crate::instructions::{TeamInstructions, TeamTacticalProfile};
use crate::persistence::models::instruction_key_code::{parse_instruction_key, InstructionKey};
use crate::persistence::models::rows::play_call_situational_parameter_row::situational_profile_from_pairs;
use crate::persistence::models::rows::tactical_instruction_value_row::TacticalInstructionValueRow;
use crate::persistence::models::rows::team_tactical_profile_situational_parameter_row::TeamTacticalProfileSituationalParameterRow;
use crate::persistence::models::situational_parameter_key_code::parse_situational_parameter_key;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct TeamTacticalProfileRow {
    pub id: String,
    pub team_id: String,
    pub name: String,
    pub is_active: i64,
    pub created_at_unix_seconds: i64,
}

impl TeamTacticalProfileRow {
    pub fn to_domain(
        &self,
        instruction_rows: &[TacticalInstructionValueRow],
        situational_rows: &[TeamTacticalProfileSituationalParameterRow],
    ) -> TacticsResult<TeamTacticalProfile> {
        let id = Uuid::parse_str(&self.id)?;
        let team_id = Uuid::parse_str(&self.team_id)?;
        let is_active = self.is_active != 0;

        let situational_profile = if situational_rows.is_empty() {
            None
        } else {
            let mut pairs = Vec::with_capacity(situational_rows.len());
            for r in situational_rows {
                let key = parse_situational_parameter_key(&r.parameter_key)?;
                pairs.push((key, r.value));
            }
            Some(situational_profile_from_pairs(&pairs))
        };

        let initial_mentality = instruction_rows
            .iter()
            .find_map(|r| {
                if parse_instruction_key(&r.instruction_key).ok()? == InstructionKey::Mentality {
                    Some(Mentality::new_clamped(r.value))
                } else {
                    None
                }
            })
            .unwrap_or_default();

        let mut builder = TeamInstructions::builder(initial_mentality);

        for row in instruction_rows {
            let key = parse_instruction_key(&row.instruction_key)?;
            match key {
                InstructionKey::Mentality => {
                    builder = builder.with_mentality(Mentality::new_clamped(row.value));
                }
                InstructionKey::Tempo => {
                    builder = builder.with_tempo(Tempo::new_clamped(row.value));
                }
                InstructionKey::Width => {
                    builder = builder.with_width(Width::new_clamped(row.value));
                }
                InstructionKey::FlankBias => {
                    builder = builder.with_flank_bias(FlankBias::new_clamped(row.value));
                }
                InstructionKey::Directness => {
                    builder = builder.with_directness(Directness::new_clamped(row.value));
                }
                InstructionKey::Structure => {
                    builder = builder.with_structure(Structure::new_clamped(row.value));
                }
                InstructionKey::PassingRange => {
                    builder = builder.with_passing_range(PassingRange::new_clamped(row.value));
                }
                InstructionKey::Aeriality => {
                    builder = builder.with_aeriality(Aeriality::new_clamped(row.value));
                }
                InstructionKey::Physicality => {
                    builder = builder.with_physicality(Physicality::new_clamped(row.value));
                }
                InstructionKey::ScoringPatience => {
                    builder =
                        builder.with_scoring_patience(ScoringPatience::new_clamped(row.value));
                }
                InstructionKey::DefensiveLineHeight => {
                    builder = builder
                        .with_defensive_line_height(DefensiveLineHeight::new_clamped(row.value));
                }
                InstructionKey::Compactness => {
                    builder = builder.with_compactness(Compactness::new_clamped(row.value));
                }
                InstructionKey::PressingIntensity => {
                    builder =
                        builder.with_pressing_intensity(PressingIntensity::new_clamped(row.value));
                }
                InstructionKey::Aggression => {
                    builder = builder.with_aggression(Aggression::new_clamped(row.value));
                }
                InstructionKey::CounterAttackIntensity => {
                    builder = builder.with_counter_attack_intensity(
                        CounterAttackIntensity::new_clamped(row.value),
                    );
                }
                InstructionKey::CounterPressIntensity => {
                    builder = builder.with_counter_press_intensity(
                        CounterPressIntensity::new_clamped(row.value),
                    );
                }
                InstructionKey::PressBlockShape => {
                    builder =
                        builder.with_press_block_shape(PressBlockShape::new_clamped(row.value));
                }
            }
        }

        let instructions = builder.build();

        Ok(TeamTacticalProfile::new(
            id,
            team_id,
            &self.name,
            instructions,
            situational_profile,
            is_active,
        ))
    }
}