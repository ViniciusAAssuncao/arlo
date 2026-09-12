use crate::artrine::resolve_primary_lead_defender_from_tables;
use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::physical::FatigueState;
use crate::resolution::duel_profiles::get_duel_profiles;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating, calculate_player_duel_rating_from_table, calculate_side_rating,
    RatingParticipants,
};
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{AttributedDuelOutcome, DuelContext, DuelKind};
use crate::spatial::DynamicSpatialMap;
use arlo_domain::pitch::Pitch;
use arlo_domain::{AttributeKey, KickFoulDecisionKind, Player, Position as DomainPosition};
use arlo_math::units::{Length, Position as VectorPosition, Velocity, MIRIM_TO_METERS};
use rand::Rng;
use serde::{Deserialize, Serialize};
use smallvec::smallvec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KickFoulRestartResult {
    pub caught: bool,
    pub reception_point: VectorPosition,
    pub receiver_id: Option<Uuid>,
    pub turnover: Option<Uuid>,
    pub duels: Vec<AttributedDuelOutcome>,
}

impl KickFoulRestartResult {
    pub fn new(
        caught: bool,
        reception_point: VectorPosition,
        receiver_id: Option<Uuid>,
        turnover: Option<Uuid>,
        duels: Vec<AttributedDuelOutcome>,
    ) -> Self {
        Self {
            caught,
            reception_point,
            receiver_id,
            turnover,
            duels,
        }
    }
}

pub fn resolve_kick_foul_restart<R: Rng + ?Sized>(
    kicker: &Player,
    kicker_pos: VectorPosition,
    target_candidates: &[&Player],
    defense_players: &[&Player],
    decision: KickFoulDecisionKind,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
    spatial_map: &DynamicSpatialMap,
    _pitch: &Pitch,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    duel_context: &DuelContext,
    rng: &mut R,
) -> KickFoulRestartResult {
    let duel_kind = match decision {
        KickFoulDecisionKind::Cross => DuelKind::CrossDistribution,
        KickFoulDecisionKind::LongLaunch => DuelKind::LongDistribution,
        _ => DuelKind::ShortDistribution,
    };

    let (off_prof, def_prof) = get_duel_profiles(duel_kind);

    let att_rating = calculate_anchored_side_rating(
        kicker,
        DomainPosition::CenterOffense,
        RatingParticipants::new(target_candidates).with_attribute_tables(tables),
        attribute_keys,
        off_prof,
    );
    let def_rating = calculate_side_rating(
        RatingParticipants::new(defense_players).with_attribute_tables(tables),
        attribute_keys,
        def_prof,
    );

    let contest_radius = Length::new(2.0 * MIRIM_TO_METERS);
    let dummy_instructions = HashMap::new();
    let lead_defender = resolve_primary_lead_defender_from_tables(
        kicker.id(),
        &HashMap::new(),
        kicker_pos,
        Velocity::zero(),
        defense_players,
        spatial_map,
        &dummy_instructions,
        tables,
        &|_| FatigueState::default(),
        contest_radius,
        None,
        rng,
    );

    let dist_context = duel_context.for_duel_kind(duel_kind);
    let att_table = tables.get(&kicker.id());
    let lead_def_table = tables.get(&lead_defender.id());
    let req = DuelResolutionRequest::with_states(
        duel_kind,
        att_rating,
        def_rating,
        kicker,
        lead_defender,
        FatigueState::default(),
        FatigueState::default(),
        attribute_keys,
        &dist_context,
    )
    .with_tables(att_table, lead_def_table);

    let raw_throw_duel = resolve_duel(req, rng);
    let throw_attributed = AttributedDuelOutcome::new(
        raw_throw_duel,
        smallvec![kicker.id()],
        smallvec![lead_defender.id()],
    );
    let mut duels = vec![throw_attributed];

    if !raw_throw_duel.attacker_won() {
        return KickFoulRestartResult::new(
            false,
            kicker_pos,
            None,
            None,
            duels,
        );
    }

    let receiver = if !target_candidates.is_empty() {
        let idx = rng.gen_range(0..target_candidates.len());
        target_candidates[idx]
    } else {
        kicker
    };
    let receiver_id = receiver.id();
    let rec_pos = spatial_map
        .get_position(&receiver_id)
        .unwrap_or(kicker_pos);

    let rec_duel_kind = match decision {
        KickFoulDecisionKind::LongLaunch | KickFoulDecisionKind::Cross => DuelKind::AerialDuel,
        _ => DuelKind::RouteContest,
    };

    let (rec_off, rec_def) = get_duel_profiles(rec_duel_kind);
    let rec_att_rating = calculate_player_duel_rating_from_table(
        receiver,
        DomainPosition::CenterOffense,
        tables
            .get(&receiver_id)
            .unwrap_or(&DEFAULT_PLAYER_ATTRIBUTE_TABLE),
        rec_off,
        &FatigueState::default(),
    );
    let rec_def_rating = calculate_side_rating(
        RatingParticipants::new(defense_players).with_attribute_tables(tables),
        attribute_keys,
        rec_def,
    );

    let rec_context = duel_context.for_duel_kind(rec_duel_kind);
    let rec_table = tables.get(&receiver_id);
    let lead_def_table2 = tables.get(&lead_defender.id());
    let rec_req = DuelResolutionRequest::with_states(
        rec_duel_kind,
        rec_att_rating,
        rec_def_rating,
        receiver,
        lead_defender,
        FatigueState::default(),
        FatigueState::default(),
        attribute_keys,
        &rec_context,
    )
    .with_tables(rec_table, lead_def_table2);

    let raw_rec_duel = resolve_duel(rec_req, rng);
    let rec_attributed = AttributedDuelOutcome::new(
        raw_rec_duel,
        smallvec![receiver_id],
        smallvec![lead_defender.id()],
    );
    duels.push(rec_attributed);

    let caught = raw_rec_duel.attacker_won();
    let turnover = if !caught && raw_rec_duel.net_advantage() <= -2.5 {
        lead_defender.team_id()
    } else {
        None
    };

    KickFoulRestartResult::new(
        caught,
        rec_pos,
        Some(receiver_id),
        turnover,
        duels,
    )
}