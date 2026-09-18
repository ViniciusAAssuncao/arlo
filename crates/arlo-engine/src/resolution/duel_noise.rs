use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::{extract_effective_attribute_value, DegradationContext};
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
    let activation = pressure_urgency_activation(pressure);
    if activation <= 0.0 {
        return 0.0;
    }
    let consistency = extract_effective_attribute_value(table, AttributeKey::Consistency, deg_ctx);
    let inconsistency = ((20.0 - consistency).max(0.0) / 20.0).clamp(0.0, 1.0);
    0.004 * inconsistency.powi(2) * activation.powi(2)
}

pub fn player_consistency_noise_scale(
    table: &PlayerAttributeTable,
    deg_ctx: &DegradationContext<'_>,
) -> f64 {
    let consistency = extract_effective_attribute_value(table, AttributeKey::Consistency, deg_ctx);
    let inconsistency = ((20.0 - consistency).max(0.0) / 20.0).clamp(0.0, 1.0);
    (inconsistency.powi(2) * 0.002).clamp(0.0, 0.005)
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