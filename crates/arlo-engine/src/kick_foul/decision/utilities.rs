use crate::attributes::PlayerAttributeTable;
use crate::resolution::duel_profiles::offense_duels::{
    cross_distribution_profile, field_goal_profile, long_distribution_profile,
    short_distribution_profile,
};
use crate::resolution::duel_profiles::DuelProfile;
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::{KickFoulDecisionKind, KickFoulScoringTier};
use smallvec::{smallvec, SmallVec};

fn calculate_table_profile_rating(table: &PlayerAttributeTable, profile: &DuelProfile) -> f64 {
    let mut total_weight = 0.0;
    let mut accumulated = 0.0;
    for w in profile.weights() {
        if w.weight > 0.0 {
            accumulated += table.get(w.key) * w.weight;
            total_weight += w.weight;
        }
    }
    if total_weight > 0.0 {
        accumulated / total_weight
    } else {
        0.0
    }
}

pub fn evaluate_kick_foul_decision_utilities(
    kicker_table: &PlayerAttributeTable,
    tier: KickFoulScoringTier,
    game_state_pressure: &GameStatePressure,
) -> SmallVec<[(KickFoulDecisionKind, f64); 4]> {
    let shoot_rating = calculate_table_profile_rating(kicker_table, &field_goal_profile());
    let shoot_bias =
        game_state_pressure.bias_for_kick_foul_decision(KickFoulDecisionKind::Shoot, tier);
    let shoot_utility = shoot_rating * shoot_bias;

    let cross_rating = calculate_table_profile_rating(kicker_table, &cross_distribution_profile());
    let cross_bias =
        game_state_pressure.bias_for_kick_foul_decision(KickFoulDecisionKind::Cross, tier);
    let cross_utility = cross_rating * cross_bias;

    let short_pass_rating =
        calculate_table_profile_rating(kicker_table, &short_distribution_profile());
    let short_pass_bias =
        game_state_pressure.bias_for_kick_foul_decision(KickFoulDecisionKind::ShortPass, tier);
    let short_pass_utility = short_pass_rating * short_pass_bias;

    let long_launch_rating =
        calculate_table_profile_rating(kicker_table, &long_distribution_profile());
    let long_launch_bias =
        game_state_pressure.bias_for_kick_foul_decision(KickFoulDecisionKind::LongLaunch, tier);
    let long_launch_utility = long_launch_rating * long_launch_bias;

    smallvec![
        (KickFoulDecisionKind::Shoot, shoot_utility),
        (KickFoulDecisionKind::Cross, cross_utility),
        (KickFoulDecisionKind::ShortPass, short_pass_utility),
        (KickFoulDecisionKind::LongLaunch, long_launch_utility),
    ]
}