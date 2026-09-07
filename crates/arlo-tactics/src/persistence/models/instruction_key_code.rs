use crate::error::{TacticsError, TacticsResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstructionKey {
    Mentality,
    Tempo,
    Width,
    FlankBias,
    Directness,
    Structure,
    DefensiveLineHeight,
    Compactness,
    PressingIntensity,
    Aggression,
    CounterAttackIntensity,
    CounterPressIntensity,
}

pub fn parse_instruction_key(code: &str) -> TacticsResult<InstructionKey> {
    match code {
        "mentality" => Ok(InstructionKey::Mentality),
        "tempo" => Ok(InstructionKey::Tempo),
        "width" => Ok(InstructionKey::Width),
        "flank_bias" => Ok(InstructionKey::FlankBias),
        "directness" => Ok(InstructionKey::Directness),
        "structure" => Ok(InstructionKey::Structure),
        "defensive_line_height" => Ok(InstructionKey::DefensiveLineHeight),
        "compactness" => Ok(InstructionKey::Compactness),
        "pressing_intensity" => Ok(InstructionKey::PressingIntensity),
        "aggression" => Ok(InstructionKey::Aggression),
        "counter_attack_intensity" => Ok(InstructionKey::CounterAttackIntensity),
        "counter_press_intensity" => Ok(InstructionKey::CounterPressIntensity),
        _ => Err(TacticsError::InvalidInstructionKey(format!(
            "Invalid instruction key: {code}"
        ))),
    }
}

pub fn instruction_key_to_code(key: InstructionKey) -> &'static str {
    match key {
        InstructionKey::Mentality => "mentality",
        InstructionKey::Tempo => "tempo",
        InstructionKey::Width => "width",
        InstructionKey::FlankBias => "flank_bias",
        InstructionKey::Directness => "directness",
        InstructionKey::Structure => "structure",
        InstructionKey::DefensiveLineHeight => "defensive_line_height",
        InstructionKey::Compactness => "compactness",
        InstructionKey::PressingIntensity => "pressing_intensity",
        InstructionKey::Aggression => "aggression",
        InstructionKey::CounterAttackIntensity => "counter_attack_intensity",
        InstructionKey::CounterPressIntensity => "counter_press_intensity",
    }
}