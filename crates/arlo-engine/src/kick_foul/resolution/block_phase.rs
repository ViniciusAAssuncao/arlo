use crate::attributes::PlayerAttributeTable;
use crate::kick_foul::resolution::participants::KickFoulParticipants;
use crate::resolution::group_rating::RatingParticipants;
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

    let req = DuelResolutionRequest::from_participants(
        DuelKind::KickBlockAttempt,
        participants.kicker,
        RatingParticipants::new(&participants.protectors).with_attribute_tables(tables),
        anchor_defender,
        RatingParticipants::new(&participants.rushers).with_attribute_tables(tables),
        attribute_keys,
        &c_context,
    );

    let raw_duel = resolve_duel(req, rng);

    AttributedDuelOutcome::new(raw_duel, attacker_ids, defender_ids)
}
