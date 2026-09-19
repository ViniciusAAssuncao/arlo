use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::{
    calculate_physical_exhaustion, extract_effective_attribute_value, DegradationContext,
};
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::AttributeKey;
use arlo_math::stats::sample_gaussian_noise;
use rand::Rng;

pub fn pressure_urgency_activation(pressure: Option<&GameStatePressure>) -> f64 {
    match pressure {
        Some(p) => {
            let urgency = p.urgency_index();
            if urgency < 1.0 {
                0.0
            } else {
                ((urgency - 1.0) / 4.0).clamp(0.0, 1.0)
            }
        }
        None => 0.0,
    }
}

pub fn player_consistency_noise_std_dev(
    table: &PlayerAttributeTable,
    deg_ctx: &DegradationContext<'_>,
    pressure: Option<&GameStatePressure>,
) -> f64 {
    let consistency = extract_effective_attribute_value(table, AttributeKey::Consistency, deg_ctx);
    let norm_consistency = (consistency.clamp(0.0, 20.0)) / 20.0;
    let inconsistency = (1.0 - norm_consistency).clamp(0.0, 1.0);

    let base_inconsistency = inconsistency.powi(3);

    let exhaustion = calculate_physical_exhaustion(deg_ctx.physical_state()).clamp(0.0, 1.0);
    let pressure_factor = pressure_urgency_activation(pressure);
    let stress_multiplier = 1.0 + 1.5 * exhaustion + 1.5 * pressure_factor;

    (0.035 * base_inconsistency * stress_multiplier).clamp(0.0, 0.08)
}

pub fn player_consistency_noise_scale(
    table: &PlayerAttributeTable,
    deg_ctx: &DegradationContext<'_>,
) -> f64 {
    let consistency = extract_effective_attribute_value(table, AttributeKey::Consistency, deg_ctx);
    let norm_consistency = (consistency.clamp(0.0, 20.0)) / 20.0;
    let inconsistency = (1.0 - norm_consistency).clamp(0.0, 1.0);
    let exhaustion = calculate_physical_exhaustion(deg_ctx.physical_state()).clamp(0.0, 1.0);

    (inconsistency.powi(2) * (0.001 + 0.003 * exhaustion)).clamp(0.0, 0.01)
}

pub fn sample_player_noise_with_pressure<R: Rng + ?Sized>(
    table: &PlayerAttributeTable,
    deg_ctx: &DegradationContext<'_>,
    pressure: Option<&GameStatePressure>,
    rng: &mut R,
) -> f64 {
    let std_dev = player_consistency_noise_std_dev(table, deg_ctx, pressure);
    if std_dev <= 1e-6 {
        0.0
    } else {
        sample_gaussian_noise(std_dev, rng)
    }
}

pub fn sample_player_noise<R: Rng + ?Sized>(
    table: &PlayerAttributeTable,
    deg_ctx: &DegradationContext<'_>,
    rng: &mut R,
) -> f64 {
    sample_player_noise_with_pressure(table, deg_ctx, None, rng)
}