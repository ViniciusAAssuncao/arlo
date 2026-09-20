use crate::attributes::PlayerAttributeTable;
use arlo_domain::AttributeKey;

pub fn calculate_artrine_drive_skill(table: &PlayerAttributeTable) -> f64 {
    let drive_tech = table.get(AttributeKey::DriveTechnique);
    let arlo_control = table.get(AttributeKey::ArloControl);
    let balance = table.get(AttributeKey::Balance);
    let decisions = table.get(AttributeKey::Decisions);
    let bravery = table.get(AttributeKey::Bravery);
    let acceleration = table.get(AttributeKey::Acceleration);

    (drive_tech * 0.35
        + arlo_control * 0.25
        + balance * 0.15
        + decisions * 0.10
        + bravery * 0.08
        + acceleration * 0.07)
        .clamp(1.0, 20.0)
}
