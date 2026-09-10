use crate::match_decision::event_translation::{
    create_envelope, translate_call_to_action_started, translate_duel_resolved,
};
use crate::resolution::context::DuelContext;
use crate::resolution::duel_kind::DuelKind;
use crate::resolution::group_rating::RatingParticipants;
use crate::resolution::resolver::{resolve_duel, DuelResolutionRequest};
use crate::resolution::AttributedDuelOutcome;
use crate::rng::RngStream;
use crate::world_state::cta_pass::participants::PhaseParticipants;
use crate::world_state::match_state::MatchState;
use arlo_domain::Position;
use arlo_events::EventSink;
use std::collections::HashMap;
use uuid::Uuid;

pub fn resolve_pass_protection_duel(
    state: &mut MatchState,
    participants: &PhaseParticipants<'_>,
    is_home_offense: bool,
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    down_number: u32,
    scrimmage_x_mirim: f64,
    sink: &mut impl EventSink,
) -> AttributedDuelOutcome {
    let target_advance_mirim = state
        .possession()
        .series_state()
        .remaining_mirins_to_target();

    let cta_event = translate_call_to_action_started(
        offense_team_id,
        defense_team_id,
        participants.passer.id(),
        participants.artrine.id(),
        down_number,
        scrimmage_x_mirim,
        target_advance_mirim,
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, cta_event));

    let context = if is_home_offense {
        DuelContext::attacker_home()
    } else {
        DuelContext::defender_home()
    };

    let mut duel_rng = state
        .rng_provider()
        .indexed_rng_for(RngStream::DuelResolution, seq);

    let fatigue_lookup = state.fatigue_lookup();
    let fatigue_fn = |id: &Uuid| fatigue_lookup.get(id);

    let pass_blocker_players: Vec<_> = participants.pass_blockers.iter().map(|(p, _)| *p).collect();
    let pass_blocker_map: HashMap<Uuid, Position> = participants
        .pass_blockers
        .iter()
        .map(|(p, pos)| (p.id(), *pos))
        .collect();

    let pass_rusher_players: Vec<_> = participants.pass_rushers.iter().map(|(p, _)| *p).collect();
    let pass_rusher_map: HashMap<Uuid, Position> = participants
        .pass_rushers
        .iter()
        .map(|(p, pos)| (p.id(), *pos))
        .collect();

    let req = DuelResolutionRequest::from_participants(
        DuelKind::PassProtection,
        participants.passer,
        RatingParticipants::from_slice_with_index(&pass_blocker_players, &pass_blocker_map)
            .with_fatigue(&fatigue_fn),
        participants.pass_rusher,
        RatingParticipants::from_slice_with_index(&pass_rusher_players, &pass_rusher_map)
            .with_fatigue(&fatigue_fn),
        state.attribute_keys(),
        &context,
    );

    let raw_pass_duel = resolve_duel(req, &mut duel_rng);

    let pass_duel_event = translate_duel_resolved(
        &raw_pass_duel,
        participants.attacker_ids.clone(),
        participants.defender_ids.clone(),
    );
    let seq = state.next_sequence();
    let clock_inst = state.clock().to_instant();
    sink.record(create_envelope(seq, clock_inst, pass_duel_event));

    AttributedDuelOutcome::new(
        raw_pass_duel,
        participants.attacker_ids.clone(),
        participants.defender_ids.clone(),
    )
}