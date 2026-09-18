use crate::attributes::{PlayerAttributeTable, DEFAULT_PLAYER_ATTRIBUTE_TABLE};
use crate::caching::get_cached_duel_profiles;
use crate::physical::FatigueState;
use crate::resolution::group_rating::{
    calculate_anchored_side_rating, calculate_player_duel_rating_from_table, calculate_side_rating,
    RatingParticipants,
};
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::{
    sample_action_progression, ActionProgressionKind, AttributedDuelOutcome, DuelContext, DuelKind,
};
use crate::team_identity::resolve_lead_defender_with_marking;
use arlo_domain::{AttributeKey, KickFoulDecisionKind, Player, Position as DomainPosition};
use rand::Rng;
use serde::{Deserialize, Serialize};
use smallvec::smallvec;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KickFoulRestartResult {
    pub caught: bool,
    pub reception_x_mirim: f64,
    pub reception_y_mirim: f64,
    pub receiver_id: Option<Uuid>,
    pub turnover: Option<Uuid>,
    pub duels: Vec<AttributedDuelOutcome>,
}

impl KickFoulRestartResult {
    pub fn new(
        caught: bool,
        reception_x_mirim: f64,
        reception_y_mirim: f64,
        receiver_id: Option<Uuid>,
        turnover: Option<Uuid>,
        duels: Vec<AttributedDuelOutcome>,
    ) -> Self {
        Self {
            caught,
            reception_x_mirim,
            reception_y_mirim,
            receiver_id,
            turnover,
            duels,
        }
    }
}

pub fn resolve_kick_foul_restart<R: Rng + ?Sized>(
    kicker: &Player,
    kicker_x_mirim: f64,
    kicker_y_mirim: f64,
    target_candidates: &[&Player],
    defense_players: &[&Player],
    decision: KickFoulDecisionKind,
    tables: &HashMap<Uuid, PlayerAttributeTable>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    duel_context: &DuelContext,
    rng: &mut R,
) -> KickFoulRestartResult {
    let duel_kind = match decision {
        KickFoulDecisionKind::Cross => DuelKind::CrossDistribution,
        KickFoulDecisionKind::LongLaunch => DuelKind::LongDistribution,
        _ => DuelKind::ShortDistribution,
    };

    let (off_prof, def_prof) = get_cached_duel_profiles(duel_kind);

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

    let dummy_instructions = HashMap::new();
    let lead_defender = resolve_lead_defender_with_marking(
        kicker.id(),
        &HashMap::new(),
        defense_players,
        &dummy_instructions,
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
            kicker_x_mirim,
            kicker_y_mirim,
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

    let rec_duel_kind = match decision {
        KickFoulDecisionKind::LongLaunch | KickFoulDecisionKind::Cross => DuelKind::AerialDuel,
        _ => DuelKind::RouteContest,
    };

    let (rec_off, rec_def) = get_cached_duel_profiles(rec_duel_kind);
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

    let prog_kind = match decision {
        KickFoulDecisionKind::Cross => ActionProgressionKind::Cross,
        KickFoulDecisionKind::LongLaunch => ActionProgressionKind::LongLaunch,
        _ => ActionProgressionKind::ShortPass,
    };
    let advance_mirim = if caught {
        sample_action_progression(prog_kind, raw_rec_duel.net_advantage(), 1.0, rng)
    } else {
        0.0
    };

    let is_home = duel_context.attacker_is_home();
    let reception_x_mirim = if is_home {
        kicker_x_mirim + advance_mirim
    } else {
        kicker_x_mirim - advance_mirim
    };

    KickFoulRestartResult::new(
        caught,
        reception_x_mirim,
        kicker_y_mirim,
        Some(receiver_id),
        turnover,
        duels,
    )
}
