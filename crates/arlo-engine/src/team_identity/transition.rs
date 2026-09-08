use crate::team_identity::geometry::depth_from_bipolar;
use arlo_math::stats::UnipolarScalar;
use arlo_tactics::{CounterAttackIntensity, CounterPressIntensity, ReleaseTempo};

pub fn counter_attack_depth_bias(
    counter_attack_intensity: CounterAttackIntensity,
    pitch_length_m: f64,
    individual_release_tempo: Option<ReleaseTempo>,
) -> f64 {
    let team_bipolar = 2.0 * counter_attack_intensity.value() - 1.0;
    let bipolar_val = match individual_release_tempo {
        Some(rt) => (team_bipolar + rt.value()) * 0.5,
        None => team_bipolar,
    };
    let depth = depth_from_bipolar(bipolar_val, pitch_length_m, true);
    depth * 0.08
}

pub fn counter_press_engagement_bias(
    counter_press_intensity: CounterPressIntensity,
    regroup_discipline: UnipolarScalar,
) -> f64 {
    let bipolar_val = 2.0 * counter_press_intensity.value() - 1.0;
    let depth_factor = depth_from_bipolar(bipolar_val, 1.0, true);
    depth_factor * (1.0 - regroup_discipline.value())
}
