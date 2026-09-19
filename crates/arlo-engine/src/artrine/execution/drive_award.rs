use crate::artrine::execution::drive_profile::DriveAwardProfile;
use crate::artrine::execution::drive_skill::calculate_artrine_drive_skill;
use crate::attributes::PlayerAttributeTable;
use arlo_domain::ArtrineDecisionKind;
use arlo_math::stats::contrast::logistic;
use arlo_math::Probability;
use rand::Rng;

pub fn award_drives_with_profile<R: Rng + ?Sized>(
    is_true_artrine: bool,
    decision_kind: ArtrineDecisionKind,
    attacker_won: bool,
    net_advantage: f64,
    table: &PlayerAttributeTable,
    mirins_advanced: f64,
    profile: &DriveAwardProfile,
    rng: &mut R,
) -> u32 {
    if !is_true_artrine
        || decision_kind != ArtrineDecisionKind::SelfCarry
        || !attacker_won
        || mirins_advanced < profile.min_advance_p1()
    {
        return 0;
    }

    let skill = calculate_artrine_drive_skill(table);
    let norm_skill = (skill - 10.0) / 10.0;

    let p1_logit = profile.p1_base_logit()
        + profile.p1_advantage_scale() * net_advantage
        + profile.p1_skill_scale() * norm_skill;
    let p1 = logistic(p1_logit).clamp(profile.p1_min_prob(), profile.p1_max_prob());
    if !Probability::new_clamped(p1).sample(rng) {
        return 0;
    }

    let mut drives = 1;

    if mirins_advanced >= profile.min_advance_p2() {
        let p2_logit = profile.p2_base_logit()
            + profile.p2_advantage_scale() * net_advantage
            + profile.p2_skill_scale() * norm_skill;
        let p2 = logistic(p2_logit).clamp(profile.p2_min_prob(), profile.p2_max_prob());
        if Probability::new_clamped(p2).sample(rng) {
            drives += 1;

            if mirins_advanced >= profile.min_advance_p3() {
                let p3_logit = profile.p3_base_logit()
                    + profile.p3_advantage_scale() * net_advantage
                    + profile.p3_skill_scale() * norm_skill;
                let p3 = logistic(p3_logit).clamp(profile.p3_min_prob(), profile.p3_max_prob());
                if Probability::new_clamped(p3).sample(rng) {
                    drives += 1;
                }
            }
        }
    }

    drives
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
    award_drives_with_profile(
        is_true_artrine,
        decision_kind,
        attacker_won,
        net_advantage,
        table,
        mirins_advanced,
        &DriveAwardProfile::default(),
        rng,
    )
}
