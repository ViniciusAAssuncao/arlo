use crate::attributes::profiles::profile::{w, AttributeProfile};
use arlo_domain::AttributeKey;

pub fn default_impulse_baseline_profile() -> AttributeProfile {
    AttributeProfile::new(vec![
        w(AttributeKey::Determination, 5.0),
        w(AttributeKey::Composure, 4.5),
        w(AttributeKey::Bravery, 4.0),
        w(AttributeKey::Consistency, 4.0),
        w(AttributeKey::Concentration, 3.5),
        w(AttributeKey::Leadership, 3.0),
        w(AttributeKey::Teamwork, 2.5),
    ])
}
