use arlo_domain::{DefensiveApproach, OffensiveApproach};
use arlo_math::stats::cosine_similarity;
use arlo_tactics::{InPossessionInstructions, OutOfPossessionInstructions, TransitionInstructions};

pub fn scalar_alignment(preferred: f64, actual: f64, range: f64) -> f64 {
    if range <= 0.0 {
        return 1.0;
    }
    (1.0 - (preferred - actual).abs() / range).clamp(0.0, 1.0)
}

pub fn offensive_approach_alignment(
    approach: OffensiveApproach,
    instructions: &InPossessionInstructions,
) -> f64 {
    let target = match approach {
        OffensiveApproach::Direct => [0.90, 0.10, 0.85, 0.15, 0.50, 0.50],
        OffensiveApproach::Positional => [0.20, 0.80, 0.40, 0.60, 0.85, 0.15],
        OffensiveApproach::Functional => [0.40, 0.60, 0.70, 0.30, 0.20, 0.80],
        OffensiveApproach::Balanced => [0.50, 0.50, 0.50, 0.50, 0.50, 0.50],
    };

    let d = instructions.directness().value();
    let t = instructions.tempo().value();
    let s = instructions.structure().value();
    let actual = [d, 1.0 - d, t, 1.0 - t, s, 1.0 - s];

    cosine_similarity(&target, &actual).clamp(0.0, 1.0)
}

pub fn defensive_approach_alignment(
    approach: DefensiveApproach,
    instructions: &OutOfPossessionInstructions,
) -> f64 {
    let target = match approach {
        DefensiveApproach::HighPress => [0.90, 0.10, 0.85, 0.15],
        DefensiveApproach::MidBlock => [0.50, 0.50, 0.50, 0.50],
        DefensiveApproach::DeepLowBlock => [0.20, 0.80, 0.15, 0.85],
    };

    let p = instructions.pressing_intensity().value();
    let l = instructions.defensive_line_height().value();
    let actual = [p, 1.0 - p, l, 1.0 - l];

    cosine_similarity(&target, &actual).clamp(0.0, 1.0)
}

pub fn passing_range_alignment(preferred: f64, instructions: &InPossessionInstructions) -> f64 {
    scalar_alignment(preferred, instructions.passing_range().value(), 2.0)
}

pub fn aeriality_alignment(preferred: f64, instructions: &InPossessionInstructions) -> f64 {
    scalar_alignment(preferred, instructions.aeriality().value(), 2.0)
}

pub fn structure_alignment(preferred: f64, instructions: &InPossessionInstructions) -> f64 {
    scalar_alignment(preferred, instructions.structure().value(), 2.0)
}

pub fn physicality_alignment(preferred: f64, instructions: &InPossessionInstructions) -> f64 {
    scalar_alignment(preferred, instructions.physicality().value(), 1.0)
}

pub fn transition_pace_alignment(preferred: f64, instructions: &TransitionInstructions) -> f64 {
    scalar_alignment(
        preferred,
        instructions.counter_attack_intensity().value(),
        1.0,
    )
}

pub fn press_block_shape_alignment(preferred: f64, instructions: &TransitionInstructions) -> f64 {
    scalar_alignment(preferred, instructions.press_block_shape().value(), 1.0)
}