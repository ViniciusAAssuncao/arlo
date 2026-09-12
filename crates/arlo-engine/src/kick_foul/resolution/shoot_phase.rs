use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::kick_foul::resolution::tier_opportunity::evaluate_kick_foul_scoring_opportunity;
use crate::match_decision::scoring::{
    duel_kind_for_opportunity, resolve_scoring_attempt, ScoringAttemptRequest, ScoringDecision,
};
use crate::resolution::duel_profiles::offense_duels::{field_goal_profile, finishing_attempt_profile};
use crate::resolution::{AttributedDuelOutcome, DuelContext};
use arlo_domain::{AttributeKey, KickFoulScoringTier, Player};
use rand::Rng;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_kick_foul_shot<R: Rng + ?Sized>(
    kicker: &Player,
    goalguard: &Player,
    tier: KickFoulScoringTier,
    offense_team_id: Uuid,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
    duel_context: &DuelContext,
    rng: &mut R,
) -> (ScoringDecision, AttributedDuelOutcome) {
    let kicker_table = tables
        .get(&kicker.id())
        .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);
    let gg_table = tables
        .get(&goalguard.id())
        .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE);

    let profile = match tier {
        KickFoulScoringTier::FirstZone => finishing_attempt_profile(),
        KickFoulScoringTier::Standard => field_goal_profile(),
    };

    let finisher_rating = profile.rate(kicker_table);
    let opportunity = evaluate_kick_foul_scoring_opportunity(tier, finisher_rating);

    let finish_context = duel_context.for_duel_kind(duel_kind_for_opportunity(opportunity));

    let req = ScoringAttemptRequest::new(
        kicker,
        goalguard,
        attribute_keys,
        offense_team_id,
        kicker.id(),
        None,
        opportunity,
        0,
        0.0,
        &finish_context,
    )
    .with_tables(Some(kicker_table), Some(gg_table));

    resolve_scoring_attempt(req, rng)
}