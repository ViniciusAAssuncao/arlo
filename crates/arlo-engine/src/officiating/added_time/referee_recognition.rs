use crate::attributes::RefereeAttributeTable;
use arlo_domain::sport_constants::{
    ADDED_TIME_AUTHORITY_RECOGNITION_SCALE, ADDED_TIME_BASE_RECOGNITION_SCALE,
    ADDED_TIME_RIGOR_RECOGNITION_SCALE, ATTRIBUTE_MAX, ATTRIBUTE_MIN,
};
use arlo_domain::AttributeKey;

pub fn referee_recognition_scale(referee_table: &RefereeAttributeTable) -> f64 {
    let rigor_raw = referee_table
        .get(AttributeKey::Rigor)
        .clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX);
    let rigor_norm = (rigor_raw - ATTRIBUTE_MIN) / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);

    let authority_raw = referee_table
        .get(AttributeKey::Authority)
        .clamp(ATTRIBUTE_MIN, ATTRIBUTE_MAX);
    let authority_norm = (authority_raw - ATTRIBUTE_MIN) / (ATTRIBUTE_MAX - ATTRIBUTE_MIN);

    ADDED_TIME_BASE_RECOGNITION_SCALE
        + (rigor_norm * ADDED_TIME_RIGOR_RECOGNITION_SCALE)
        + (authority_norm * ADDED_TIME_AUTHORITY_RECOGNITION_SCALE)
}
