use crate::attributes::profiles::{
    cross_profile, long_launch_profile, self_carry_profile, self_finish_profile,
    short_pass_profile, AttributeProfile,
};
use arlo_domain::ArtrineDecisionKind;
use std::collections::HashMap;
use std::sync::OnceLock;

static DECISION_PROFILES_CACHE: OnceLock<HashMap<ArtrineDecisionKind, AttributeProfile>> =
    OnceLock::new();

fn init_decision_profiles_cache() -> HashMap<ArtrineDecisionKind, AttributeProfile> {
    let mut map = HashMap::with_capacity(5);
    map.insert(ArtrineDecisionKind::SelfCarry, self_carry_profile());
    map.insert(ArtrineDecisionKind::ShortPass, short_pass_profile());
    map.insert(ArtrineDecisionKind::LongLaunch, long_launch_profile());
    map.insert(ArtrineDecisionKind::Cross, cross_profile());
    map.insert(ArtrineDecisionKind::SelfFinish, self_finish_profile());
    map
}

pub fn get_cached_decision_profile(kind: ArtrineDecisionKind) -> &'static AttributeProfile {
    &DECISION_PROFILES_CACHE.get_or_init(init_decision_profiles_cache)[&kind]
}

pub fn get_decision_profile(kind: ArtrineDecisionKind) -> &'static AttributeProfile {
    get_cached_decision_profile(kind)
}
