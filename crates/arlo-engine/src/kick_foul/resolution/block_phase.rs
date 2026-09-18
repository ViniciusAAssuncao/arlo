use crate::attributes::PlayerAttributeTable;
use crate::caching::get_cached_duel_profiles;
use crate::kick_foul::resolution::participants::KickFoulParticipants;
use crate::resolution::group_rating::{calculate_side_rating, RatingParticipants};
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use arlo_domain::AttributeKey;
use rand::Rng;
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_kick_block_duel<R: Rng + ?Sized>(
    participants: &KickFoulParticipants<'_>,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    duel_context: &DuelContext,
    rng: &mut R,
) -> AttributedDuelOutcome {
    let anchor_defender = participants
        .rushers
        .first()
        .copied()
        .unwrap_or(participants.goalguard);

    let mut attacker_ids: SmallVec<[Uuid; 4]> = smallvec![participants.kicker.id()];
    for p in &participants.protectors {
        if !attacker_ids.contains(&p.id()) {
            attacker_ids.push(p.id());
        }
    }

    let mut defender_ids: SmallVec<[Uuid; 4]> = smallvec![anchor_defender.id()];
    for p in &participants.rushers {
        if !defender_ids.contains(&p.id()) {
            defender_ids.push(p.id());
        }
    }

    let c_context = duel_context.for_duel_kind(DuelKind::KickBlockAttempt);
    let (att_prof, def_prof) = get_cached_duel_profiles(DuelKind::KickBlockAttempt);

    let att_rating = calculate_side_rating(
        RatingParticipants::new(&participants.protectors).with_attribute_tables(tables),
        attribute_keys,
        att_prof,
    );
    let def_rating = calculate_side_rating(
        RatingParticipants::new(&participants.rushers).with_attribute_tables(tables),
        attribute_keys,
        def_prof,
    );

    let req = DuelResolutionRequest::with_states(
        DuelKind::KickBlockAttempt,
        att_rating,
        def_rating,
        participants.kicker,
        anchor_defender,
        crate::physical::PhysicalState::initial(),
        crate::physical::PhysicalState::initial(),
        attribute_keys,
        &c_context,
    )
    .with_tables(
        tables.get(&participants.kicker.id()),
        tables.get(&anchor_defender.id()),
    );

    let raw_duel = resolve_duel(req, rng);

    AttributedDuelOutcome::new(raw_duel, attacker_ids, defender_ids)
}
