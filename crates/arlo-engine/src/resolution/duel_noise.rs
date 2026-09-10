use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::extract_effective_attribute_value_with_impulse;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline;
use arlo_domain::sport_constants::{ATTRIBUTE_SATURATION_THRESHOLD, BASE_NOISE_SCALE};
use arlo_domain::{AttributeKey, Player};
pub use arlo_math::stats::SkewNormalParams;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn player_noise_distribution_with_impulse(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    impulse_state: &ImpulseState,
) -> SkewNormalParams {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let baseline = calculate_player_impulse_baseline(player, attribute_keys);
    let consistency = extract_effective_attribute_value_with_impulse(
        &table,
        AttributeKey::Consistency,
        physical_state,
        impulse_state,
        baseline,
    );
    let technique = extract_effective_attribute_value_with_impulse(
        &table,
        AttributeKey::Technique,
        physical_state,
        impulse_state,
        baseline,
    );
    let flair = extract_effective_attribute_value_with_impulse(
        &table,
        AttributeKey::Flair,
        physical_state,
        impulse_state,
        baseline,
    );
    let composure = extract_effective_attribute_value_with_impulse(
        &table,
        AttributeKey::Composure,
        physical_state,
        impulse_state,
        baseline,
    );

    let fatigue_noise_scale = 1.0
        + (1.0 - physical_state.energy()) * 0.60
        + (1.0 - physical_state.w_prime_balance()) * 0.40;

    let depression_from_impulse = if impulse_state.accumulator() < baseline {
        let deficit = baseline - impulse_state.accumulator();
        (2.0 / (1.0 + (-0.06 * deficit).exp()) - 1.0).clamp(0.0, 0.60)
    } else {
        0.0
    };

    let scale = BASE_NOISE_SCALE
        * (1.0 + (20.0 - consistency).max(0.0) / ATTRIBUTE_SATURATION_THRESHOLD)
        * fatigue_noise_scale
        * (1.0 + depression_from_impulse);
    let shape = ((technique + flair) / 2.0 - composure) / ATTRIBUTE_SATURATION_THRESHOLD;
    let location = 0.0;

    SkewNormalParams::new(location, scale, shape)
}

pub fn player_noise_distribution(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
) -> SkewNormalParams {
    let baseline = calculate_player_impulse_baseline(player, attribute_keys);
    player_noise_distribution_with_impulse(
        player,
        attribute_keys,
        physical_state,
        &ImpulseState::from_baseline(baseline),
    )
}

pub fn sample_player_noise_with_impulse<R: Rng + ?Sized>(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    impulse_state: &ImpulseState,
    rng: &mut R,
) -> f64 {
    let params = player_noise_distribution_with_impulse(
        player,
        attribute_keys,
        physical_state,
        impulse_state,
    );
    params.sample(rng)
}

pub fn sample_player_noise<R: Rng + ?Sized>(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    rng: &mut R,
) -> f64 {
    let params = player_noise_distribution(player, attribute_keys, physical_state);
    params.sample(rng)
}