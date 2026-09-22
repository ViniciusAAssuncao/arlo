use crate::artrine::execution::drive_profile::DriveAwardProfile;
use crate::artrine::execution::drive_skill::calculate_artrine_drive_skill;
use crate::attributes::PlayerAttributeTable;
use arlo_domain::sport_constants::ARTRO_ROW_SPACING_MIRIM;
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
        || mirins_advanced <= 0.0
    {
        return 0;
    }

    let rows_crossed = (mirins_advanced / ARTRO_ROW_SPACING_MIRIM).floor() as u32;
    if rows_crossed == 0 {
        return 0;
    }

    let skill = calculate_artrine_drive_skill(table);
    let norm_skill = (skill - 10.0) / 10.0;

    let logit = profile.base_logit()
        + profile.advantage_scale() * net_advantage
        + profile.skill_scale() * norm_skill;
    
    let prob = logistic(logit).clamp(profile.min_prob(), profile.max_prob());
    let probability = Probability::new_clamped(prob);

    let mut drives = 0;
    for _ in 0..rows_crossed {
        if probability.sample(rng) {
            drives += 1;
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
