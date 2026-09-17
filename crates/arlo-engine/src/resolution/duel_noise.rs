use crate::attributes::PlayerAttributeTable;
use crate::caching::impulse_baseline_profile;
use crate::physical::systems::degradation::extract_effective_attribute_value_with_impulse;
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::calculate_player_impulse_baseline_from_table_with_profile;
use arlo_domain::{AttributeKey, Player};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SkewNormalParams {
    scale: f64,
}

impl SkewNormalParams {
    pub fn new(_location: f64, scale: f64, _shape: f64) -> Self {
        Self { scale }
    }

    pub fn scale(&self) -> f64 {
        self.scale
    }

    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        if self.scale <= 0.0001 {
            0.0
        } else {
            rng.gen_range(-self.scale..=self.scale)
        }
    }
}

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
    let scale = ((20.0 - consistency).max(0.0) * 0.015).clamp(0.0, 0.30);
    SkewNormalParams::new(0.0, scale, 0.0)
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
