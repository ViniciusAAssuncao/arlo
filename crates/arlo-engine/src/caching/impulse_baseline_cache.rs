use crate::attributes::profiles::default_impulse_baseline_profile;
use crate::psychology::systems::baseline::ImpulseBaselineProfile;
use std::sync::OnceLock;

static IMPULSE_BASELINE_PROFILE_CACHE: OnceLock<ImpulseBaselineProfile> = OnceLock::new();

pub fn impulse_baseline_profile() -> &'static ImpulseBaselineProfile {
    IMPULSE_BASELINE_PROFILE_CACHE.get_or_init(default_impulse_baseline_profile)
}

pub fn get_cached_impulse_baseline_profile() -> &'static ImpulseBaselineProfile {
    impulse_baseline_profile()
}
