use arlo_domain::{DefensiveApproach, OffensiveApproach};
use arlo_tactics::{InPossessionInstructions, OutOfPossessionInstructions};

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let norm_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        0.0
    } else {
        (dot / (norm_a * norm_b)).clamp(0.0, 1.0)
    }
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

    cosine_similarity(&target, &actual)
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

    cosine_similarity(&target, &actual)
}