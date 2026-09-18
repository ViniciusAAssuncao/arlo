use crate::attributes::PlayerAttributeTable;
use arlo_domain::{ArtrineDecisionKind, AttributeKey};
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;

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

pub fn award_drives<R: Rng + ?Sized>(
    is_true_artrine: bool,
    decision_kind: ArtrineDecisionKind,
    attacker_won: bool,
    net_advantage: f64,
    table: &PlayerAttributeTable,
    mirins_advanced: f64,
    rng: &mut R,
) -> u32 {
    if !is_true_artrine
        || decision_kind != ArtrineDecisionKind::SelfCarry
        || !attacker_won
        || mirins_advanced < 1.0
    {
        return 0;
    }

    let skill = calculate_artrine_drive_skill(table);
    let norm_skill = (skill - 10.0) / 10.0;

    let p1_logit = -0.15 + 0.32 * net_advantage + 0.45 * norm_skill;
    let p1 = logistic(p1_logit).clamp(0.05, 0.95);
    if !Probability::new_clamped(p1).sample(rng) {
        return 0;
    }

    let mut drives = 1;

    if mirins_advanced >= 4.5 {
        let p2_logit = -1.75 + 0.28 * net_advantage + 0.55 * norm_skill;
        let p2 = logistic(p2_logit).clamp(0.01, 0.85);
        if Probability::new_clamped(p2).sample(rng) {
            drives += 1;

            if mirins_advanced >= 8.5 {
                let p3_logit = -2.70 + 0.25 * net_advantage + 0.65 * norm_skill;
                let p3 = logistic(p3_logit).clamp(0.005, 0.70);
                if Probability::new_clamped(p3).sample(rng) {
                    drives += 1;
                }
            }
        }
    }

    drives
}
