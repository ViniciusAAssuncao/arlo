use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::{extract_effective_attribute_value, DegradationContext};
use arlo_domain::AttributeKey;
use rand::Rng;

pub fn player_consistency_noise_scale(
    table: &PlayerAttributeTable,
    deg_ctx: &DegradationContext<'_>,
) -> f64 {
    let consistency = extract_effective_attribute_value(table, AttributeKey::Consistency, deg_ctx);
    ((20.0 - consistency).max(0.0) * 0.0035).clamp(0.0, 0.07)
}

pub fn sample_player_noise<R: Rng + ?Sized>(
    table: &PlayerAttributeTable,
    deg_ctx: &DegradationContext<'_>,
    rng: &mut R,
) -> f64 {
    let scale = player_consistency_noise_scale(table, deg_ctx);
    if scale <= 1e-6 {
        0.0
    } else {
        rng.gen_range(-scale..=scale)
    }
}