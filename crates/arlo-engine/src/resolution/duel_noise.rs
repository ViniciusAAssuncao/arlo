use crate::attributes::PlayerAttributeTable;
use crate::caching::impulse_baseline_profile;
use crate::physical::systems::degradation::extract_effective_attribute_value_with_impulse;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline_from_table_with_profile;
use arlo_domain::sport_constants::{ATTRIBUTE_SATURATION_THRESHOLD, BASE_NOISE_SCALE};
use arlo_domain::{AttributeKey, Player};
pub use arlo_math::stats::SkewNormalParams;
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn player_noise_distribution_from_table_with_impulse(
    _player: &Player,
    table: &PlayerAttributeTable,
    physical_state: &PhysicalState,
    impulse_state: &ImpulseState,
    baseline: f64,
) -> SkewNormalParams {
    let consistency = extract_effective_attribute_value_with_impulse(
        table,
        AttributeKey::Consistency,
        physical_state,
        impulse_state,
        baseline,
    );
    let technique = extract_effective_attribute_value_with_impulse(
        table,
        AttributeKey::Technique,
        physical_state,
        impulse_state,
        baseline,
    );
    let flair = extract_effective_attribute_value_with_impulse(
        table,
        AttributeKey::Flair,
        physical_state,
        impulse_state,
        baseline,
    );
    let composure = extract_effective_attribute_value_with_impulse(
        table,
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

pub fn sample_player_noise_from_table_with_impulse<R: Rng + ?Sized>(
    player: &Player,
    table: &PlayerAttributeTable,
    physical_state: &PhysicalState,
    impulse_state: &ImpulseState,
    baseline: f64,
    rng: &mut R,
) -> f64 {
    let params = player_noise_distribution_from_table_with_impulse(
        player,
        table,
        physical_state,
        impulse_state,
        baseline,
    );
    params.sample(rng)
}

pub fn sample_player_noise_from_table_with_baseline<R: Rng + ?Sized>(
    player: &Player,
    table: &PlayerAttributeTable,
    physical_state: &PhysicalState,
    rng: &mut R,
) -> f64 {
    let profile = impulse_baseline_profile();
    let baseline = calculate_player_impulse_baseline_from_table_with_profile(table, profile);
    let impulse_state = ImpulseState::from_baseline(baseline);
    sample_player_noise_from_table_with_impulse(
        player,
        table,
        physical_state,
        &impulse_state,
        baseline,
        rng,
    )
}

pub fn sample_player_noise_with_impulse<R: Rng + ?Sized>(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    impulse_state: &ImpulseState,
    rng: &mut R,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let profile = impulse_baseline_profile();
    let baseline = calculate_player_impulse_baseline_from_table_with_profile(&table, profile);
    sample_player_noise_from_table_with_impulse(
        player,
        &table,
        physical_state,
        impulse_state,
        baseline,
        rng,
    )
}

pub fn sample_player_noise<R: Rng + ?Sized>(
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    rng: &mut R,
) -> f64 {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    sample_player_noise_from_table_with_baseline(player, &table, physical_state, rng)
}
