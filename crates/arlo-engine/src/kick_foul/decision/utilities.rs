use crate::attributes::PlayerAttributeTable;
use crate::resolution::duel_profiles::offense_duels::{
    cross_distribution_profile, field_goal_profile, long_distribution_profile,
    short_distribution_profile,
};
use crate::world_state::context_analyzer::GameStatePressure;
use arlo_domain::{KickFoulDecisionKind, KickFoulScoringTier};
use smallvec::{smallvec, SmallVec};

pub fn evaluate_kick_foul_decision_utilities(
    kicker_table: &PlayerAttributeTable,
    tier: KickFoulScoringTier,
    game_state_pressure: &GameStatePressure,
    lateral_ratio: f64,
) -> SmallVec<[(KickFoulDecisionKind, f64); 4]> {
    let shoot_rating = field_goal_profile().rate(kicker_table);
    let shoot_bias =
        game_state_pressure.bias_for_kick_foul_decision(KickFoulDecisionKind::Shoot, tier, lateral_ratio);
    let shoot_utility = shoot_rating * shoot_bias;

    let cross_rating = cross_distribution_profile().rate(kicker_table);
    let cross_bias =
        game_state_pressure.bias_for_kick_foul_decision(KickFoulDecisionKind::Cross, tier, lateral_ratio);
    let cross_utility = cross_rating * cross_bias;

    let short_pass_rating = short_distribution_profile().rate(kicker_table);
    let short_pass_bias =
        game_state_pressure.bias_for_kick_foul_decision(KickFoulDecisionKind::ShortPass, tier, lateral_ratio);
    let short_pass_utility = short_pass_rating * short_pass_bias;

    let long_launch_rating = long_distribution_profile().rate(kicker_table);
    let long_launch_bias =
        game_state_pressure.bias_for_kick_foul_decision(KickFoulDecisionKind::LongLaunch, tier, lateral_ratio);
    let long_launch_utility = long_launch_rating * long_launch_bias;

    smallvec![
        (KickFoulDecisionKind::Shoot, shoot_utility),
        (KickFoulDecisionKind::Cross, cross_utility),
        (KickFoulDecisionKind::ShortPass, short_pass_utility),
        (KickFoulDecisionKind::LongLaunch, long_launch_utility),
    ]
}