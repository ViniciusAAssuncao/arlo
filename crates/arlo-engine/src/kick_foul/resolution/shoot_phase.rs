use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::caching::get_cached_duel_profiles;
use crate::kick_foul::resolution::tier_opportunity::evaluate_kick_foul_scoring_opportunity;
use crate::match_decision::scoring::{
    duel_kind_for_opportunity, resolve_scoring_attempt, ScoringAttemptRequest, ScoringDecision,
};
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
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

    let (profile, _) = match tier {
        KickFoulScoringTier::FirstZone => get_cached_duel_profiles(DuelKind::FinishingAttempt),
        KickFoulScoringTier::Standard => get_cached_duel_profiles(DuelKind::FieldGoalAttempt),
    };

    let finisher_rating = profile.rate(kicker_table);
    let opportunity = evaluate_kick_foul_scoring_opportunity(tier, finisher_rating);

    let finish_context = duel_context.for_duel_kind(duel_kind_for_opportunity(opportunity));

    let normalized_proximity = match tier {
        KickFoulScoringTier::FirstZone => 0.95,
        KickFoulScoringTier::Standard => 0.85,
    };

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
        normalized_proximity,
        &finish_context,
    )
    .with_tables(Some(kicker_table), Some(gg_table));

    resolve_scoring_attempt(req, rng)
}
